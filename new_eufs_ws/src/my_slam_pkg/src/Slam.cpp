#include <algorithm>
#include <array>
#include <chrono>
#include <cstddef>
#include <cstdint>
#include <cmath>
#include <limits>
#include <memory>
#include <mutex>
#include <random>
#include <string>
#include <vector>

#include <rclcpp/rclcpp.hpp>
#include <rclcpp/qos.hpp>
#include <nav_msgs/msg/odometry.hpp>

#include "eufs_msgs/msg/cone_array.hpp"
#include "eufs_msgs/msg/cone_array_with_covariance.hpp"

#include <tf2_ros/transform_broadcaster.h>
#include <geometry_msgs/msg/point.hpp>
#include <geometry_msgs/msg/transform_stamped.hpp>
#include <visualization_msgs/msg/marker.hpp>
#include <visualization_msgs/msg/marker_array.hpp>

#include <Eigen/Dense>

using std::placeholders::_1;

namespace
{
constexpr double PI = 3.14159265358979323846;
constexpr double DEFAULT_DT = 0.05;

double wrapToPi(double angle)
{
    return std::atan2(std::sin(angle), std::cos(angle));
}
}

struct ConeDetection
{
    double range{0.0};
    double bearing{0.0};
    int color{3};
    Eigen::Matrix2d covariance{Eigen::Matrix2d::Identity()};
};

struct Landmark
{
    Eigen::Vector2d mu{Eigen::Vector2d::Zero()};
    Eigen::Matrix2d sigma{Eigen::Matrix2d::Identity() * 0.01};
    int color{3};
    int hits{0};
};

struct Particle
{
    double weight{1.0};
    double x{0.0};
    double y{0.0};
    double yaw{0.0};
    Eigen::Matrix3d P{Eigen::Matrix3d::Identity() * 0.001};
    std::vector<Landmark> map;
};

class FastSLAM2 final : public rclcpp::Node
{
public:
    FastSLAM2()
        : Node("fast_slam_node"), gen_(rd_())
    {
        declare_parameter<int>("num_particles", 30);
        declare_parameter<double>("process_noise_xy", 0.0004);
        declare_parameter<double>("process_noise_yaw", 0.0001);
        declare_parameter<double>("landmark_match_distance", 2.0);
        declare_parameter<int>("min_landmark_hits", 1);
        declare_parameter<double>("fallback_dt", DEFAULT_DT);
        declare_parameter<std::string>(
            "cones_topic", "/camera_0/cones");
        declare_parameter<std::string>("odom_topic", "/odometry/filtered");

        num_particles_ = static_cast<int>(std::max<int64_t>(5, get_parameter("num_particles").as_int()));
        process_noise_xy_ =
            std::max(1e-8, get_parameter("process_noise_xy").as_double());
        process_noise_yaw_ =
            std::max(1e-8, get_parameter("process_noise_yaw").as_double());
        landmark_match_distance_ =
            std::max(0.2, get_parameter("landmark_match_distance").as_double());
        min_landmark_hits_ =
            static_cast<int>(std::max<int64_t>(1, get_parameter("min_landmark_hits").as_int()));
        fallback_dt_ =
            std::clamp(get_parameter("fallback_dt").as_double(), 0.005, 0.2);

        cones_topic_ = get_parameter("cones_topic").as_string();
        odom_topic_ = get_parameter("odom_topic").as_string();

        Q_control_ << 0.2 * 0.2, 0.0,
                      0.0, 0.15 * 0.15;

        particles_.reserve(static_cast<std::size_t>(num_particles_));
        for (int i = 0; i < num_particles_; ++i)
        {
            Particle particle;
            particle.weight = 1.0 / static_cast<double>(num_particles_);
            particle.P = Eigen::Matrix3d::Identity() * 0.001;
            particles_.push_back(particle);
        }

        cones_pub_ =
            create_publisher<eufs_msgs::msg::ConeArray>("/planning/cones", 10);
        landmark_cov_pub_ =
            create_publisher<eufs_msgs::msg::ConeArrayWithCovariance>(
                "/slam/landmarks", 10);
        slam_odom_pub_ =
            create_publisher<nav_msgs::msg::Odometry>("/slam/odom", 10);
        native_marker_pub_ =
            create_publisher<visualization_msgs::msg::MarkerArray>(
                "/slam/native_cones", 10);

        tf_broadcaster_ =
            std::make_unique<tf2_ros::TransformBroadcaster>(*this);

        const auto cone_qos = rclcpp::QoS(rclcpp::KeepLast(20)).best_effort();
        cones_sub_ =
            create_subscription<eufs_msgs::msg::ConeArrayWithCovariance>(
                cones_topic_,
                cone_qos,
                std::bind(&FastSLAM2::conesCallback, this, _1));

        const auto odom_qos = rclcpp::QoS(rclcpp::KeepLast(50));
        odom_sub_ = create_subscription<nav_msgs::msg::Odometry>(
            odom_topic_,
            odom_qos,
            std::bind(&FastSLAM2::odomCallback, this, _1));

        timer_ = create_wall_timer(
            std::chrono::milliseconds(50),
            std::bind(&FastSLAM2::runSLAM, this));

        RCLCPP_INFO(
            get_logger(),
            "FastSLAM 2.0 ready: cones=%s odom=%s particles=%d min_hits=%d",
            cones_topic_.c_str(),
            odom_topic_.c_str(),
            num_particles_,
            min_landmark_hits_);
    }

private:
    int num_particles_{30};
    int min_landmark_hits_{1};

    double process_noise_xy_{0.0004};
    double process_noise_yaw_{0.0001};
    double landmark_match_distance_{2.0};
    double fallback_dt_{DEFAULT_DT};

    std::string cones_topic_;
    std::string odom_topic_;

    std::vector<Particle> particles_;
    Eigen::Matrix2d Q_control_{Eigen::Matrix2d::Zero()};

    std::vector<ConeDetection> z_buffer_;
    std::mutex slam_mutex_;

    std::random_device rd_;
    std::mt19937 gen_;

    double vx_{0.0};
    double yaw_rate_{0.0};
    double pending_dt_{0.0};
    double last_cone_time_sec_{0.0};
    double last_odom_time_sec_{0.0};

    rclcpp::Subscription<eufs_msgs::msg::ConeArrayWithCovariance>::SharedPtr
        cones_sub_;
    rclcpp::Subscription<nav_msgs::msg::Odometry>::SharedPtr odom_sub_;

    rclcpp::Publisher<eufs_msgs::msg::ConeArray>::SharedPtr cones_pub_;
    rclcpp::Publisher<eufs_msgs::msg::ConeArrayWithCovariance>::SharedPtr
        landmark_cov_pub_;
    rclcpp::Publisher<nav_msgs::msg::Odometry>::SharedPtr slam_odom_pub_;
    rclcpp::Publisher<visualization_msgs::msg::MarkerArray>::SharedPtr
        native_marker_pub_;

    std::unique_ptr<tf2_ros::TransformBroadcaster> tf_broadcaster_;
    rclcpp::TimerBase::SharedPtr timer_;

    static double stampToSec(const builtin_interfaces::msg::Time &stamp)
    {
        return static_cast<double>(stamp.sec) +
               static_cast<double>(stamp.nanosec) * 1e-9;
    }

    bool validCovariance(const Eigen::Matrix2d &covariance) const
    {
        if (!covariance.allFinite())
            return false;

        const double symmetry_error =
            std::abs(covariance(0, 1) - covariance(1, 0));
        if (symmetry_error > 1e-8)
            return false;

        if (covariance(0, 0) < 0.0 || covariance(1, 1) < 0.0)
            return false;

        return covariance.determinant() >= -1e-10;
    }

    bool validPoseCovariance(const Eigen::Matrix3d &covariance) const
    {
        if (!covariance.allFinite())
            return false;

        Eigen::SelfAdjointEigenSolver<Eigen::Matrix3d> solver(covariance);
        if (solver.info() != Eigen::Success)
            return false;

        return solver.eigenvalues().minCoeff() >= -1e-9;
    }

    void addConeToBuffer(
        const eufs_msgs::msg::ConeWithCovariance &cone,
        int color)
    {
        ConeDetection detection;
        detection.range = std::hypot(cone.point.x, cone.point.y);
        detection.bearing = std::atan2(cone.point.y, cone.point.x);
        detection.color = color;

        Eigen::Matrix2d point_covariance;
        point_covariance << cone.covariance[0], cone.covariance[1],
                            cone.covariance[2], cone.covariance[3];

        if (detection.range <= 0.1 ||
            detection.range >= 20.0 ||
            !validCovariance(point_covariance))
        {
            return;
        }

        const double range_sq =
            std::max(detection.range * detection.range, 1e-9);

        Eigen::Matrix2d polar_jacobian;
        polar_jacobian <<
            cone.point.x / detection.range,
            cone.point.y / detection.range,
            -cone.point.y / range_sq,
            cone.point.x / range_sq;

        detection.covariance =
            polar_jacobian * point_covariance * polar_jacobian.transpose();

        detection.covariance =
            0.5 * (detection.covariance +
                   detection.covariance.transpose());

        if (validCovariance(detection.covariance))
            z_buffer_.push_back(detection);
    }

    void conesCallback(
        const eufs_msgs::msg::ConeArrayWithCovariance::SharedPtr msg)
    {
        std::lock_guard<std::mutex> lock(slam_mutex_);

        z_buffer_.clear();

        for (const auto &cone : msg->blue_cones)
            addConeToBuffer(cone, 0);

        for (const auto &cone : msg->yellow_cones)
            addConeToBuffer(cone, 1);

        for (const auto &cone : msg->orange_cones)
            addConeToBuffer(cone, 2);

        for (const auto &cone : msg->big_orange_cones)
            addConeToBuffer(cone, 2);

        for (const auto &cone : msg->unknown_color_cones)
            addConeToBuffer(cone, 3);

        const double current_time = stampToSec(msg->header.stamp);

        if (last_cone_time_sec_ > 0.0)
        {
            const double cone_dt = current_time - last_cone_time_sec_;

            // If no usable odometry is arriving, cone timestamps provide
            // a deterministic fallback clock so SLAM can still initialize.
            if (cone_dt > 1e-4 && cone_dt < 0.2)
                pending_dt_ = std::max(pending_dt_, cone_dt);
        }

        last_cone_time_sec_ = current_time;
    }

    void odomCallback(const nav_msgs::msg::Odometry::SharedPtr msg)
    {
        std::lock_guard<std::mutex> lock(slam_mutex_);

        vx_ = msg->twist.twist.linear.x;
        yaw_rate_ = msg->twist.twist.angular.z;

        const double current_time = stampToSec(msg->header.stamp);

        if (last_odom_time_sec_ > 0.0)
        {
            const double odom_dt = current_time - last_odom_time_sec_;

            if (odom_dt > 1e-4 && odom_dt < 0.2)
                pending_dt_ += odom_dt;
        }

        last_odom_time_sec_ = current_time;
    }

    void predictParticles(double dt, double vx, double yaw_rate)
    {
        const double sigma_vx =
            std::max(std::sqrt(Q_control_(0, 0)),
                     std::abs(vx) * 0.05);
        const double sigma_yaw =
            std::max(std::sqrt(Q_control_(1, 1)),
                     std::abs(yaw_rate) * 0.05);
        const double sigma_vy =
            std::max(0.01, std::abs(yaw_rate) * 0.05);

        std::normal_distribution<double> noise_vx(0.0, sigma_vx);
        std::normal_distribution<double> noise_vy(0.0, sigma_vy);
        std::normal_distribution<double> noise_yaw(0.0, sigma_yaw);

        for (auto &particle : particles_)
        {
            const double noisy_vx = vx + noise_vx(gen_);
            const double noisy_vy = noise_vy(gen_);
            const double noisy_yaw_rate = yaw_rate + noise_yaw(gen_);

            const double old_yaw = particle.yaw;

            particle.x +=
                (noisy_vx * std::cos(old_yaw) -
                 noisy_vy * std::sin(old_yaw)) * dt;

            particle.y +=
                (noisy_vx * std::sin(old_yaw) +
                 noisy_vy * std::cos(old_yaw)) * dt;

            particle.yaw =
                wrapToPi(old_yaw + noisy_yaw_rate * dt);

            Eigen::Matrix3d G = Eigen::Matrix3d::Identity();

            G(0, 2) =
                (-noisy_vx * std::sin(old_yaw) -
                 noisy_vy * std::cos(old_yaw)) * dt;

            G(1, 2) =
                (noisy_vx * std::cos(old_yaw) -
                 noisy_vy * std::sin(old_yaw)) * dt;

            Eigen::Matrix3d Q_pose = Eigen::Matrix3d::Zero();
            Q_pose(0, 0) = process_noise_xy_;
            Q_pose(1, 1) = process_noise_xy_;
            Q_pose(2, 2) = process_noise_yaw_;

            particle.P =
                G * particle.P * G.transpose() + Q_pose;

            particle.P =
                0.5 * (particle.P + particle.P.transpose());
        }
    }

    bool measurementModel(
        const Particle &particle,
        const Landmark &landmark,
        Eigen::Vector2d &predicted,
        Eigen::Matrix2d &H_landmark,
        Eigen::Matrix<double, 2, 3> &H_pose) const
    {
        const double dx = landmark.mu(0) - particle.x;
        const double dy = landmark.mu(1) - particle.y;
        const double q = dx * dx + dy * dy;

        if (q < 1e-8)
            return false;

        const double range = std::sqrt(q);

        predicted(0) = range;
        predicted(1) =
            wrapToPi(std::atan2(dy, dx) - particle.yaw);

        H_landmark <<
            dx / range, dy / range,
            -dy / q,    dx / q;

        H_pose <<
            -dx / range, -dy / range, 0.0,
             dy / q,     -dx / q,    -1.0;

        return true;
    }

    bool updateOneMeasurement(Particle &particle, const ConeDetection &z)
    {
        int best_index = -1;
        double best_distance = std::numeric_limits<double>::max();
        double best_mahalanobis = std::numeric_limits<double>::max();

        const double global_theta =
            wrapToPi(particle.yaw + z.bearing);

        const Eigen::Vector2d observed_global(
            particle.x + z.range * std::cos(global_theta),
            particle.y + z.range * std::sin(global_theta));

        for (std::size_t i = 0; i < particle.map.size(); ++i)
        {
            Landmark &landmark = particle.map[i];

            if (landmark.color != z.color)
                continue;

            const double distance =
                (landmark.mu - observed_global).norm();

            if (distance > landmark_match_distance_)
                continue;

            Eigen::Vector2d z_hat;
            Eigen::Matrix2d H_landmark;
            Eigen::Matrix<double, 2, 3> H_pose;

            if (!measurementModel(
                    particle,
                    landmark,
                    z_hat,
                    H_landmark,
                    H_pose))
            {
                continue;
            }

            Eigen::Matrix2d S =
                H_landmark * landmark.sigma *
                    H_landmark.transpose() +
                z.covariance;

            S += Eigen::Matrix2d::Identity() * 1e-9;

            Eigen::LDLT<Eigen::Matrix2d> solver(S);
            if (solver.info() != Eigen::Success)
                continue;

            const Eigen::Vector2d innovation(
                z.range - z_hat(0),
                wrapToPi(z.bearing - z_hat(1)));

            const Eigen::Vector2d whitened =
                solver.solve(innovation);

            const double mahalanobis =
                innovation.dot(whitened);

            if (!std::isfinite(mahalanobis) ||
                mahalanobis > 9.21)
            {
                continue;
            }

            if (mahalanobis < best_mahalanobis ||
                (std::abs(mahalanobis - best_mahalanobis) < 1e-9 &&
                 distance < best_distance))
            {
                best_index = static_cast<int>(i);
                best_distance = distance;
                best_mahalanobis = mahalanobis;
            }
        }

        if (best_index < 0)
        {
            // Initialize a new landmark from the current pose and
            // the measured cone range/bearing.
            Landmark landmark;

            landmark.mu <<
                observed_global.x(),
                observed_global.y();

            const double c = std::cos(global_theta);
            const double s = std::sin(global_theta);

            Eigen::Matrix2d Gz;
            Gz <<
                c, -z.range * s,
                s,  z.range * c;

            Eigen::Matrix<double, 2, 3> Gpose;
            Gpose <<
                1.0, 0.0, -z.range * s,
                0.0, 1.0,  z.range * c;

            landmark.sigma =
                Gz * z.covariance * Gz.transpose() +
                Gpose * particle.P * Gpose.transpose();

            landmark.sigma =
                0.5 * (landmark.sigma +
                       landmark.sigma.transpose());

            // Keep a small covariance floor so the visualization remains
            // meaningful rather than collapsing to a zero-size ellipse.
            landmark.sigma += Eigen::Matrix2d::Identity() * 1e-6;

            if (!validCovariance(landmark.sigma))
                return false;

            landmark.color = z.color;
            landmark.hits = 1;
            particle.map.push_back(landmark);

            return true;
        }

        Landmark &landmark =
            particle.map[static_cast<std::size_t>(best_index)];

        Eigen::Vector2d z_hat;
        Eigen::Matrix2d H_landmark;
        Eigen::Matrix<double, 2, 3> H_pose;

        if (!measurementModel(
                particle,
                landmark,
                z_hat,
                H_landmark,
                H_pose))
        {
            return false;
        }

        Eigen::Matrix2d S =
            H_landmark * landmark.sigma *
                H_landmark.transpose() +
            z.covariance;

        S += Eigen::Matrix2d::Identity() * 1e-9;

        Eigen::LDLT<Eigen::Matrix2d> solver(S);
        if (solver.info() != Eigen::Success)
            return false;

        const Eigen::Vector2d innovation(
            z.range - z_hat(0),
            wrapToPi(z.bearing - z_hat(1)));

        const Eigen::Matrix2d K_landmark =
            landmark.sigma *
            H_landmark.transpose() *
            solver.solve(Eigen::Matrix2d::Identity());

        landmark.mu += K_landmark * innovation;
        landmark.sigma =
            (Eigen::Matrix2d::Identity() -
             K_landmark * H_landmark) *
            landmark.sigma;

        landmark.sigma =
            0.5 * (landmark.sigma +
                   landmark.sigma.transpose());

        landmark.sigma += Eigen::Matrix2d::Identity() * 1e-9;
        landmark.hits++;

        // FastSLAM 2.0 pose proposal.
        Eigen::Matrix3d prior_information =
            particle.P.inverse();

        Eigen::Matrix3d proposal_information =
            H_pose.transpose() *
            solver.solve(H_pose) +
            prior_information;

        Eigen::LDLT<Eigen::Matrix3d> proposal_solver(
            proposal_information);

        if (proposal_solver.info() == Eigen::Success)
        {
            const Eigen::Matrix3d proposal_covariance =
                proposal_solver.solve(Eigen::Matrix3d::Identity());

            const Eigen::Vector3d state_update =
                proposal_covariance *
                H_pose.transpose() *
                solver.solve(innovation);

            particle.x += state_update(0);
            particle.y += state_update(1);
            particle.yaw =
                wrapToPi(particle.yaw + state_update(2));

            particle.P =
                0.5 * (proposal_covariance +
                       proposal_covariance.transpose());
        }

        // Particle importance weight.
        const double determinant =
            std::max(1e-12, S.determinant());

        const double likelihood =
            std::exp(-0.5 * best_mahalanobis) /
            (2.0 * PI * std::sqrt(determinant));

        if (std::isfinite(likelihood) && likelihood > 1e-12)
            particle.weight *= likelihood;

        return true;
    }

    void updateParticles(
        const std::vector<ConeDetection> &measurements)
    {
        for (auto &particle : particles_)
        {
            for (const auto &measurement : measurements)
            {
                updateOneMeasurement(particle, measurement);
            }
        }
    }

    void resampleParticles()
    {
        double sum_weight = 0.0;

        for (const auto &particle : particles_)
            sum_weight += particle.weight;

        if (!std::isfinite(sum_weight) ||
            sum_weight <= 1e-12)
        {
            for (auto &particle : particles_)
                particle.weight =
                    1.0 / static_cast<double>(num_particles_);

            return;
        }

        double weight_square_sum = 0.0;

        for (auto &particle : particles_)
        {
            particle.weight /= sum_weight;
            weight_square_sum +=
                particle.weight * particle.weight;
        }

        const double n_effective =
            1.0 / std::max(weight_square_sum, 1e-12);

        if (n_effective >=
            0.5 * static_cast<double>(num_particles_))
        {
            return;
        }

        std::vector<Particle> new_particles;
        new_particles.reserve(
            static_cast<std::size_t>(num_particles_));

        std::uniform_real_distribution<double> distribution(
            0.0,
            1.0 / static_cast<double>(num_particles_));

        double r = distribution(gen_);
        double cumulative = particles_[0].weight;
        std::size_t index = 0;

        for (int m = 0; m < num_particles_; ++m)
        {
            const double u =
                r + static_cast<double>(m) /
                        static_cast<double>(num_particles_);

            while (u > cumulative &&
                   index + 1 < particles_.size())
            {
                ++index;
                cumulative += particles_[index].weight;
            }

            Particle copy = particles_[index];
            copy.weight =
                1.0 / static_cast<double>(num_particles_);

            new_particles.push_back(copy);
        }

        particles_ = std::move(new_particles);

        // Small rejuvenation noise keeps the particle distribution
        // informative for covariance estimation.
        std::normal_distribution<double> position_jitter(
            0.0, 0.005);
        std::normal_distribution<double> yaw_jitter(
            0.0, 0.002);

        for (std::size_t i = 1; i < particles_.size(); ++i)
        {
            particles_[i].x += position_jitter(gen_);
            particles_[i].y += position_jitter(gen_);
            particles_[i].yaw =
                wrapToPi(
                    particles_[i].yaw +
                    yaw_jitter(gen_));

            particles_[i].P +=
                Eigen::Matrix3d::Identity() * 1e-6;
        }
    }

    Particle getBestParticle() const
    {
        return *std::max_element(
            particles_.begin(),
            particles_.end(),
            [](const Particle &a, const Particle &b)
            {
                return a.weight < b.weight;
            });
    }

    void aggregatePose(
        Eigen::Vector3d &mean,
        Eigen::Matrix3d &covariance) const
    {
        mean.setZero();
        covariance.setZero();

        double total_weight = 0.0;
        for (const auto &particle : particles_)
            total_weight += particle.weight;

        const bool fallback =
            !std::isfinite(total_weight) ||
            total_weight <= 1e-12;

        const double uniform_weight =
            1.0 / static_cast<double>(particles_.size());

        double sin_yaw = 0.0;
        double cos_yaw = 0.0;

        for (const auto &particle : particles_)
        {
            const double weight =
                fallback
                    ? uniform_weight
                    : particle.weight / total_weight;

            mean(0) += weight * particle.x;
            mean(1) += weight * particle.y;
            sin_yaw += weight * std::sin(particle.yaw);
            cos_yaw += weight * std::cos(particle.yaw);
        }

        mean(2) =
            std::atan2(sin_yaw, cos_yaw);

        for (const auto &particle : particles_)
        {
            const double weight =
                fallback
                    ? uniform_weight
                    : particle.weight / total_weight;

            Eigen::Vector3d delta;
            delta <<
                particle.x - mean(0),
                particle.y - mean(1),
                wrapToPi(particle.yaw - mean(2));

            covariance +=
                weight *
                (particle.P +
                 delta * delta.transpose());
        }

        covariance =
            0.5 * (covariance + covariance.transpose());

        covariance(0, 0) =
            std::max(covariance(0, 0), 1e-8);
        covariance(1, 1) =
            std::max(covariance(1, 1), 1e-8);
        covariance(2, 2) =
            std::max(covariance(2, 2), 1e-8);
    }

    eufs_msgs::msg::ConeArrayWithCovariance
    aggregateLandmarks(
        const Particle &reference,
        const rclcpp::Time &stamp) const
    {
        eufs_msgs::msg::ConeArrayWithCovariance output;

        output.header.stamp = stamp;
        output.header.frame_id = "map";

        double total_weight = 0.0;
        for (const auto &particle : particles_)
            total_weight += particle.weight;

        const bool fallback =
            !std::isfinite(total_weight) ||
            total_weight <= 1e-12;

        const double uniform_weight =
            1.0 / static_cast<double>(particles_.size());

        for (const auto &reference_landmark : reference.map)
        {
            if (reference_landmark.hits < min_landmark_hits_)
                continue;

            Eigen::Vector2d mean =
                Eigen::Vector2d::Zero();

            double matched_weight = 0.0;

            for (const auto &particle : particles_)
            {
                const Landmark *nearest = nullptr;
                double nearest_distance =
                    landmark_match_distance_;

                for (const auto &candidate : particle.map)
                {
                    if (candidate.color !=
                        reference_landmark.color)
                    {
                        continue;
                    }

                    const double distance =
                        (candidate.mu -
                         reference_landmark.mu).norm();

                    if (distance < nearest_distance)
                    {
                        nearest = &candidate;
                        nearest_distance = distance;
                    }
                }

                if (nearest == nullptr)
                    continue;

                const double weight =
                    fallback
                        ? uniform_weight
                        : particle.weight /
                              total_weight;

                mean += weight * nearest->mu;
                matched_weight += weight;
            }

            if (matched_weight <= 1e-12)
                continue;

            mean /= matched_weight;

            Eigen::Matrix2d covariance =
                Eigen::Matrix2d::Zero();

            for (const auto &particle : particles_)
            {
                const Landmark *nearest = nullptr;
                double nearest_distance =
                    landmark_match_distance_;

                for (const auto &candidate : particle.map)
                {
                    if (candidate.color !=
                        reference_landmark.color)
                    {
                        continue;
                    }

                    const double distance =
                        (candidate.mu -
                         reference_landmark.mu).norm();

                    if (distance < nearest_distance)
                    {
                        nearest = &candidate;
                        nearest_distance = distance;
                    }
                }

                if (nearest == nullptr)
                    continue;

                const double normalized_weight =
                    (fallback
                         ? uniform_weight
                         : particle.weight /
                               total_weight) /
                    matched_weight;

                const Eigen::Vector2d delta =
                    nearest->mu - mean;

                covariance +=
                    normalized_weight *
                    (nearest->sigma +
                     delta * delta.transpose());
            }

            covariance =
                0.5 * (covariance +
                       covariance.transpose());

            covariance +=
                Eigen::Matrix2d::Identity() * 1e-9;

            if (!validCovariance(covariance))
                continue;

            eufs_msgs::msg::ConeWithCovariance cone;
            cone.point.x = mean(0);
            cone.point.y = mean(1);
            cone.point.z = 0.0;

            cone.covariance = {
                covariance(0, 0),
                covariance(0, 1),
                covariance(1, 0),
                covariance(1, 1)};

            if (reference_landmark.color == 0)
                output.blue_cones.push_back(cone);
            else if (reference_landmark.color == 1)
                output.yellow_cones.push_back(cone);
            else if (reference_landmark.color == 2)
                output.big_orange_cones.push_back(cone);
            else
                output.unknown_color_cones.push_back(cone);
        }

        return output;
    }

    void publishPlanningCones(const Particle &particle)
    {
        eufs_msgs::msg::ConeArray output;
        output.header.stamp = now();
        output.header.frame_id = "map";

        for (const auto &landmark : particle.map)
        {
            if (landmark.hits < min_landmark_hits_)
                continue;

            geometry_msgs::msg::Point point;
            point.x = landmark.mu(0);
            point.y = landmark.mu(1);
            point.z = 0.0;

            if (landmark.color == 0)
                output.blue_cones.push_back(point);
            else if (landmark.color == 1)
                output.yellow_cones.push_back(point);
            else if (landmark.color == 2)
                output.big_orange_cones.push_back(point);
        }

        cones_pub_->publish(output);
    }

    void publishNativeMarkers(
        const Particle &particle)
    {
        visualization_msgs::msg::MarkerArray array;

        visualization_msgs::msg::Marker clear;
        clear.action =
            visualization_msgs::msg::Marker::DELETEALL;
        array.markers.push_back(clear);

        int id = 0;

        for (const auto &landmark : particle.map)
        {
            if (landmark.hits < min_landmark_hits_)
                continue;

            visualization_msgs::msg::Marker marker;

            marker.header.stamp = now();
            marker.header.frame_id = "map";
            marker.ns = "track_cones";
            marker.id = id++;

            marker.type =
                visualization_msgs::msg::Marker::CYLINDER;
            marker.action =
                visualization_msgs::msg::Marker::ADD;

            marker.pose.position.x = landmark.mu(0);
            marker.pose.position.y = landmark.mu(1);
            marker.pose.position.z = 0.15;

            marker.scale.x = 0.32;
            marker.scale.y = 0.32;
            marker.scale.z = 0.30;

            marker.color.a = 1.0;

            if (landmark.color == 0)
            {
                marker.color.r = 0.0;
                marker.color.g = 0.15;
                marker.color.b = 1.0;
            }
            else if (landmark.color == 1)
            {
                marker.color.r = 1.0;
                marker.color.g = 0.9;
                marker.color.b = 0.0;
            }
            else if (landmark.color == 2)
            {
                marker.color.r = 1.0;
                marker.color.g = 0.35;
                marker.color.b = 0.0;
            }
            else
            {
                marker.color.r = 0.7;
                marker.color.g = 0.7;
                marker.color.b = 0.7;
            }

            array.markers.push_back(marker);
        }

        native_marker_pub_->publish(array);
    }

    void publishPose(
        const Eigen::Vector3d &pose,
        const Eigen::Matrix3d &covariance,
        double linear_velocity,
        double yaw_rate,
        const rclcpp::Time &stamp)
    {
        nav_msgs::msg::Odometry odom;

        odom.header.stamp = stamp;
        odom.header.frame_id = "map";
        odom.child_frame_id = "base_footprint";

        odom.pose.pose.position.x = pose(0);
        odom.pose.pose.position.y = pose(1);

        odom.pose.pose.orientation.z =
            std::sin(pose(2) * 0.5);
        odom.pose.pose.orientation.w =
            std::cos(pose(2) * 0.5);

        odom.pose.covariance[0] = covariance(0, 0);
        odom.pose.covariance[1] = covariance(0, 1);
        odom.pose.covariance[5] = covariance(0, 2);

        odom.pose.covariance[6] = covariance(1, 0);
        odom.pose.covariance[7] = covariance(1, 1);
        odom.pose.covariance[11] = covariance(1, 2);

        odom.pose.covariance[30] = covariance(2, 0);
        odom.pose.covariance[31] = covariance(2, 1);
        odom.pose.covariance[35] = covariance(2, 2);

        odom.twist.twist.linear.x =
            linear_velocity;
        odom.twist.twist.angular.z =
            yaw_rate;

        slam_odom_pub_->publish(odom);

        geometry_msgs::msg::TransformStamped transform;

        transform.header.stamp = stamp;
        transform.header.frame_id = "map";
        transform.child_frame_id = "base_footprint";

        transform.transform.translation.x = pose(0);
        transform.transform.translation.y = pose(1);

        transform.transform.rotation.z =
            std::sin(pose(2) * 0.5);
        transform.transform.rotation.w =
            std::cos(pose(2) * 0.5);

        tf_broadcaster_->sendTransform(transform);
    }

    void runSLAM()
    {
        std::vector<ConeDetection> local_measurements;
        double local_dt = 0.0;
        double local_vx = 0.0;
        double local_yaw_rate = 0.0;

        {
            std::lock_guard<std::mutex> lock(slam_mutex_);

            if (!z_buffer_.empty())
            {
                local_measurements = z_buffer_;
                z_buffer_.clear();
            }

            local_dt = pending_dt_;
            pending_dt_ = 0.0;

            local_vx = vx_;
            local_yaw_rate = yaw_rate_;

            // The old implementation stopped here if odometry had not
            // arrived first. That prevented /camera_0/cones from ever
            // creating the first landmarks. Use cone timing when available,
            // then a small deterministic fallback for simulator startup.
            if (local_dt <= 0.0 && !local_measurements.empty())
                local_dt = fallback_dt_;

            local_dt =
                std::clamp(local_dt, 0.005, 0.2);
        }

        // Nothing to update and no new measurement.
        if (local_measurements.empty() &&
            local_dt <= 0.0)
        {
            return;
        }

        if (local_dt > 0.0)
        {
            predictParticles(
                local_dt,
                local_vx,
                local_yaw_rate);
        }

        if (!local_measurements.empty())
        {
            updateParticles(local_measurements);
            resampleParticles();
        }

        const Particle best_particle =
            getBestParticle();

        publishPlanningCones(best_particle);

        const rclcpp::Time stamp = now();

        landmark_cov_pub_->publish(
            aggregateLandmarks(
                best_particle,
                stamp));

        Eigen::Vector3d pose_mean;
        Eigen::Matrix3d pose_covariance;

        aggregatePose(
            pose_mean,
            pose_covariance);

        if (!validPoseCovariance(pose_covariance))
        {
            RCLCPP_WARN(
                get_logger(),
                "Aggregate pose covariance invalid; skipping pose publication");
            return;
        }

        publishPose(
            pose_mean,
            pose_covariance,
            local_vx,
            local_yaw_rate,
            stamp);

        publishNativeMarkers(best_particle);

        if (!local_measurements.empty())
        {
            std::size_t blue = 0;
            std::size_t yellow = 0;
            std::size_t orange = 0;

            for (const auto &landmark : best_particle.map)
            {
                if (landmark.hits < min_landmark_hits_)
                    continue;

                if (landmark.color == 0)
                    ++blue;
                else if (landmark.color == 1)
                    ++yellow;
                else if (landmark.color == 2)
                    ++orange;
            }

            RCLCPP_INFO(
                get_logger(),
                "FastSLAM update: measurements=%zu landmarks=%zu blue=%zu yellow=%zu orange=%zu",
                local_measurements.size(),
                best_particle.map.size(),
                blue,
                yellow,
                orange);
        }
    }
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<FastSLAM2>());
    rclcpp::shutdown();
    return 0;
}
