#include "rectification.h"
#include "calibration.h"

bool Rectifier::init_from_file(const std::string& path)
{
    if (!load_extrinsics(path))
        return false;

    compute_rectification_maps(
        map1L_, map2L_,
        map1R_, map2R_);

    return true;
}

void Rectifier::rectify(
    const cv::Mat& left,
    const cv::Mat& right,
    cv::Mat& left_rect,
    cv::Mat& right_rect)
{
    cv::remap(
        left,
        left_rect,
        map1L_,
        map2L_,
        cv::INTER_LINEAR);

    cv::remap(
        right,
        right_rect,
        map1R_,
        map2R_,
        cv::INTER_LINEAR);
}

void Rectifier::draw_epipolar_lines(
    cv::Mat& left,
    cv::Mat& right,
    int spacing)
{
    for (int y = 0; y < left.rows; y += spacing)
    {
        cv::line(
            left,
            cv::Point(0, y),
            cv::Point(left.cols, y),
            cv::Scalar(0, 255, 0),
            1);

        cv::line(
            right,
            cv::Point(0, y),
            cv::Point(right.cols, y),
            cv::Scalar(0, 255, 0),
            1);
    }
}
