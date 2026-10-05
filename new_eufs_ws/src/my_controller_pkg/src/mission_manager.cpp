#include <rclcpp/rclcpp.hpp>

class MissionManager : public rclcpp::Node
{
public:
    MissionManager() : Node("mission_manager")
    {
        RCLCPP_INFO(
            get_logger(),
            "Mission manager is passive; Track Drive mission selection remains owned by the EUFS GUI.");
    }
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<MissionManager>());
    rclcpp::shutdown();
    return 0;
}
