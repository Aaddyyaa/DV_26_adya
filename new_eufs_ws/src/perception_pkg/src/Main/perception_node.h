#pragma once

#include <memory>

#include <rclcpp/rclcpp.hpp>
#include <sensor_msgs/msg/image.hpp>

#include <opencv2/opencv.hpp>
#include <cv_bridge/cv_bridge.h>
#include "Cone/cone.h"
#include "Keypoints/keypoints.h"
#include "Calibration/rectification.h"
#include "Depth/cone_depth.hpp"
#include "main.h"
#include <eufs_msgs/msg/cone_array.hpp>
#include <eufs_msgs/msg/cone_array_with_covariance.hpp>
#include <geometry_msgs/msg/point.hpp>
#include <array>
#include <deque>
#include <vector>


class PerceptionNode : public rclcpp::Node
{
public:

    PerceptionNode();

private:

    void leftCallback(
        const sensor_msgs::msg::Image::SharedPtr msg);

    void rightCallback(
        const sensor_msgs::msg::Image::SharedPtr msg);

    void processFrame();

    rclcpp::Subscription<sensor_msgs::msg::Image>::SharedPtr left_sub_;

    rclcpp::Subscription<sensor_msgs::msg::Image>::SharedPtr right_sub_;

    rclcpp::Publisher<eufs_msgs::msg::ConeArray>::SharedPtr cone_pub_;
    rclcpp::Publisher<eufs_msgs::msg::ConeArrayWithCovariance>::SharedPtr cone_cov_pub_;

Rectifier rectifier_;

ConeDepth::CameraConfig cam_;

ConeDepth::PipelineConfig depth_cfg_;

cv::dnn::Net cone_net_;

cv::dnn::Net keypoints_net_;

    cv::Mat left_img_;

    cv::Mat right_img_;

    bool left_ready_ = false;

    bool right_ready_ = false;

struct TemporalCone
{
    geometry_msgs::msg::Point point;
    std::array<double, 4> covariance;
    int class_id;
};

std::deque<std::vector<TemporalCone>> history_;

const int HISTORY_SIZE = 5;
const int MIN_OBSERVATIONS = 1;       // publish on first confident hit (moving cones
                                        //   are seen only 1-2 frames before passing)
const double MATCH_DISTANCE = 0.60;    // widen so a real cone correlates frame-to-frame
                                        //   as the car moves (was 0.20 → dropped moving cones)

void temporalFilter(
    const std::vector<ConeDepth::ConeResult>& input,
    eufs_msgs::msg::ConeArray& output,
    eufs_msgs::msg::ConeArrayWithCovariance& covariance_output);

bool measurementCovariance(
    const ConeDepth::ConeResult& result,
    std::array<double, 4>& covariance) const;

double pixel_sigma_at_unit_quality_ = 0.5;
double min_disparity_sigma_px_ = 0.05;
};
