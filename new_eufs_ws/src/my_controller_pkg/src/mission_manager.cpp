#include <rclcpp/rclcpp.hpp>
#include <eufs_msgs/srv/set_can_state.hpp>
#include <eufs_msgs/msg/can_state.hpp>
#include <std_srvs/srv/trigger.hpp>
#include <chrono>

using namespace std::chrono_literals;

class MissionManager : public rclcpp::Node {
public:
    MissionManager() : Node("mission_manager") {
        this->declare_parameter("mission", "skidpad");

        client_reset_ = this->create_client<std_srvs::srv::Trigger>("/ros_can/reset");
        client_mission_ = this->create_client<eufs_msgs::srv::SetCanState>("/ros_can/set_mission");
        
        state_pub_ = this->create_publisher<eufs_msgs::msg::CanState>("/ros_can/state", 10);
        
        sub_state_ = this->create_subscription<eufs_msgs::msg::CanState>(
            "/ros_can/state", 10, std::bind(&MissionManager::state_cb, this, std::placeholders::_1));
            
        timer_ = this->create_wall_timer(1000ms, std::bind(&MissionManager::check_status, this));
        
        RCLCPP_INFO(this->get_logger(), "Universal Mission Manager Initialized.");
    }

private:
    void state_cb(const eufs_msgs::msg::CanState::SharedPtr msg) { 
        current_state_ = msg->as_state; 
    }

    void check_status() {
        if (!client_mission_->wait_for_service(1s)) {
            RCLCPP_INFO_ONCE(this->get_logger(), "Waiting for EUFS mission service...");
            return;
        }

        std::string mission_str = get_parameter("mission").as_string();
        int ami_state = eufs_msgs::msg::CanState::AMI_TRACK_DRIVE;

        if (mission_str == "acceleration")
            ami_state = eufs_msgs::msg::CanState::AMI_ACCELERATION;
        else if (mission_str == "skidpad")
            ami_state = eufs_msgs::msg::CanState::AMI_SKIDPAD;
        else if (mission_str == "autocross")
            ami_state = eufs_msgs::msg::CanState::AMI_AUTOCROSS;
        else if (mission_str == "trackdrive")
            ami_state = eufs_msgs::msg::CanState::AMI_TRACK_DRIVE;

        // The EUFS simulator starts in AS_OFF with AMI_NOT_SELECTED.
        // Selecting the mission is what moves it to AS_READY; repeatedly
        // calling /ros_can/reset here kept the car permanently OFF.
        if (current_state_ == eufs_msgs::msg::CanState::AS_OFF && !mission_sent_) {
            auto req = std::make_shared<eufs_msgs::srv::SetCanState::Request>();
            req->ami_state = ami_state;
            req->as_state = eufs_msgs::msg::CanState::AS_DRIVING;

            RCLCPP_INFO(
                this->get_logger(),
                "AS is OFF. Selecting autonomous mission: %s",
                mission_str.c_str());

            client_mission_->async_send_request(
                req,
                [this](rclcpp::Client<eufs_msgs::srv::SetCanState>::SharedFuture future) {
                    try {
                        if (future.get()->success) {
                            mission_sent_ = true;
                            RCLCPP_INFO(
                                this->get_logger(),
                                "Mission selected; waiting for EUFS AS_READY -> AS_DRIVING transition.");
                        } else {
                            RCLCPP_WARN(
                                this->get_logger(),
                                "EUFS rejected the mission request.");
                        }
                    } catch (const std::exception &error) {
                        RCLCPP_ERROR(
                            this->get_logger(),
                            "Mission request failed: %s",
                            error.what());
                    }
                });
        }
        else if (current_state_ == eufs_msgs::msg::CanState::AS_READY) {
            RCLCPP_INFO_ONCE(
                this->get_logger(),
                "EUFS AS_READY: waiting for automatic transition to AS_DRIVING.");
        }
        else if (current_state_ == eufs_msgs::msg::CanState::AS_DRIVING) {
            RCLCPP_INFO_ONCE(
                this->get_logger(),
                "SUCCESS: EUFS is in AS_DRIVING.");
        }
    }

    rclcpp::Client<std_srvs::srv::Trigger>::SharedPtr client_reset_;
    rclcpp::Client<eufs_msgs::srv::SetCanState>::SharedPtr client_mission_;
    rclcpp::Publisher<eufs_msgs::msg::CanState>::SharedPtr state_pub_;
    rclcpp::Subscription<eufs_msgs::msg::CanState>::SharedPtr sub_state_;
    rclcpp::TimerBase::SharedPtr timer_;
    uint16_t current_state_ = 0;
};

int main(int argc, char **argv) {
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<MissionManager>());
    rclcpp::shutdown();
    return 0;
}