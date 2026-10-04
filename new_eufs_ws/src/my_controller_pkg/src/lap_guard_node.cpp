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
        leave_radius_ = declare_parameter("leave_start_radius_m", 6.0);
        finish_radius_ = declare_parameter("finish_radius_m", 6.0);
        minimum_distance_ = declare_parameter("minimum_distance_before_finish_m", 50.0);

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
    void stateCallback(
        const eufs_msgs::msg::CanState::SharedPtr msg)
    {
        if (msg->as_state == eufs_msgs::msg::CanState::AS_DRIVING)
        {
            driving_ = true;

            if (!have_start_ && !completed_ && have_latest_pose_)
            {
                captureStart(
                    latest_x_,
                    latest_y_);
            }
        }
        else if (!have_start_)
        {
            driving_ = false;
        }
    }

    void captureStart(double x, double y)
    {
        start_x_ = x;
        start_y_ = y;
        have_start_ = true;
        armed_ = true;
        total_distance_ = 0.0;
        left_start_zone_ = false;

        RCLCPP_INFO(
            get_logger(),
            "Lap start captured at (%.2f, %.2f).",
            start_x_,
            start_y_);
    }

    void groundTruthCallback(
        const eufs_msgs::msg::CarState::SharedPtr msg)
    {
        const double x = msg->pose.pose.position.x;
        const double y = msg->pose.pose.position.y;

        latest_x_ = x;
        latest_y_ = y;
        have_latest_pose_ = true;

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

            if (!have_start_)
                captureStart(x, y);

            return;
        }

        total_distance_ += std::hypot(
            x - previous_x_,
            y - previous_y_);

        previous_x_ = x;
        previous_y_ = y;

        // Capture the first pose seen while Track Drive is actually driving.
        // This is preferable to a startup pose from before the mission began.
        if (!have_start_)
        {
            captureStart(x, y);
            return;
        }

        const double distance_to_start =
            std::hypot(x - start_x_, y - start_y_);

        // Hysteresis: the car must leave the grid/start region before a
        // return to that same region can count as a completed lap.
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

        // Do not require a heading match. EUFS can approach the grid/finish
        // area with a different yaw after a tight final turn. Position plus
        // travelled distance is the robust completion condition here.
        if (distance_to_start <= finish_radius_)
        {
            std_msgs::msg::Bool completed_msg;
            completed_msg.data = true;
            finish_pub_->publish(completed_msg);

            completed_ = true;
            armed_ = false;

            RCLCPP_WARN(
                get_logger(),
                "FULL LAP COMPLETE: travelled %.2f m, finish distance %.2f m.",
                total_distance_,
                distance_to_start);
        }
    }

    rclcpp::Subscription<eufs_msgs::msg::CarState>::SharedPtr gt_sub_;
    rclcpp::Subscription<eufs_msgs::msg::CanState>::SharedPtr state_sub_;
    rclcpp::Publisher<std_msgs::msg::Bool>::SharedPtr finish_pub_;

    double leave_radius_{6.0};
    double finish_radius_{6.0};
    double minimum_distance_{50.0};

    double start_x_{0.0};
    double start_y_{0.0};

    double latest_x_{0.0};
    double latest_y_{0.0};
    double previous_x_{0.0};
    double previous_y_{0.0};

    double total_distance_{0.0};

    bool driving_{false};
    bool have_latest_pose_{false};
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
