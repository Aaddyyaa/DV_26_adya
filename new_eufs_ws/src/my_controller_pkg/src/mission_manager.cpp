#include <rclcpp/rclcpp.hpp>
#include <eufs_msgs/srv/set_can_state.hpp>
#include <eufs_msgs/msg/can_state.hpp>
#include <chrono>
#include <string>
#include <functional>
#include <cstdint>

using namespace std::chrono_literals;

class MissionManager : public rclcpp::Node
{
public:
    MissionManager() : Node("mission_manager")
    {
        declare_parameter<std::string>("mission", "trackdrive");
        declare_parameter<bool>("enable", false);

        client_mission_ =
            create_client<eufs_msgs::srv::SetCanState>(
                "/ros_can/set_mission");

        state_sub_ =
            create_subscription<eufs_msgs::msg::CanState>(
                "/ros_can/state",
                10,
                std::bind(
                    &MissionManager::stateCallback,
                    this,
                    std::placeholders::_1));

        timer_ =
            create_wall_timer(
                500ms,
                std::bind(
                    &MissionManager::checkStatus,
                    this));

        RCLCPP_INFO(
            get_logger(),
            "Mission manager ready. Mission selection is disabled by default; "
            "use the official EUFS GUI for Track Drive.");
    }

private:
    void stateCallback(
        const eufs_msgs::msg::CanState::SharedPtr msg)
    {
        current_state_ = msg->as_state;
        have_state_ = true;

        if (current_state_ ==
            eufs_msgs::msg::CanState::AS_DRIVING)
        {
            RCLCPP_INFO_ONCE(
                get_logger(),
                "EUFS reports AS_DRIVING.");
        }
    }

    void checkStatus()
    {
        // This node is intentionally passive unless explicitly enabled.
        // The official ros_can_sim GUI owns mission selection and Manual Drive
        // transitions; sending competing /ros_can/set_mission requests from a
        // second node was a source of state conflicts.
        if (!get_parameter("enable").as_bool())
            return;

        if (!have_state_)
            return;

        if (!client_mission_->service_is_ready())
            return;

        if (mission_sent_)
            return;

        if (current_state_ !=
            eufs_msgs::msg::CanState::AS_OFF)
        {
            return;
        }

        const std::string mission =
            get_parameter("mission").as_string();

        int ami_state =
            eufs_msgs::msg::CanState::AMI_TRACK_DRIVE;

        if (mission == "acceleration")
            ami_state =
                eufs_msgs::msg::CanState::AMI_ACCELERATION;
        else if (mission == "skidpad")
            ami_state =
                eufs_msgs::msg::CanState::AMI_SKIDPAD;
        else if (mission == "autocross")
            ami_state =
                eufs_msgs::msg::CanState::AMI_AUTOCROSS;

        auto request =
            std::make_shared<
                eufs_msgs::srv::SetCanState::Request>();

        request->ami_state = ami_state;
        request->as_state =
            eufs_msgs::msg::CanState::AS_READY;

        client_mission_->async_send_request(
            request,
            [this](
                rclcpp::Client<
                    eufs_msgs::srv::SetCanState>::SharedFuture future)
            {
                try
                {
                    if (future.get()->success)
                    {
                        mission_sent_ = true;
                        RCLCPP_INFO(
                            get_logger(),
                            "Mission selected successfully.");
                    }
                    else
                    {
                        RCLCPP_WARN(
                            get_logger(),
                            "EUFS rejected mission selection.");
                    }
                }
                catch (const std::exception &error)
                {
                    RCLCPP_ERROR(
                        get_logger(),
                        "Mission service call failed: %s",
                        error.what());
                }
            });
    }

    rclcpp::Client<
        eufs_msgs::srv::SetCanState>::SharedPtr client_mission_;

    rclcpp::Subscription<
        eufs_msgs::msg::CanState>::SharedPtr state_sub_;

    rclcpp::TimerBase::SharedPtr timer_;

    uint8_t current_state_{0};
    bool have_state_{false};
    bool mission_sent_{false};
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);
    rclcpp::spin(
        std::make_shared<MissionManager>());
    rclcpp::shutdown();
    return 0;
}
