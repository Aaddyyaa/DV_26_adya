#include <algorithm>
#include <chrono>
#include <cmath>
#include <functional>
#include <limits>
#include <memory>
#include <string>
#include <vector>

#include <rclcpp/rclcpp.hpp>
#include <nav_msgs/msg/odometry.hpp>
#include <nav_msgs/msg/path.hpp>
#include <std_msgs/msg/float64_multi_array.hpp>
#include <std_msgs/msg/bool.hpp>
#include <ackermann_msgs/msg/ackermann_drive_stamped.hpp>
#include <tf2/LinearMath/Quaternion.h>
#include <tf2/LinearMath/Matrix3x3.h>
#include <visualization_msgs/msg/marker.hpp>
#include <eufs_msgs/msg/can_state.hpp>

using std::placeholders::_1;

namespace
{
constexpr double CONTROL_DT = 0.05;
}

class HybridControllerNode : public rclcpp::Node {
public:
    HybridControllerNode() : Node("pure_pursuit_node") {
        this->declare_parameter("odom_topic", std::string("/custom_odom"));
        this->declare_parameter("startup_duration_sec", 1.5);
        this->declare_parameter("startup_accel_mps2", 2.0);
        this->declare_parameter("startup_speed_mps", 0.8);

        this->declare_parameter("L_base", 1.53);
        this->declare_parameter("L_min", 1.8);
        this->declare_parameter("k_pure", 0.15);

        this->declare_parameter("max_speed_limit", 1.5);
        this->declare_parameter("max_accel", 1.0);
        this->declare_parameter("max_decel", 4.0);
        this->declare_parameter("max_steering", 0.5);
        this->declare_parameter("min_speed_mps", 0.6);
        this->declare_parameter("max_steering_rate_rad_s", 1.5);
        this->declare_parameter("max_tracking_error_m", 0.75);
        this->declare_parameter("startup_duration_sec", 1.0);
        this->declare_parameter("startup_accel_mps2", 1.5);
        this->declare_parameter("startup_speed_mps", 0.8);
        this->declare_parameter("startup_max_steering_rad", 0.12);

        const std::string odom_topic =
            get_parameter("odom_topic").as_string();

        odom_sub_ = this->create_subscription<nav_msgs::msg::Odometry>(
            odom_topic, 10,
            std::bind(&HybridControllerNode::odomCallback, this, _1));

        path_sub_ = this->create_subscription<nav_msgs::msg::Path>(
            "/target_path", 10,
            std::bind(&HybridControllerNode::pathCallback, this, _1));

        speed_profile_sub_ =
            this->create_subscription<std_msgs::msg::Float64MultiArray>(
                "/target_speeds", 10,
                std::bind(
                    &HybridControllerNode::speedProfileCallback,
                    this,
                    _1));

        can_state_sub_ =
            this->create_subscription<eufs_msgs::msg::CanState>(
                "/ros_can/state", 10,
                [this](const eufs_msgs::msg::CanState::SharedPtr msg) {
                    const bool driving =
                        msg->as_state == eufs_msgs::msg::CanState::AS_DRIVING;

                    if (driving && !as_driving_) {
                        startup_time_ = this->now();
                    }

                    if (!driving) {
                        startup_time_ = rclcpp::Time(0, 0, RCL_ROS_TIME);
                    }

                    as_driving_ = driving;
                });

        mission_completed_sub_ =
            this->create_subscription<std_msgs::msg::Bool>(
                "/ros_can/mission_completed",
                rclcpp::QoS(1).reliable(),
                [this](const std_msgs::msg::Bool::SharedPtr msg) {
                    if (msg->data) {
                        mission_completed_ = true;
                        RCLCPP_INFO(
                            this->get_logger(),
                            "Lap completion received; stopping controller.");
                    }
                });

        drive_pub_ =
            this->create_publisher<ackermann_msgs::msg::AckermannDriveStamped>(
                "/cmd", 10);

        vis_pub_ =
            this->create_publisher<visualization_msgs::msg::Marker>(
                "lookahead_marker", 10);

        timer_ = this->create_wall_timer(
            std::chrono::milliseconds(50),
            std::bind(&HybridControllerNode::controlLoop, this));

        RCLCPP_INFO(
            this->get_logger(),
            "Controller using %s in the same map frame as /target_path.",
            odom_topic.c_str());
    }

private:
    double x_ = 0.0;
    double y_ = 0.0;
    double psi_ = 0.0;
    double vx_ = 0.0;

    nav_msgs::msg::Path path_;
    std::vector<double> speed_profile_;

    bool has_odom_ = false;
    bool has_path_ = false;
    bool mission_completed_ = false;
    bool startup_active_ = false;
    bool as_driving_ = false;

    double last_steering_ = 0.0;
    size_t last_closest_idx_ = 0;
    rclcpp::Time startup_time_{0, 0, RCL_ROS_TIME};

    rclcpp::Subscription<nav_msgs::msg::Odometry>::SharedPtr odom_sub_;
    rclcpp::Subscription<nav_msgs::msg::Path>::SharedPtr path_sub_;
    rclcpp::Subscription<std_msgs::msg::Float64MultiArray>::SharedPtr
        speed_profile_sub_;
    rclcpp::Subscription<std_msgs::msg::Bool>::SharedPtr
        mission_completed_sub_;
    rclcpp::Subscription<eufs_msgs::msg::CanState>::SharedPtr
        can_state_sub_;

    rclcpp::Publisher<ackermann_msgs::msg::AckermannDriveStamped>::SharedPtr
        drive_pub_;
    rclcpp::Publisher<visualization_msgs::msg::Marker>::SharedPtr vis_pub_;
    rclcpp::TimerBase::SharedPtr timer_;

    void odomCallback(
        const nav_msgs::msg::Odometry::SharedPtr msg)
    {
        x_ = msg->pose.pose.position.x;
        y_ = msg->pose.pose.position.y;

        tf2::Quaternion q(
            msg->pose.pose.orientation.x,
            msg->pose.pose.orientation.y,
            msg->pose.pose.orientation.z,
            msg->pose.pose.orientation.w);

        double roll = 0.0;
        double pitch = 0.0;
        double yaw = 0.0;
        tf2::Matrix3x3(q).getRPY(roll, pitch, yaw);

        psi_ = yaw;
        vx_ = msg->twist.twist.linear.x;
        has_odom_ = true;
    }

    void pathCallback(
        const nav_msgs::msg::Path::SharedPtr msg)
    {
        const bool had_path = has_path_;
        path_ = *msg;
        has_path_ = !path_.poses.empty();

        if (!has_path_) {
            last_closest_idx_ = 0;
            last_steering_ = 0.0;
            startup_active_ = false;
            return;
        }

        if (!had_path) {
            startup_active_ = true;
            startup_time_ = this->now();
            last_closest_idx_ = 0;
        }

        // Every fresh planner path begins at the current SLAM pose, so index
        // zero is the only safe re-anchoring point. Never jump into the middle
        // of a closed-loop path.
        last_closest_idx_ = 0;
    }

    void speedProfileCallback(
        const std_msgs::msg::Float64MultiArray::SharedPtr msg)
    {
        speed_profile_ = msg->data;
    }

    void publishStopCommand()
    {
        ackermann_msgs::msg::AckermannDriveStamped msg;
        msg.header.stamp = this->now();
        msg.drive.speed = 0.0;
        msg.drive.acceleration =
            std::abs(vx_) > 0.15
                ? -get_parameter("max_decel").as_double()
                : 0.0;
        msg.drive.steering_angle =
            std::abs(vx_) > 0.15 ? last_steering_ : 0.0;
        drive_pub_->publish(msg);
    }

    void controlLoop()
    {
        if (mission_completed_) {
            publishStopCommand();
            return;
        }

        if (!has_odom_) {
            return;
        }

        if (!has_path_) {
            bool bootstrap_active = false;

            if (as_driving_) {
                if (startup_time_.nanoseconds() == 0) {
                    startup_time_ = this->now();
                }

                const double elapsed =
                    (this->now() - startup_time_).seconds();

                bootstrap_active =
                    elapsed < get_parameter("startup_duration_sec").as_double();
            }

            if (bootstrap_active) {
                ackermann_msgs::msg::AckermannDriveStamped bootstrap;
                bootstrap.header.stamp = this->now();
                bootstrap.drive.speed = std::max(
                    0.2,
                    std::min(
                        get_parameter("max_speed_limit").as_double(),
                        get_parameter("startup_speed_mps").as_double()));
                bootstrap.drive.acceleration =
                    get_parameter("startup_accel_mps2").as_double();
                bootstrap.drive.steering_angle = 0.0;
                bootstrap.drive.jerk = 0.0;
                drive_pub_->publish(bootstrap);

                RCLCPP_INFO_THROTTLE(
                    this->get_logger(),
                    *this->get_clock(),
                    1000,
                    "AS_DRIVING bootstrap: speed=%.2f accel=%.2f waiting for first planner path.",
                    bootstrap.drive.speed,
                    bootstrap.drive.acceleration);
            } else {
                publishStopCommand();
            }

            return;
        }

        const size_t N = path_.poses.size();
        if (N < 3) {
            publishStopCommand();
            return;
        }

        const size_t search_begin =
            last_closest_idx_ > 2 ? last_closest_idx_ - 2 : 0;
        const size_t search_end =
            std::min(N - 1, last_closest_idx_ + 12);

        double min_d = std::numeric_limits<double>::infinity();
        size_t best_idx = last_closest_idx_;

        for (size_t i = search_begin; i <= search_end; ++i) {
            const double dx =
                path_.poses[i].pose.position.x - x_;
            const double dy =
                path_.poses[i].pose.position.y - y_;
            const double d = std::hypot(dx, dy);

            if (d < min_d) {
                min_d = d;
                best_idx = i;
            }
        }

        last_closest_idx_ = best_idx;

        const double max_tracking_error =
            get_parameter("max_tracking_error_m").as_double();

        if (min_d > max_tracking_error) {
            RCLCPP_WARN_THROTTLE(
                this->get_logger(),
                *this->get_clock(),
                2000,
                "Tracking error %.2f m; rejecting path command.",
                min_d);
            publishStopCommand();
            return;
        }

        const double speed = std::max(0.0, std::abs(vx_));
        const double L_base =
            get_parameter("L_base").as_double();
        const double Ld = std::clamp(
            get_parameter("L_min").as_double()
            + get_parameter("k_pure").as_double() * speed,
            1.5,
            3.0);

        size_t idx_ld = last_closest_idx_;
        double accumulated = 0.0;

        for (size_t i = last_closest_idx_ + 1; i < N; ++i) {
            accumulated += std::hypot(
                path_.poses[i].pose.position.x -
                    path_.poses[i - 1].pose.position.x,
                path_.poses[i].pose.position.y -
                    path_.poses[i - 1].pose.position.y);

            idx_ld = i;
            if (accumulated >= Ld) {
                break;
            }
        }

        const double tx = path_.poses[idx_ld].pose.position.x;
        const double ty = path_.poses[idx_ld].pose.position.y;
        const double dx = tx - x_;
        const double dy = ty - y_;
        const double ly =
            -dx * std::sin(psi_) + dy * std::cos(psi_);

        const double Ld_sq = std::max(0.05, dx * dx + dy * dy);
        const double delta =
            std::atan2(2.0 * L_base * ly, Ld_sq);

        visualization_msgs::msg::Marker marker;
        marker.header.frame_id = "map";
        marker.header.stamp = this->now();
        marker.ns = "lookahead";
        marker.id = 0;
        marker.type = visualization_msgs::msg::Marker::SPHERE;
        marker.action = visualization_msgs::msg::Marker::ADD;
        marker.pose.position.x = tx;
        marker.pose.position.y = ty;
        marker.pose.position.z = 0.5;
        marker.scale.x = 0.4;
        marker.scale.y = 0.4;
        marker.scale.z = 0.4;
        marker.color.a = 1.0;
        marker.color.g = 1.0;
        vis_pub_->publish(marker);

        bool geometry_valid = true;
        const size_t geometry_end =
            std::min(N - 1, last_closest_idx_ + 12);

        for (size_t i = last_closest_idx_ + 1;
             i <= geometry_end;
             ++i) {
            const double segment = std::hypot(
                path_.poses[i].pose.position.x -
                    path_.poses[i - 1].pose.position.x,
                path_.poses[i].pose.position.y -
                    path_.poses[i - 1].pose.position.y);

            if (!std::isfinite(segment) || segment > 1.25) {
                geometry_valid = false;
                break;
            }

            if (i >= last_closest_idx_ + 2) {
                const double h1 = std::atan2(
                    path_.poses[i - 1].pose.position.y -
                        path_.poses[i - 2].pose.position.y,
                    path_.poses[i - 1].pose.position.x -
                        path_.poses[i - 2].pose.position.x);
                const double h2 = std::atan2(
                    path_.poses[i].pose.position.y -
                        path_.poses[i - 1].pose.position.y,
                    path_.poses[i].pose.position.x -
                        path_.poses[i - 1].pose.position.x);

                if (std::abs(std::atan2(
                        std::sin(h2 - h1),
                        std::cos(h2 - h1)))
                    > (100.0 * 3.141592653589793 / 180.0)) {
                    geometry_valid = false;
                    break;
                }
            }
        }

        if (!geometry_valid) {
            RCLCPP_WARN_THROTTLE(
                this->get_logger(),
                *this->get_clock(),
                2000,
                "Invalid path geometry; holding vehicle stopped.");
            publishStopCommand();
            return;
        }

        bool startup = startup_active_ || as_driving_;
        if (startup) {
            const double elapsed =
                startup_time_.nanoseconds() == 0
                    ? 0.0
                    : (this->now() - startup_time_).seconds();

            if (!as_driving_ ||
                elapsed >= get_parameter("startup_duration_sec").as_double() ||
                std::abs(vx_) >= 0.40) {
                startup_active_ = false;
                startup = false;
            }
        }

        double raw_steering = std::clamp(
            delta,
            -get_parameter("max_steering").as_double(),
            get_parameter("max_steering").as_double());

        if (startup) {
            raw_steering = std::clamp(
                raw_steering,
                -get_parameter("startup_max_steering_rad").as_double(),
                get_parameter("startup_max_steering_rad").as_double());
        }

        const double max_step =
            get_parameter("max_steering_rate_rad_s").as_double()
            * CONTROL_DT;

        const double rate_limited =
            std::clamp(
                raw_steering,
                last_steering_ - max_step,
                last_steering_ + max_step);

        last_steering_ = rate_limited;

        ackermann_msgs::msg::AckermannDriveStamped drive_msg;
        drive_msg.header.stamp = this->now();
        drive_msg.drive.steering_angle = last_steering_;

        const double max_speed_limit =
            get_parameter("max_speed_limit").as_double();
        const double decel =
            get_parameter("max_decel").as_double();

        double target_velocity =
            startup
                ? std::min(
                    max_speed_limit,
                    get_parameter("startup_speed_mps").as_double())
                : max_speed_limit;
        double path_distance = 0.0;

        const size_t velocity_scan_limit =
            std::min(N - 1, last_closest_idx_ + 80);

        for (size_t i = last_closest_idx_;
             i <= velocity_scan_limit;
             ++i) {
            if (i > last_closest_idx_) {
                path_distance += std::hypot(
                    path_.poses[i].pose.position.x -
                        path_.poses[i - 1].pose.position.x,
                    path_.poses[i].pose.position.y -
                        path_.poses[i - 1].pose.position.y);
            }

            double node_speed = max_speed_limit;
            if (i < speed_profile_.size()) {
                node_speed = std::clamp(
                    speed_profile_[i],
                    0.0,
                    max_speed_limit);
            }

            const double required_current_speed =
                std::sqrt(std::max(
                    0.0,
                    node_speed * node_speed
                    + 2.0 * decel * path_distance));

            target_velocity =
                std::min(target_velocity, required_current_speed);
        }

        const double steering_abs = std::abs(raw_steering);
        if (steering_abs > 0.45) {
            target_velocity = std::min(target_velocity, 0.8);
        } else if (steering_abs > 0.35) {
            target_velocity = std::min(target_velocity, 1.0);
        }

        const double min_speed =
            std::max(0.0, get_parameter("min_speed_mps").as_double());

        target_velocity = std::clamp(
            target_velocity,
            std::min(min_speed, max_speed_limit),
            max_speed_limit);

        drive_msg.drive.speed = target_velocity;
        drive_msg.drive.jerk = 0.0;

        if (startup) {
            drive_msg.drive.acceleration =
                get_parameter("startup_accel_mps2").as_double();
        } else if (target_velocity < vx_) {
            drive_msg.drive.acceleration = -decel;
        } else {
            drive_msg.drive.acceleration =
                get_parameter("max_accel").as_double();
        }

        RCLCPP_INFO_THROTTLE(
            this->get_logger(),
            *this->get_clock(),
            2000,
            "CMD speed=%.2f steer=%.3f idx=%zu/%zu tracking=%.2f",
            drive_msg.drive.speed,
            drive_msg.drive.steering_angle,
            last_closest_idx_,
            N,
            min_d);

        drive_pub_->publish(drive_msg);
    }
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<HybridControllerNode>());
    rclcpp::shutdown();
    return 0;
}
