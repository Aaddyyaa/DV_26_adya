#include <cmath>
#include <functional>
#include <memory>

#include <rclcpp/rclcpp.hpp>
#include <std_msgs/msg/bool.hpp>
#include <eufs_msgs/msg/car_state.hpp>

class LapGuardNode final : public rclcpp::Node
{
public:
    LapGuardNode()
        : Node("lap_guard")
    {
        min_distance_ = declare_parameter("min_distance_before_finish_m", 60.0);
        finish_radius_ = declare_parameter("finish_radius_m", 3.0);
        finish_line_half_width_ =
            declare_parameter("finish_line_half_width_m", 3.0);

        gt_sub_ = create_subscription<eufs_msgs::msg::CarState>(
            "/ground_truth/state",
            rclcpp::SensorDataQoS(),
            std::bind(&LapGuardNode::groundTruthCallback, this, std::placeholders::_1));

        finish_pub_ = create_publisher<std_msgs::msg::Bool>(
            "/ros_can/mission_completed", rclcpp::QoS(1).reliable());

        RCLCPP_INFO(
            get_logger(),
            "Lap guard armed: finish after %.1f m traveled, finish radius %.1f m, line half-width %.1f m.",
            min_distance_, finish_radius_, finish_line_half_width_);
    }

private:
    void groundTruthCallback(const eufs_msgs::msg::CarState::SharedPtr msg)
    {
        const double x = msg->pose.pose.position.x;
        const double y = msg->pose.pose.position.y;
        const auto &q = msg->pose.pose.orientation;
        const double yaw = std::atan2(
            2.0 * (q.w * q.z + q.x * q.y),
            1.0 - 2.0 * (q.y * q.y + q.z * q.z));

        if (!have_start_)
        {
            start_x_ = x;
            start_y_ = y;
            start_yaw_ = yaw;
            previous_x_ = x;
            previous_y_ = y;
            have_start_ = true;

            RCLCPP_INFO(
                get_logger(),
                "LAP STARTED: start=(%.2f, %.2f), heading=%.2f rad.",
                start_x_,
                start_y_,
                start_yaw_);
            return;
        }

        const double segment_start_x = previous_x_;
        const double segment_start_y = previous_y_;

        total_distance_ += std::hypot(
            x - segment_start_x,
            y - segment_start_y);

        previous_x_ = x;
        previous_y_ = y;

        if (completed_ || total_distance_ < min_distance_)
            return;

        const double distance_to_start =
            std::hypot(x - start_x_, y - start_y_);

        const double heading_error = std::atan2(
            std::sin(yaw - start_yaw_),
            std::cos(yaw - start_yaw_));

        // Treat the start direction as the finish-line normal. A finish is
        // detected when the vehicle crosses the line from the outside of the
        // start gate toward the track side, while remaining close laterally.
        const double heading_x = std::cos(start_yaw_);
        const double heading_y = std::sin(start_yaw_);

        const double signed_start =
            (segment_start_x - start_x_) * heading_x +
            (segment_start_y - start_y_) * heading_y;

        const double signed_now =
            (x - start_x_) * heading_x +
            (y - start_y_) * heading_y;

        const double lateral_start =
            -(segment_start_x - start_x_) * heading_y +
            (segment_start_y - start_y_) * heading_x;

        const double lateral_now =
            -(x - start_x_) * heading_y +
            (y - start_y_) * heading_x;

        const bool crossed_start_gate =
            signed_start < 0.0 &&
            signed_now >= 0.0 &&
            std::abs(0.5 * (lateral_start + lateral_now)) <=
                std::max(1.0, finish_line_half_width_);

        const bool inside_finish_circle =
            distance_to_start <= finish_radius_ &&
            std::abs(heading_error) <= 1.2;

        if (crossed_start_gate || inside_finish_circle)
        {
            crossed_finish_line_ = crossed_start_gate;

            std_msgs::msg::Bool msg_out;
            msg_out.data = true;
            finish_pub_->publish(msg_out);
            completed_ = true;

            RCLCPP_INFO(
                get_logger(),
                "LAP COMPLETED: traveled %.2f m, finish distance %.2f m%s.",
                total_distance_,
                distance_to_start,
                crossed_start_gate ? " (finish-line crossing)" : "");
        }
    }

    rclcpp::Subscription<eufs_msgs::msg::CarState>::SharedPtr gt_sub_;
    rclcpp::Publisher<std_msgs::msg::Bool>::SharedPtr finish_pub_;

    double min_distance_{60.0};
    double finish_radius_{3.0};
    double finish_line_half_width_{3.0};
    double start_x_{0.0};
    double start_y_{0.0};
    double start_yaw_{0.0};
    double previous_x_{0.0};
    double previous_y_{0.0};
    double total_distance_{0.0};
    bool crossed_finish_line_{false};
    bool have_start_{false};
    bool completed_{false};
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<LapGuardNode>());
    rclcpp::shutdown();
    return 0;
}
