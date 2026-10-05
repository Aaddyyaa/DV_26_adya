#include <algorithm>
#include <cmath>
#include <memory>
#include <string>

#include <rclcpp/rclcpp.hpp>
#include <eufs_msgs/msg/cone_array.hpp>
#include <eufs_msgs/msg/cone_array_with_covariance.hpp>

class SimTrackBridge final : public rclcpp::Node
{
public:
    SimTrackBridge()
        : Node("sim_track_bridge")
    {
        input_topic_ = declare_parameter(
            "input_topic", std::string("/ground_truth/track"));
        output_topic_ = declare_parameter(
            "output_topic", std::string("/planning/cones"));

        sub_ = create_subscription<eufs_msgs::msg::ConeArrayWithCovariance>(
            input_topic_,
            rclcpp::QoS(10),
            std::bind(&SimTrackBridge::callback, this, std::placeholders::_1));

        pub_ = create_publisher<eufs_msgs::msg::ConeArray>(
            output_topic_, rclcpp::QoS(10));

        RCLCPP_INFO(
            get_logger(),
            "Simulation track bridge: %s -> %s",
            input_topic_.c_str(),
            output_topic_.c_str());
    }

private:
    void callback(
        const eufs_msgs::msg::ConeArrayWithCovariance::SharedPtr msg)
    {
        eufs_msgs::msg::ConeArray out;
        out.header = msg->header;
        if (out.header.frame_id.empty())
            out.header.frame_id = "map";

        auto copy = [](const auto &source, auto &target)
        {
            target.reserve(source.size());
            for (const auto &cone : source)
            {
                if (!std::isfinite(cone.point.x) ||
                    !std::isfinite(cone.point.y))
                    continue;

                target.push_back(cone.point);
            }
        };

        copy(msg->blue_cones, out.blue_cones);
        copy(msg->yellow_cones, out.yellow_cones);
        copy(msg->orange_cones, out.orange_cones);
        copy(msg->big_orange_cones, out.big_orange_cones);
        copy(msg->unknown_color_cones, out.unknown_color_cones);

        pub_->publish(out);
    }

    std::string input_topic_;
    std::string output_topic_;

    rclcpp::Subscription<eufs_msgs::msg::ConeArrayWithCovariance>::SharedPtr
        sub_;
    rclcpp::Publisher<eufs_msgs::msg::ConeArray>::SharedPtr pub_;
};

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);
    rclcpp::spin(std::make_shared<SimTrackBridge>());
    rclcpp::shutdown();
    return 0;
}
