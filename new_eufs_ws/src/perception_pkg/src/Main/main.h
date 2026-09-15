#pragma once

#include <string>

#include <opencv2/opencv.hpp>
#include <opencv2/dnn.hpp>

#include "Cone/cone.h"
#include "Keypoints/keypoints.h"

// ============================================================
// Detection thresholds
// ============================================================

inline constexpr float CONE_CONF_THRESHOLD = 0.6f;
inline constexpr float CONF_THRESHOLD       = 0.6f;
inline constexpr float NMS_THRESHOLD        = 0.45f;
inline constexpr float NUM_THRESHOLD        = 0.5f;
inline constexpr float KPT_THRESHOLD        = 0.5f;

// ============================================================
// YOLO input size
// ============================================================

inline const cv::Size MODEL_SIZE = cv::Size(640, 640);

// ============================================================
// Model paths
// ============================================================

// YOLOv8m cone detection model
inline const std::string cone_path =
    "/home/aadddyaa/projects/DV_26/new_eufs_ws/models/"
    "yolov8m_new/bestyolov8m_new.onnx";

// YOLOv8n-pose cone keypoint model
inline const std::string keypoints_path =
    "/home/aadddyaa/projects/DV_26/new_eufs_ws/models/"
    "yolov8n/weights/yolov8nposebest.onnx";

// ============================================================
// Offline testing images
// Not used by ROS2 perception_node
// ============================================================

inline const std::string img_path_left =
    "/home/aadddyaa/projects/DV_26/new_eufs_ws/"
    "runs/detect/predict-2/img2.jpg";

inline const std::string img_path_right =
    "/home/aadddyaa/projects/DV_26/new_eufs_ws/"
    "runs/detect/predict/img2.jpg";