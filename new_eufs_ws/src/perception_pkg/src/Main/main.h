#pragma once
#include <opencv2/opencv.hpp>
#include <opencv2/dnn.hpp>
#include "Cone/cone.h"
#include "Keypoints/keypoints.h"

inline constexpr float    CONE_CONF_THRESHOLD = 0.6f;
inline constexpr float    CONF_THRESHOLD      = 0.6f;
inline constexpr float    NMS_THRESHOLD       = 0.45f;
inline constexpr float    NUM_THRESHOLD       = 0.5f;
inline constexpr float    KPT_THRESHOLD       = 0.5f;
inline const cv::Size     MODEL_SIZE          = cv::Size(640, 640);

inline const std::string cone_path =
"/home/alr/Desktop/new_perception/new_eufs_ws/src/perception_pkg/models/bestyolov8m_new.onnx";

inline const std::string keypoints_path =
"/home/alr/Desktop/new_perception/new_eufs_ws/src/perception_pkg/models/yolov8nposebest.onnx";

// Offline testing images (not used by ROS2 perception_node)
inline const std::string img_path_left =
"/home/alr/Desktop/new_perception/Perception/Images/img2.jpg";

inline const std::string img_path_right =
"/home/alr/Desktop/new_perception/Perception/Images/img2.jpg";
