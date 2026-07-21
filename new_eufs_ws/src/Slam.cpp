#include <rclcpp/rclcpp.hpp>
#include <Eigen/Dense>
#include "eufs_msgs/msg/cone_array.hpp"
#include "eufs_msgs/msg/cone_array_with_covariance.hpp"
#include "nav_msgs/msg/odometry.hpp"
#include <vector>
#include <cmath>
#include <memory>
#include <algorithm>
#include <rclcpp/qos.hpp>

using std::placeholders::_1;

inline double wrapToPi(double a) {
    return std::atan2(std::sin(a), std::cos(a));
}

struct ConeDetection {
    double range;
    double bearing;
    int color; 
};

struct LandmarkInfo {
    int color;
    int hits;
};

class EKFSLAM : public rclcpp::Node {
public:
    EKFSLAM() : Node("ekf_slam_node") {
        x_ = Eigen::VectorXd::Zero(3);
        P_ = Eigen::MatrixXd::Identity(3, 3) * 0.01;

        R_.setZero();
        R_(0, 0) = std::pow(0.5, 2); 
        R_(1, 1) = std::pow(0.1, 2);

        cones_pub_ = create_publisher<eufs_msgs::msg::ConeArray>("/planning/cones", 10);
        slam_odom_pub_ = create_publisher<nav_msgs::msg::Odometry>("/slam/odom", 10);    
        
        auto qos = rclcpp::QoS(rclcpp::KeepLast(10)).best_effort();
        
        cones_sub_ = create_subscription<eufs_msgs::msg::ConeArrayWithCovariance>(
            "/ground_truth/cones", qos, std::bind(&EKFSLAM::conesCallback, this, _1));
        
        odom_sub_ = create_subscription<nav_msgs::msg::Odometry>(
            "/odometry/filtered", 10, std::bind(&EKFSLAM::odomCallback, this, _1));

        timer_ = create_wall_timer(std::chrono::milliseconds(50), std::bind(&EKFSLAM::runSLAM, this));

        RCLCPP_INFO(this->get_logger(), "EKF-SLAM initialized. ROBUST LOCALIZATION ACTIVE.");
    }

private:
    Eigen::VectorXd x_;
    Eigen::MatrixXd P_;
    Eigen::Matrix2d R_;
    std::vector<ConeDetection> z_buffer_;
    std::vector<LandmarkInfo> lm_info_;
    
    bool lap_closed_ = false;
    double total_dist_ = 0.0; 
    int lap_count_ = 0;             
    double last_lap_dist_ = 0.0;    
    
    double vx_ = 0.0, yaw_rate_ = 0.0, dt_ = 0.0;
    rclcpp::Time last_odom_time_{0, 0, RCL_ROS_TIME};

    rclcpp::Subscription<eufs_msgs::msg::ConeArrayWithCovariance>::SharedPtr cones_sub_;
    rclcpp::Subscription<nav_msgs::msg::Odometry>::SharedPtr odom_sub_;
    rclcpp::Publisher<eufs_msgs::msg::ConeArray>::SharedPtr cones_pub_;
    rclcpp::Publisher<nav_msgs::msg::Odometry>::SharedPtr slam_odom_pub_;
    rclcpp::TimerBase::SharedPtr timer_;

    void odomCallback(const nav_msgs::msg::Odometry::SharedPtr msg) {
        vx_ = msg->twist.twist.linear.x;
        yaw_rate_ = msg->twist.twist.angular.z;
        rclcpp::Time msg_time = msg->header.stamp;
        if (last_odom_time_.nanoseconds() != 0) {
            dt_ = (msg_time - last_odom_time_).seconds();
        }
        last_odom_time_ = msg_time;
    }

    void conesCallback(const eufs_msgs::msg::ConeArrayWithCovariance::SharedPtr msg) {
        z_buffer_.clear();
        for (const auto &cone : msg->blue_cones) addConeToBuffer(cone.point, 0); 
        for (const auto &cone : msg->yellow_cones) addConeToBuffer(cone.point, 1);
        for (const auto &cone : msg->big_orange_cones) addConeToBuffer(cone.point, 2); 
    }

    void addConeToBuffer(const geometry_msgs::msg::Point &cone, int color) {
        ConeDetection z;
        z.range = std::hypot(cone.x, cone.y);
        z.bearing = std::atan2(cone.y, cone.x);
        z.color = color;
        if (z.range > 0.1 && z.range < 30.0) z_buffer_.push_back(z);
    }

    void predict() {
        double psi = x_(2);
        x_(0) += vx_ * std::cos(psi) * dt_;
        x_(1) += vx_ * std::sin(psi) * dt_;
        x_(2) = wrapToPi(x_(2) + yaw_rate_ * dt_);

        Eigen::MatrixXd G = Eigen::MatrixXd::Identity(x_.size(), x_.size());
        G(0, 2) = -vx_ * std::sin(psi) * dt_;
        G(1, 2) = vx_ * std::cos(psi) * dt_;

        P_ = G * P_ * G.transpose();
        Eigen::Matrix3d Q = Eigen::Matrix3d::Identity() * 0.01;
        P_.block<3, 3>(0, 0) += Q;
    }

    void measurementModel(int j, Eigen::Vector2d &zhat, Eigen::MatrixXd &H) {
        int idx = 3 + 2 * j;
        double dx = x_(idx) - x_(0), dy = x_(idx + 1) - x_(1);
        double r2 = dx * dx + dy * dy, r = std::sqrt(r2) + 1e-6;
        zhat << r, wrapToPi(std::atan2(dy, dx) - x_(2));
        H = Eigen::MatrixXd::Zero(2, x_.size());
        H(0, 0) = -dx / r;  H(0, 1) = -dy / r;  H(0, 2) = 0;
        H(1, 0) = dy / r2;  H(1, 1) = -dx / r2; H(1, 2) = -1.0;
        H(0, idx) = dx / r; H(0, idx + 1) = dy / r;
        H(1, idx) = -dy / r2; H(1, idx + 1) = dx / r2;
    }

    void runSLAM() {
        if (dt_ <= 0.0 || dt_ > 0.2) return; 
        predict();
        
        total_dist_ += std::abs(vx_) * dt_;
        dt_ = 0.0;

        RCLCPP_INFO_THROTTLE(
            this->get_logger(), *this->get_clock(), 2000, 
            "🏎️  ACTIVE LAP: %d  |  Total Distance: %.1f meters", lap_count_ + 1, total_dist_
        );

        if ((total_dist_ - last_lap_dist_) > 30.0 && std::hypot(x_(0), x_(1)) < 12.0) {
            lap_count_++;
            last_lap_dist_ = total_dist_; 

            RCLCPP_INFO(this->get_logger(), "=======================================");
            RCLCPP_INFO(this->get_logger(), "🏁 LAP %d COMPLETED! (Lap Distance: %.1f m)", lap_count_, total_dist_ - last_lap_dist_);
            
            if (!lap_closed_) {
                lap_closed_ = true;
                RCLCPP_INFO(this->get_logger(), "🔒 MAP LOCKED. ROBUST LOCALIZATION ENGAGED.");
            }
            RCLCPP_INFO(this->get_logger(), "=======================================");
        }

        // --- HARDENED FIX 1: DYNAMIC TRUST ---
        // If lap is closed, trust the cameras 5x less. Prevents violent snapping.
        Eigen::Matrix2d R_dynamic = lap_closed_ ? (R_ * 5.0) : R_;

        for (const auto &z : z_buffer_) {
            int best_j = -1; 
            double best_md = 1e9;
            double angle = wrapToPi(x_(2) + z.bearing);
            double det_x = x_(0) + z.range * std::cos(angle);
            double det_y = x_(1) + z.range * std::sin(angle);

            for (size_t j = 0; j < lm_info_.size(); ++j) {
                if (lm_info_[j].color != z.color) continue;
                double dx = x_(3 + 2 * j) - det_x;
                double dy = x_(3 + 2 * j + 1) - det_y;
                
                // Coarse Gate: If the cone is more than 1.5m away from the map point, physically impossible. Skip it.
                if (std::hypot(dx, dy) > 1.5) continue; 

                Eigen::Vector2d zhat; Eigen::MatrixXd H;
                measurementModel(j, zhat, H);
                Eigen::Vector2d v; v << z.range - zhat(0), wrapToPi(z.bearing - zhat(1));
                
                Eigen::Matrix2d S;
                if (lap_closed_) {
                    // Fast localized inverse
                    Eigen::Matrix3d P_rob = P_.block<3, 3>(0, 0);
                    Eigen::Matrix<double, 2, 3> H_rob = H.leftCols(3);
                    S = H_rob * P_rob * H_rob.transpose() + R_dynamic;
                } else {
                    S = H * P_ * H.transpose() + R_dynamic;
                }
                
                if (S.determinant() < 1e-6) continue;
                double md = v.transpose() * S.inverse() * v;
                
                // --- HARDENED FIX 2: STRICT GATING ---
                // Lower threshold in localization mode (e.g., 3.0 instead of 9.21) to reject bad matches
                double gate_limit = lap_closed_ ? 3.0 : 9.21;
                
                if (md < best_md && md < gate_limit) { best_md = md; best_j = (int)j; }
            }

            if (best_j >= 0) {
                lm_info_[best_j].hits++;
                
                if (lap_closed_) {
                    Eigen::Matrix3d P_robot = P_.block<3, 3>(0, 0);
                    double dx = x_(3 + 2 * best_j) - x_(0);
                    double dy = x_(3 + 2 * best_j + 1) - x_(1);
                    double r2 = dx * dx + dy * dy;
                    double r = std::sqrt(r2) + 1e-6;
                    
                    Eigen::Matrix<double, 2, 3> H_robot;
                    H_robot << -dx / r, -dy / r, 0,
                                dy / r2, -dx / r2, -1.0;
                    
                    Eigen::Matrix2d S = H_robot * P_robot * H_robot.transpose() + R_dynamic;
                    Eigen::Matrix<double, 3, 2> K_robot = P_robot * H_robot.transpose() * S.inverse();
                    
                    Eigen::Vector2d zhat; zhat << r, wrapToPi(std::atan2(dy, dx) - x_(2));
                    Eigen::Vector2d v; v << z.range - zhat(0), wrapToPi(z.bearing - zhat(1));
                    
                    Eigen::Vector3d state_update = K_robot * v;
                    
                    // --- HARDENED FIX 3: INNOVATION CLAMPING ---
                    // Physically prevent the car from "teleporting" to a bad map point.
                    state_update(0) = std::clamp(state_update(0), -0.1, 0.1); // Max 10cm jump per frame
                    state_update(1) = std::clamp(state_update(1), -0.1, 0.1); 
                    state_update(2) = std::clamp(state_update(2), -0.05, 0.05); // Max ~3 degrees rotation
                    
                    x_.head<3>() += state_update;
                    x_(2) = wrapToPi(x_(2));
                    P_.block<3, 3>(0, 0) = (Eigen::Matrix3d::Identity() - K_robot * H_robot) * P_robot;
                } else {
                    Eigen::Vector2d zhat; Eigen::MatrixXd H;
                    measurementModel(best_j, zhat, H);
                    Eigen::Vector2d v; v << z.range - zhat(0), wrapToPi(z.bearing - zhat(1));
                    Eigen::Matrix2d S = H * P_ * H.transpose() + R_dynamic;
                    Eigen::MatrixXd K = P_ * H.transpose() * S.inverse();
                    x_ += K * v;
                    P_ = (Eigen::MatrixXd::Identity(x_.size(), x_.size()) - K * H) * P_;
                }

            } else if (!lap_closed_) {
                bool duplicate = false;
                for(size_t i=0; i < lm_info_.size(); ++i) {
                    if(std::hypot(x_(3+2*i)-det_x, x_(3+2*i+1)-det_y) < 1.5) { duplicate = true; break; }
                }
                if(!duplicate && lm_info_.size() < 400) {
                    int old_size = x_.size();
                    x_.conservativeResize(old_size + 2); x_.tail<2>() << det_x, det_y;
                    Eigen::MatrixXd Gx(2, 3); Gx << 1, 0, -z.range*std::sin(angle), 0, 1, z.range*std::cos(angle);
                    Eigen::Matrix2d Gz; Gz << std::cos(angle), -z.range*std::sin(angle), std::sin(angle), z.range*std::cos(angle);
                    Eigen::MatrixXd P_new = Eigen::MatrixXd::Zero(old_size+2, old_size+2);
                    P_new.topLeftCorner(old_size, old_size) = P_;
                    Eigen::MatrixXd Pxr = P_.block(0, 0, old_size, 3) * Gx.transpose();
                    P_new.block(0, old_size, old_size, 2) = Pxr; P_new.block(old_size, 0, 2, old_size) = Pxr.transpose();
                    P_new.bottomRightCorner(2, 2) = Gx * P_.block<3, 3>(0, 0) * Gx.transpose() + Gz * R_ * Gz.transpose();
                    P_ = P_new;
                    LandmarkInfo new_lm; new_lm.color = z.color; new_lm.hits = 1;
                    lm_info_.push_back(new_lm);
                }
            }
        }
        z_buffer_.clear();
        P_ = 0.5 * (P_ + P_.transpose());
        P_ += Eigen::MatrixXd::Identity(P_.rows(), P_.cols()) * 1e-9;

        auto out_msg = eufs_msgs::msg::ConeArray();
        out_msg.header.stamp = this->now();
        out_msg.header.frame_id = "map";
        
        for (size_t i = 0; i < lm_info_.size(); ++i) {
            double cx = x_(3 + 2 * i);
            double cy = x_(3 + 2 * i + 1);
            
            if (std::hypot(cx - x_(0), cy - x_(1)) < 15.0) {
                geometry_msgs::msg::Point p; p.x = cx; p.y = cy; p.z = 0.0;
                if (lm_info_[i].color == 0) out_msg.blue_cones.push_back(p);
                else if (lm_info_[i].color == 1) out_msg.yellow_cones.push_back(p);
                else if (lm_info_[i].color == 2) out_msg.big_orange_cones.push_back(p);
            }
        }
        cones_pub_->publish(out_msg);

        nav_msgs::msg::Odometry slam_odom;
        slam_odom.header.stamp = this->now();
        slam_odom.header.frame_id = "map";
        slam_odom.pose.pose.position.x = x_(0); slam_odom.pose.pose.position.y = x_(1);
        slam_odom.pose.pose.orientation.z = std::sin(x_(2) * 0.5); 
        slam_odom.pose.pose.orientation.w = std::cos(x_(2) * 0.5);
        slam_odom.twist.twist.linear.x = vx_; slam_odom.twist.twist.angular.z = yaw_rate_;
        slam_odom_pub_->publish(slam_odom);
    }
};

int main(int argc, char **argv) {
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<EKFSLAM>());
    rclcpp::shutdown();
    return 0;
}
