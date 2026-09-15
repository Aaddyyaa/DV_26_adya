#include <eufs_msgs/msg/cone_array.hpp>
#include <geometry_msgs/msg/point.hpp>
#include "perception_node.h"

#include <functional>
#include <cmath>
#include <iostream>

PerceptionNode::PerceptionNode()
: Node("perception_node")
{
    // --------------------------------------------------
    // Load stereo rectification configuration
    // --------------------------------------------------

    if (!rectifier_.init_from_file(
        "/home/aaddyyyaa/projects/DV_26/new_eufs_ws/src/perception_pkg/config/extrinsics.txt"))
    {
        RCLCPP_FATAL(
            this->get_logger(),
            "Failed to load extrinsics.txt");

        rclcpp::shutdown();

        return;
    }

    // --------------------------------------------------
    // Load camera configuration
    // --------------------------------------------------

    if (!cam_.load_from_file(
        "/home/aaddyyyaa/projects/DV_26/new_eufs_ws/src/perception_pkg/config/extrinsics.txt"))
    {
        RCLCPP_FATAL(
            this->get_logger(),
            "Failed to load camera config");

        rclcpp::shutdown();

        return;
    }

    // --------------------------------------------------
    // Depth configuration
    // --------------------------------------------------

    const double min_cone_dist_m = 0.5;

    depth_cfg_.disp_max =
        static_cast<int>(
            std::ceil(
                cam_.fx *
                cam_.baseline /
                min_cone_dist_m)) + 30;

    // These are explicit model assumptions for sub-pixel stereo matching. The
    // actual NCC, keypoint visibility, disparity spread, and number of valid
    // matches determine the covariance for every individual cone.
    pixel_sigma_at_unit_quality_ = this->declare_parameter<double>(
        "pixel_sigma_at_unit_quality", pixel_sigma_at_unit_quality_);
    min_disparity_sigma_px_ = this->declare_parameter<double>(
        "min_disparity_sigma_px", min_disparity_sigma_px_);

    // --------------------------------------------------
    // Load neural networks
    // --------------------------------------------------

    cone_net_ =
        load_net_cone(
            cone_path,
            false);

    keypoints_net_ =
        load_net_keypoints(
            keypoints_path,
            true);

    RCLCPP_INFO(
        this->get_logger(),
        "Models loaded successfully");

    // --------------------------------------------------
    // Cone publisher
    // --------------------------------------------------

    cone_pub_ =
        this->create_publisher<eufs_msgs::msg::ConeArray>(
            "/perception/cones",
            10);
    cone_cov_pub_ =
        this->create_publisher<eufs_msgs::msg::ConeArrayWithCovariance>(
            "/perception/cones_with_covariance",
            10);

    // --------------------------------------------------
    // Left camera subscriber
    // --------------------------------------------------

    left_sub_ =
        this->create_subscription<sensor_msgs::msg::Image>(
            "/zed/left/image_rect_color",
            10,
            std::bind(
                &PerceptionNode::leftCallback,
                this,
                std::placeholders::_1));

    // --------------------------------------------------
    // Right camera subscriber
    // --------------------------------------------------

    right_sub_ =
        this->create_subscription<sensor_msgs::msg::Image>(
            "/zed/right/image_rect_color",
            10,
            std::bind(
                &PerceptionNode::rightCallback,
                this,
                std::placeholders::_1));

    RCLCPP_INFO(
        this->get_logger(),
        "Perception node started.");
}


// ======================================================
// LEFT IMAGE CALLBACK
// ======================================================

void PerceptionNode::leftCallback(
    const sensor_msgs::msg::Image::SharedPtr msg)
{
    try
    {
        left_img_ =
            cv_bridge::toCvCopy(msg, "bgr8")->image.clone();

        left_ready_ = true;

        RCLCPP_INFO(
            this->get_logger(),
            "Left image received");
    }
    catch (const cv_bridge::Exception &e)
    {
        RCLCPP_ERROR(
            this->get_logger(),
            "cv_bridge left error: %s",
            e.what());
    }

    if (left_ready_ && right_ready_)
    {
        processFrame();
    }
}


// ======================================================
// RIGHT IMAGE CALLBACK
// ======================================================

void PerceptionNode::rightCallback(
    const sensor_msgs::msg::Image::SharedPtr msg)
{
    try
    {
        right_img_ =
            cv_bridge::toCvCopy(msg, "bgr8")->image.clone();

        right_ready_ = true;

        RCLCPP_INFO(
            this->get_logger(),
            "Right image received");
    }
    catch (const cv_bridge::Exception &e)
    {
        RCLCPP_ERROR(
            this->get_logger(),
            "cv_bridge right error: %s",
            e.what());
    }

    if (left_ready_ && right_ready_)
    {
        processFrame();
    }
}


// ======================================================
// PROCESS FRAME
// ======================================================

void PerceptionNode::processFrame()
{
    cv::Mat rect_left;
    cv::Mat rect_right;

    // --------------------------------------------------
    // Stereo rectification
    // --------------------------------------------------

    rectifier_.rectify(
        left_img_,
        right_img_,
        rect_left,
        rect_right);

    // --------------------------------------------------
    // Cone detection
    // --------------------------------------------------

    std::vector<Detection> detections =
        detect(
            rect_left,
            cone_net_);

    std::cout
        << "\n========== AFTER detect() ==========\n";

    for (const auto &d : detections)
    {
        std::cout
            << "class_id=" << d.class_id
            << " label=" << d.label
            << " conf=" << d.confidence
            << std::endl;
    }

    std::cout
        << "====================================\n";

    // --------------------------------------------------
    // Visualization image
    // --------------------------------------------------

    cv::Mat vis_left =
        rect_left.clone();

    cv::Mat vis_right =
        rect_right.clone();

    draw_detections(
        vis_left,
        detections);

    cv::imwrite(
        "/tmp/after_draw.png",
        vis_left);

    std::cout
        << "[DEBUG] Saved /tmp/after_draw.png"
        << std::endl;

    // --------------------------------------------------
    // Keypoint detection
    // --------------------------------------------------

    std::vector<Keypoints> keypoints =
        detect_keypoints(
            detections,
            vis_left,
            keypoints_net_);

    // --------------------------------------------------
    // Stereo depth estimation
    // --------------------------------------------------

    std::vector<ConeDepth::ConeResult> depth_results =
        ConeDepth::estimate_depths(
            rect_left,
            rect_right,
            detections,
            keypoints,
            cam_,
            depth_cfg_);

    // --------------------------------------------------
    // Publish cones
    // --------------------------------------------------

    eufs_msgs::msg::ConeArray cone_msg;
    eufs_msgs::msg::ConeArrayWithCovariance cone_cov_msg;

    cone_msg.header.stamp =
        this->now();

    cone_msg.header.frame_id =
        "base_footprint";
    cone_cov_msg.header = cone_msg.header;

    temporalFilter(
        depth_results,
        cone_msg,
        cone_cov_msg);

    cone_pub_->publish(
        cone_msg);
    cone_cov_pub_->publish(
        cone_cov_msg);

    // --------------------------------------------------
    // Debug: total cones published
    // --------------------------------------------------

    {
        const int total =
            static_cast<int>(
                cone_msg.yellow_cones.size())
            +
            static_cast<int>(
                cone_msg.blue_cones.size())
            +
            static_cast<int>(
                cone_msg.orange_cones.size())
            +
            static_cast<int>(
                cone_msg.big_orange_cones.size())
            +
            static_cast<int>(
                cone_msg.unknown_color_cones.size());

        std::cout
            << "[GHOST-STAGE7-PUB] published="
            << total
            << " (y="
            << cone_msg.yellow_cones.size()
            << " b="
            << cone_msg.blue_cones.size()
            << " o="
            << cone_msg.orange_cones.size()
            << " B="
            << cone_msg.big_orange_cones.size()
            << " u="
            << cone_msg.unknown_color_cones.size()
            << ")"
            << std::endl;

        std::cout
            << "[GHOST-FRAME] ---- frame end ----"
            << std::endl;
    }

    // --------------------------------------------------
    // Visualization
    // --------------------------------------------------

    ConeDepth::draw_results(
        vis_left,
        vis_right,
        depth_results);

    cv::Mat epiL =
        vis_left.clone();

    cv::Mat epiR =
        vis_right.clone();

    Rectifier::draw_epipolar_lines(
        epiL,
        epiR,
        50);

    cv::Mat left_big =
        epiL;

    cv::Mat right_big =
        epiR;

    // --------------------------------------------------
    // OpenCV windows
    // --------------------------------------------------

    cv::namedWindow(
        "Left Detection",
        cv::WINDOW_NORMAL);

    cv::namedWindow(
        "Right Detection",
        cv::WINDOW_NORMAL);

    cv::resizeWindow(
        "Left Detection",
        900,
        500);

    cv::resizeWindow(
        "Right Detection",
        900,
        500);

    std::cout
        << "[WINDOW] Before resize"
        << std::endl;

    cv::resize(
        vis_left,
        left_big,
        cv::Size(),
        1.5,
        1.5);

    cv::resize(
        vis_right,
        right_big,
        cv::Size(),
        1.5,
        1.5);

    std::cout
        << "[WINDOW] Before imshow"
        << std::endl;

    cv::imshow(
        "Left Detection",
        left_big);

    cv::imshow(
        "Right Detection",
        right_big);

    std::cout
        << "[WINDOW] Before waitKey"
        << std::endl;

    cv::waitKey(1);

    std::cout
        << "[WINDOW] After waitKey"
        << std::endl;

    // --------------------------------------------------
    // Reset synchronization flags
    // --------------------------------------------------

    left_ready_ = false;
    right_ready_ = false;
}


// ======================================================
// TEMPORAL FILTER
// ======================================================

void PerceptionNode::temporalFilter(
    const std::vector<ConeDepth::ConeResult>& input,
    eufs_msgs::msg::ConeArray& output,
    eufs_msgs::msg::ConeArrayWithCovariance& covariance_output)
{
    std::vector<TemporalCone> current;

    // --------------------------------------------------
    // Convert valid depth results to temporal cones
    // --------------------------------------------------

    for (const auto &c : input)
    {
        if (!c.valid)
            continue;

        TemporalCone tc;

        tc.point.x =
            c.position.x;

        tc.point.y =
            c.position.y;

        tc.point.z =
            0.0;

        if (!measurementCovariance(c, tc.covariance))
        {
            RCLCPP_WARN(
                this->get_logger(),
                "Rejecting cone with invalid stereo covariance");
            continue;
        }

        tc.class_id =
            c.class_id;

        current.push_back(
            tc);
    }

    // --------------------------------------------------
    // Debug: cones entering temporal filter
    // --------------------------------------------------

    std::cout
        << "[GHOST-STAGE5-IN] into_filter="
        << current.size()
        << std::endl;

    // --------------------------------------------------
    // Add current frame to history
    // --------------------------------------------------

    history_.push_back(
        current);

    if (history_.size() > HISTORY_SIZE)
    {
        history_.pop_front();
    }

    int published_count = 0;

    // --------------------------------------------------
    // Temporal matching
    // --------------------------------------------------

    for (const auto &cone : current)
    {
        int count = 0;

        for (const auto &frame : history_)
        {
            for (const auto &old : frame)
            {
                if (old.class_id != cone.class_id)
                    continue;

                double dx =
                    cone.point.x -
                    old.point.x;

                double dy =
                    cone.point.y -
                    old.point.y;

                if (std::hypot(dx, dy) <
                    MATCH_DISTANCE)
                {
                    count++;
                    break;
                }
            }
        }

        // ------------------------------------------------
        // Require minimum observations
        // ------------------------------------------------

        if (count < MIN_OBSERVATIONS)
            continue;

        // ------------------------------------------------
        // Publish according to cone class
        // ------------------------------------------------

        switch (cone.class_id)
        {
            case 0:
                output.yellow_cones.push_back(
                    cone.point);
                {
                    eufs_msgs::msg::ConeWithCovariance out;
                    out.point = cone.point;
                    out.covariance = cone.covariance;
                    covariance_output.yellow_cones.push_back(out);
                }
                break;

            case 1:
                output.blue_cones.push_back(
                    cone.point);
                {
                    eufs_msgs::msg::ConeWithCovariance out;
                    out.point = cone.point;
                    out.covariance = cone.covariance;
                    covariance_output.blue_cones.push_back(out);
                }
                break;

            case 2:
                output.orange_cones.push_back(
                    cone.point);
                {
                    eufs_msgs::msg::ConeWithCovariance out;
                    out.point = cone.point;
                    out.covariance = cone.covariance;
                    covariance_output.orange_cones.push_back(out);
                }
                break;

            case 3:
                output.big_orange_cones.push_back(
                    cone.point);
                {
                    eufs_msgs::msg::ConeWithCovariance out;
                    out.point = cone.point;
                    out.covariance = cone.covariance;
                    covariance_output.big_orange_cones.push_back(out);
                }
                break;

            default:
                output.unknown_color_cones.push_back(
                    cone.point);
                {
                    eufs_msgs::msg::ConeWithCovariance out;
                    out.point = cone.point;
                    out.covariance = cone.covariance;
                    covariance_output.unknown_color_cones.push_back(out);
                }
                break;
        }

        ++published_count;
    }

    // --------------------------------------------------
    // Debug: cones leaving temporal filter
    // --------------------------------------------------

    std::cout
        << "[GHOST-STAGE6-OUT] left_filter="
        << published_count
        << std::endl;
}

bool PerceptionNode::measurementCovariance(
    const ConeDepth::ConeResult& result,
    std::array<double, 4>& covariance) const
{
    // The output point preserves the baseline's X/Y coordinate convention.
    // For X=(u-cx)Z/fx and Y=(v-cy)Z/fy, propagate pixel/disparity error
    // through the stereo model. No covariance is published when the evidence
    // retained by the depth pipeline is incomplete or non-finite.
    double weighted_disparity = 0.0;
    double weighted_disparity_sq = 0.0;
    double weighted_ncc = 0.0;
    double weighted_visibility = 0.0;
    double weight_sum = 0.0;
    int valid_matches = 0;

    for (const auto& keypoint : result.kp_res)
    {
        if (!keypoint.match.valid || !keypoint.pt3d.valid ||
            !std::isfinite(keypoint.match.disparity) ||
            !std::isfinite(keypoint.match.ncc_score))
        {
            continue;
        }

        const double visibility = std::max(
            0.0, static_cast<double>(keypoint.left_kp.visibility));
        const double ncc = std::max(0.0, keypoint.match.ncc_score);
        const double weight = ncc * visibility;
        if (weight <= 0.0)
        {
            continue;
        }

        weighted_disparity += weight * keypoint.match.disparity;
        weighted_disparity_sq += weight * keypoint.match.disparity *
            keypoint.match.disparity;
        weighted_ncc += weight * ncc;
        weighted_visibility += weight * visibility;
        weight_sum += weight;
        ++valid_matches;
    }

    if (valid_matches == 0 || weight_sum <= 0.0 ||
        !std::isfinite(result.position.x) ||
        !std::isfinite(result.position.y) ||
        !std::isfinite(result.position.z) || result.position.z <= 0.0 ||
        cam_.fx <= 0.0 || cam_.fy <= 0.0)
    {
        return false;
    }

    const double disparity = weighted_disparity / weight_sum;
    const double disparity_variance = std::max(
        0.0,
        weighted_disparity_sq / weight_sum - disparity * disparity);
    const double quality = std::max(
        1e-6,
        (weighted_ncc / weight_sum) *
            (weighted_visibility / weight_sum));
    const double sigma_d = std::max(
        min_disparity_sigma_px_, std::sqrt(disparity_variance)) /
        std::sqrt(quality * valid_matches);
    const double sigma_u = pixel_sigma_at_unit_quality_ /
        std::sqrt(quality * valid_matches);
    const double sigma_v = sigma_u;

    if (!std::isfinite(disparity) || disparity <= 0.0 ||
        !std::isfinite(sigma_d) || !std::isfinite(sigma_u) ||
        sigma_d <= 0.0 || sigma_u <= 0.0)
    {
        return false;
    }

    const double x = result.position.x;
    const double y = result.position.y;
    const double z = result.position.z;
    const double inv_disparity = 1.0 / disparity;
    const double disparity_variance_model = sigma_d * sigma_d;
    const double x_variance =
        std::pow(z / cam_.fx, 2) * sigma_u * sigma_u +
        std::pow(x * inv_disparity, 2) * disparity_variance_model;
    const double y_variance =
        std::pow(z / cam_.fy, 2) * sigma_v * sigma_v +
        std::pow(y * inv_disparity, 2) * disparity_variance_model;
    const double xy_covariance =
        x * y * inv_disparity * inv_disparity * disparity_variance_model;

    covariance = {x_variance, xy_covariance, xy_covariance, y_variance};
    return std::all_of(
        covariance.begin(), covariance.end(),
        [](double value) { return std::isfinite(value); }) &&
        covariance[0] > 0.0 && covariance[3] > 0.0 &&
        covariance[0] * covariance[3] >= covariance[1] * covariance[2];
}
