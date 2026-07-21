#pragma once

#include <opencv2/opencv.hpp>
#include <string>

class Rectifier
{
public:
    Rectifier() = default;

    bool init_from_file(const std::string& path = "extrinsics.txt");

    void rectify(
        const cv::Mat& left,
        const cv::Mat& right,
        cv::Mat& left_rect,
        cv::Mat& right_rect);

    static void draw_epipolar_lines(
        cv::Mat& left,
        cv::Mat& right,
        int spacing = 50);

private:
    cv::Mat map1L_, map2L_;
    cv::Mat map1R_, map2R_;
};
