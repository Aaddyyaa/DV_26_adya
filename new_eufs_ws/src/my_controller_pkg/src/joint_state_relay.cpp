#include <memory>

#include <rclcpp/rclcpp.hpp>
#include <sensor_msgs/msg/joint_state.hpp>

class JointStateRelay final : public rclcpp::Node
{
public:
  JointStateRelay()
  : Node("eufs_joint_state_relay")
  {
    const auto qos = rclcpp::SensorDataQoS();

    publisher_ = create_publisher<sensor_msgs::msg::JointState>(
      "/joint_states", qos);

    subscription_ = create_subscription<sensor_msgs::msg::JointState>(
      "/eufs/joint_states",
      qos,
      [this](const sensor_msgs::msg::JointState::SharedPtr msg)
      {
        publisher_->publish(*msg);
      });

    RCLCPP_INFO(
      get_logger(),
      "Relaying /eufs/joint_states -> /joint_states for robot_state_publisher");
  }

private:
  rclcpp::Subscription<sensor_msgs::msg::JointState>::SharedPtr subscription_;
  rclcpp::Publisher<sensor_msgs::msg::JointState>::SharedPtr publisher_;
};

int main(int argc, char **argv)
{
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<JointStateRelay>());
  rclcpp::shutdown();
  return 0;
}
