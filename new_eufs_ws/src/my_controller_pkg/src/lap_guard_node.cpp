#include <cmath>
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
        finish_radius_ = declare_parameter("finish_radius_m", 2.0);

        gt_sub_ = create_subscription<eufs_msgs::msg::CarState>(
            "/ground_truth/state",
            rclcpp::SensorDataQoS(),
            std::bind(&LapGuardNode::groundTruthCallback, this, std::placeholders::_1));

        finish_pub_ = create_publisher<std_msgs::msg::Bool>(
            "/ros_can/mission_completed", rclcpp::QoS(1).reliable());

        RCLCPP_INFO(
            get_logger(),
            "Lap guard armed: finish after %.1f m traveled and return within %.1f m of start.",
            min_distance_, finish_radius_);
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
            return;
        }

        total_distance_ += std::hypot(
            x - previous_x_,
            y - previous_y_);

        previous_x_ = x;
        previous_y_ = y;

        if (completed_ || total_distance_ < min_distance_)
            return;

        const double distance_to_start =
            std::hypot(x - start_x_, y - start_y_);

        const double heading_error = std::atan2(
            std::sin(yaw - start_yaw_),
            std::cos(yaw - start_yaw_));

        if (distance_to_start <= finish_radius_ &&
            std::abs(heading_error) <= 0.9)
        {
            std_msgs::msg::Bool msg_out;
            msg_out.data = true;
            finish_pub_->publish(msg_out);
            completed_ = true;

            RCLCPP_WARN(
                get_logger(),
                "FULL LAP COMPLETE: traveled %.2f m, finish distance %.2f m.",
                total_distance_,
                distance_to_start);
        }
    }

    rclcpp::Subscription<eufs_msgs::msg::CarState>::SharedPtr gt_sub_;
    rclcpp::Publisher<std_msgs::msg::Bool>::SharedPtr finish_pub_;

    double min_distance_{60.0};
    double finish_radius_{2.0};
    double start_x_{0.0};
    double start_y_{0.0};
    double start_yaw_{0.0};
    double previous_x_{0.0};
    double previous_y_{0.0};
    double total_distance_{0.0};
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
