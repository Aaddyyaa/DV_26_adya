#include <cmath>
#include <memory>
#include <functional>

#include <rclcpp/rclcpp.hpp>
#include <std_msgs/msg/bool.hpp>
#include <eufs_msgs/msg/car_state.hpp>
#include <eufs_msgs/msg/can_state.hpp>

class LapGuardNode final : public rclcpp::Node
{
public:
    LapGuardNode()
        : Node("lap_guard")
    {
        leave_radius_ = declare_parameter("leave_start_radius_m", 5.0);
        finish_radius_ = declare_parameter("finish_radius_m", 3.0);
        minimum_distance_ = declare_parameter("minimum_distance_before_finish_m", 15.0);
        heading_tolerance_ = declare_parameter("finish_heading_tolerance_rad", 1.2);

        gt_sub_ = create_subscription<eufs_msgs::msg::CarState>(
            "/ground_truth/state",
            rclcpp::SensorDataQoS(),
            std::bind(
                &LapGuardNode::groundTruthCallback,
                this,
                std::placeholders::_1));

        state_sub_ = create_subscription<eufs_msgs::msg::CanState>(
            "/ros_can/state",
            rclcpp::QoS(1).reliable(),
            std::bind(
                &LapGuardNode::stateCallback,
                this,
                std::placeholders::_1));

        finish_pub_ = create_publisher<std_msgs::msg::Bool>(
            "/ros_can/mission_completed",
            rclcpp::QoS(1).reliable());

        RCLCPP_INFO(
            get_logger(),
            "Lap guard ready: arm on AS_DRIVING, leave %.1f m start zone, "
            "then finish inside %.1f m of start after %.1f m travel.",
            leave_radius_,
            finish_radius_,
            minimum_distance_);
    }

private:
    static double yawFromQuaternion(
        const geometry_msgs::msg::Quaternion &q)
    {
        return std::atan2(
            2.0 * (q.w * q.z + q.x * q.y),
            1.0 - 2.0 * (q.y * q.y + q.z * q.z));
    }

    void stateCallback(
        const eufs_msgs::msg::CanState::SharedPtr msg)
    {
        driving_ =
            (msg->as_state == eufs_msgs::msg::CanState::AS_DRIVING);

        if (!driving_ && !completed_)
        {
            armed_ = false;
            have_start_ = false;
            left_start_zone_ = false;
            total_distance_ = 0.0;
        }
    }

    void groundTruthCallback(
        const eufs_msgs::msg::CarState::SharedPtr msg)
    {
        const double x = msg->pose.pose.position.x;
        const double y = msg->pose.pose.position.y;
        const double yaw = yawFromQuaternion(msg->pose.pose.orientation);

        if (!driving_ || completed_)
        {
            previous_x_ = x;
            previous_y_ = y;
            have_previous_ = true;
            return;
        }

        if (!have_previous_)
        {
            previous_x_ = x;
            previous_y_ = y;
            have_previous_ = true;
        }

        total_distance_ += std::hypot(
            x - previous_x_,
            y - previous_y_);

        previous_x_ = x;
        previous_y_ = y;

        // Capture the real start pose only after Track Drive has entered
        // AS_DRIVING. This prevents startup/GUI settling from becoming the
        // finish reference.
        if (!have_start_)
        {
            start_x_ = x;
            start_y_ = y;
            start_yaw_ = yaw;
            have_start_ = true;
            armed_ = true;

            RCLCPP_INFO(
                get_logger(),
                "Lap start captured at (%.2f, %.2f), yaw %.2f rad.",
                start_x_,
                start_y_,
                start_yaw_);
            return;
        }

        const double distance_to_start =
            std::hypot(x - start_x_, y - start_y_);

        // The car must first leave the start/finish area. Without this
        // hysteresis, a stationary car on the grid could immediately satisfy
        // the finish test.
        if (!left_start_zone_ &&
            distance_to_start >= leave_radius_)
        {
            left_start_zone_ = true;

            RCLCPP_INFO(
                get_logger(),
                "Lap guard: car left start zone after %.2f m travelled.",
                total_distance_);
        }

        if (!armed_ ||
            !left_start_zone_ ||
            total_distance_ < minimum_distance_)
        {
            return;
        }

        const double heading_error = std::atan2(
            std::sin(yaw - start_yaw_),
            std::cos(yaw - start_yaw_));

        if (distance_to_start <= finish_radius_ &&
            std::abs(heading_error) <= heading_tolerance_)
        {
            std_msgs::msg::Bool completed_msg;
            completed_msg.data = true;
            finish_pub_->publish(completed_msg);

            completed_ = true;
            armed_ = false;

            RCLCPP_WARN(
                get_logger(),
                "FULL LAP COMPLETE: travelled %.2f m, finish distance %.2f m, "
                "heading error %.2f rad.",
                total_distance_,
                distance_to_start,
                std::abs(heading_error));
        }
    }

    rclcpp::Subscription<eufs_msgs::msg::CarState>::SharedPtr gt_sub_;
    rclcpp::Subscription<eufs_msgs::msg::CanState>::SharedPtr state_sub_;
    rclcpp::Publisher<std_msgs::msg::Bool>::SharedPtr finish_pub_;

    double leave_radius_{5.0};
    double finish_radius_{3.0};
    double minimum_distance_{15.0};
    double heading_tolerance_{1.2};

    double start_x_{0.0};
    double start_y_{0.0};
    double start_yaw_{0.0};

    double previous_x_{0.0};
    double previous_y_{0.0};
    double total_distance_{0.0};

    bool driving_{false};
    bool have_previous_{false};
    bool have_start_{false};
    bool armed_{false};
    bool left_start_zone_{false};
    bool completed_{false};
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<LapGuardNode>());
    rclcpp::shutdown();
    return 0;
}
