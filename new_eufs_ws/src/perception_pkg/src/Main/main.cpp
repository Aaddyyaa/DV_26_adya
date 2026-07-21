#include <memory>

#include "rclcpp/rclcpp.hpp"
#include "perception_node.h"

int main(int argc, char **argv)
{
    rclcpp::init(argc, argv);

    auto node = std::make_shared<PerceptionNode>();

    rclcpp::spin(node);

    rclcpp::shutdown();

    return 0;
}

