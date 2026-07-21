#include <eufs_msgs/msg/cone_array.hpp>
#include <geometry_msgs/msg/point.hpp>
#include "perception_node.h"

#include <functional>

PerceptionNode::PerceptionNode()
: Node("perception_node")
{

if (!rectifier_.init_from_file(
    "/home/alr/Desktop/new_perception/new_eufs_ws/src/perception_pkg/config/extrinsics.txt"))
{
    RCLCPP_FATAL(
        this->get_logger(),
        "Failed to load extrinsics.txt");

    rclcpp::shutdown();

    return;
}

if (!cam_.load_from_file(
    "/home/alr/Desktop/new_perception/new_eufs_ws/src/perception_pkg/config/extrinsics.txt"))
{
    RCLCPP_FATAL(
        this->get_logger(),
        "Failed to load camera config");

    rclcpp::shutdown();

    return;
}

const double min_cone_dist_m = 0.5;

depth_cfg_.disp_max =
    static_cast<int>(
        std::ceil(
            cam_.fx *
            cam_.baseline /
            min_cone_dist_m)) + 30;

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

cone_pub_ =
    this->create_publisher<eufs_msgs::msg::ConeArray>(
        "/perception/cones",
        10);



    left_sub_ =
        this->create_subscription<sensor_msgs::msg::Image>(
            "/zed/left/image_rect_color",
            10,
            std::bind(
                &PerceptionNode::leftCallback,
                this,
                std::placeholders::_1));

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

void PerceptionNode::processFrame()
{
    cv::Mat rect_left;
    cv::Mat rect_right;

    rectifier_.rectify(
        left_img_,
        right_img_,
        rect_left,
        rect_right);

    // --------------------------
    // Cone detection
    // --------------------------

    std::vector<Detection> detections =
        detect(rect_left, cone_net_);

std::cout << "\n========== AFTER detect() ==========\n";

for (const auto &d : detections)
{
    std::cout
        << "class_id=" << d.class_id
        << " label=" << d.label
        << " conf=" << d.confidence
        << std::endl;
}

std::cout << "====================================\n";


    cv::Mat vis_left = rect_left.clone();
    cv::Mat vis_right = rect_right.clone();

    draw_detections(vis_left, detections);

    cv::imwrite("/tmp/after_draw.png", vis_left);
    std::cout << "[DEBUG] Saved /tmp/after_draw.png" << std::endl;
    // --------------------------
    // Keypoint detection
    // --------------------------

std::vector<Keypoints> keypoints =
    detect_keypoints(
        detections,
        vis_left,
        keypoints_net_);

// --------------------------
// Stereo depth estimation
// --------------------------

std::vector<ConeDepth::ConeResult> depth_results =
    ConeDepth::estimate_depths(
        rect_left,
        rect_right,
        detections,
        keypoints,
        cam_,
        depth_cfg_);

// --------------------------
// Publish cones
// --------------------------

eufs_msgs::msg::ConeArray cone_msg;

cone_msg.header.stamp = this->now();
cone_msg.header.frame_id = "base_footprint";

temporalFilter(depth_results, cone_msg);

cone_pub_->publish(cone_msg);

// [GHOST-STAGE7-PUB] total cones published this frame:
{
    const int total = static_cast<int>(cone_msg.yellow_cones.size())
                    + static_cast<int>(cone_msg.blue_cones.size())
                    + static_cast<int>(cone_msg.orange_cones.size())
                    + static_cast<int>(cone_msg.big_orange_cones.size())
                    + static_cast<int>(cone_msg.unknown_color_cones.size());
    std::cout << "[GHOST-STAGE7-PUB] published=" << total
              << " (y=" << cone_msg.yellow_cones.size()
              << " b=" << cone_msg.blue_cones.size()
              << " o=" << cone_msg.orange_cones.size()
              << " B=" << cone_msg.big_orange_cones.size()
              << " u=" << cone_msg.unknown_color_cones.size() << ")"
              << std::endl;
    std::cout << "[GHOST-FRAME] ---- frame end ----" << std::endl;
}

// --------------------------
// Visualization
// --------------------------

ConeDepth::draw_results(
    vis_left,
    vis_right,
    depth_results);

cv::Mat epiL = vis_left.clone();
cv::Mat epiR = vis_right.clone();

Rectifier::draw_epipolar_lines(epiL, epiR, 50);

cv::Mat left_big = epiL;
cv::Mat right_big = epiR;

cv::namedWindow("Left Detection", cv::WINDOW_NORMAL);
cv::namedWindow("Right Detection", cv::WINDOW_NORMAL);

cv::resizeWindow("Left Detection", 900, 500);
cv::resizeWindow("Right Detection", 900, 500);



std::cout << "[WINDOW] Before resize" << std::endl;

cv::resize(vis_left, left_big, cv::Size(), 1.5, 1.5);
cv::resize(vis_right, right_big, cv::Size(), 1.5, 1.5);

std::cout << "[WINDOW] Before imshow" << std::endl;

cv::imshow("Left Detection", left_big);
cv::imshow("Right Detection", right_big);

std::cout << "[WINDOW] Before waitKey" << std::endl;

cv::waitKey(1);

std::cout << "[WINDOW] After waitKey" << std::endl;


    left_ready_ = false;
    right_ready_ = false;
}



void PerceptionNode::temporalFilter(
    const std::vector<ConeDepth::ConeResult>& input,
    eufs_msgs::msg::ConeArray& output)
{
    std::vector<TemporalCone> current;

    for (const auto &c : input)
    {
        if (!c.valid)
            continue;

        TemporalCone tc;

        tc.point.x = c.position.x;
        tc.point.y = c.position.y;
        tc.point.z = 0.0;

        tc.class_id = c.class_id;

        current.push_back(tc);
    }

    // [GHOST-STAGE5-IN] valid cones entering the temporal filter (== valid depth):
    std::cout << "[GHOST-STAGE5-IN] into_filter=" << current.size() << std::endl;

    history_.push_back(current);

    if (history_.size() > HISTORY_SIZE)
        history_.pop_front();

    int published_count = 0;

    for (const auto &cone : current)
    {
        int count = 0;

        for (const auto &frame : history_)
        {
            for (const auto &old : frame)
            {
                if (old.class_id != cone.class_id)
                    continue;

                double dx = cone.point.x - old.point.x;
                double dy = cone.point.y - old.point.y;

                if (std::hypot(dx, dy) < MATCH_DISTANCE)
                {
                    count++;
                    break;
                }
            }
        }

        if (count < MIN_OBSERVATIONS)
            continue;

        switch (cone.class_id)
        {
            case 0:
                output.yellow_cones.push_back(cone.point);
                break;

            case 1:
                output.blue_cones.push_back(cone.point);
                break;

            case 2:
                output.orange_cones.push_back(cone.point);
                break;

            case 3:
                output.big_orange_cones.push_back(cone.point);
                break;

            default:
                output.unknown_color_cones.push_back(cone.point);
                break;
        }
        ++published_count;
    }

    // [GHOST-STAGE6-OUT] cones leaving the temporal filter (gated + published):
    std::cout << "[GHOST-STAGE6-OUT] left_filter=" << published_count << std::endl;
}
