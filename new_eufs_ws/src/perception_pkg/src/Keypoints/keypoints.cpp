
#include <opencv2/opencv.hpp>
#include <fstream>
#include <iostream>
#include <vector>
#include <string>
#include <algorithm>


#include "keypoints.h"
#include "Cone/cone.h"
#include "Main/main.h"


const std::vector<cv::Scalar> KEYPOINT_COLORS = {
    cv::Scalar(0, 255, 255),   cv::Scalar(0, 255, 255),   // top    - Yellow
    cv::Scalar(0, 165, 255),   cv::Scalar(0, 165, 255),   // upper  - Orange
    cv::Scalar(0, 0, 255),     cv::Scalar(0, 0, 255),     // lower  - Red
    cv::Scalar(255, 0, 0),     cv::Scalar(255, 0, 0),     // bottom - Blue
};



cv::dnn::Net load_net_keypoints(const std::string &path, bool use_cuda){

    std::cout << "Loading ONNX: " << path << std::endl;

    cv::dnn::Net net = cv::dnn::readNetFromONNX(path);


    if (net.empty()){
        std::cerr << "[ERROR] Failed to load model: " << path << "\n";
        exit(EXIT_FAILURE);
    }

    if(use_cuda){
        net.setPreferableBackend(cv::dnn::DNN_BACKEND_CUDA);
        net.setPreferableTarget(cv::dnn::DNN_TARGET_CUDA);
    }
    else{
        net.setPreferableBackend(cv::dnn::DNN_BACKEND_OPENCV);
        net.setPreferableTarget(cv::dnn::DNN_TARGET_CPU);
    }
    return net;
}

cv::Scalar get_keypoint_color(int keypoint_index) {
    if (keypoint_index < 0 || keypoint_index >= (int)KEYPOINT_COLORS.size())
        return cv::Scalar(255, 255, 255);
    return KEYPOINT_COLORS[keypoint_index];
}

void draw_keypoints(cv::Mat& img, const Keypoints& kpts){
    for (int k = 0; k < 8; k++) {
        if (kpts.p[k].visibility < KPT_THRESHOLD) continue;

        cv::Scalar color = get_keypoint_color(k);
        cv::circle(img, kpts.p[k].point, 5, color,              -1, cv::LINE_AA);
        cv::circle(img, kpts.p[k].point, 6, cv::Scalar(0, 0, 0), 1, cv::LINE_AA);
    }
}

std::vector<Keypoints> detect_keypoints(
    std::vector<Detection>& detections,
    cv::Mat& img,
    cv::dnn::Net& net)
{
    std::vector<Keypoints> results;

    std::cout << "[KEYPOINT] detections received = "
              << detections.size() << std::endl;
for (auto& detection : detections) {

    std::cout
        << "[INPUT BOX] "
        << "x=" << detection.box.x
        << " y=" << detection.box.y
        << " w=" << detection.box.width
        << " h=" << detection.box.height
        << std::endl;

std::cout << "\n========== ROI DEBUG ==========\n";

std::cout
    << "img = "
    << img.cols << "x" << img.rows
    << std::endl;

std::cout
    << "det = "
    << detection.box.x << " "
    << detection.box.y << " "
    << detection.box.width << "x"
    << detection.box.height
    << std::endl;   

 cv::Rect roi_rect =
        detection.box &
        cv::Rect(0, 0, img.cols, img.rows);

std::cout
    << "roi = "
    << roi_rect.x << " "
    << roi_rect.y << " "
    << roi_rect.width << "x"
    << roi_rect.height
    << std::endl;

std::cout << "===============================\n";

std::cout << "[ROI] "
          << roi_rect.x << " "
          << roi_rect.y << " "
          << roi_rect.width << "x"
          << roi_rect.height << std::endl;

if (roi_rect.width <= 1 || roi_rect.height <= 1) {
    std::cout << "[ROI SKIPPED]" << std::endl;
    continue;
}

std::cout << "[ROI VALID]" << std::endl;

cv::Mat roi = img(roi_rect).clone();
        cv::Mat blob;
        cv::dnn::blobFromImage(
            roi,
            blob,
            1.0 / 255.0,
            MODEL_SIZE,
            cv::Scalar(),
            true,
            false);

        net.setInput(blob);

std::cout << "[DEBUG] Before keypoint forward" << std::endl;

std::vector<cv::Mat> outputs;
net.forward(outputs, net.getUnconnectedOutLayersNames());

std::cout << "[DEBUG] After keypoint forward" << std::endl;



        int rows = outputs[0].size[2];
        int dimensions = outputs[0].size[1];

        cv::Mat out = outputs[0].reshape(1, dimensions);
        cv::transpose(out, out);

        float* data = (float*)out.data;

float x_scale = (float)roi_rect.width  / MODEL_SIZE.width;
float y_scale = (float)roi_rect.height / MODEL_SIZE.height;


std::cout
    << "[BOX] "
    << "x=" << detection.box.x
    << " y=" << detection.box.y
    << " w=" << detection.box.width
    << " h=" << detection.box.height
    << std::endl;

        Keypoints best;
        best.confidence = -1.0f;

        for (int i = 0; i < rows; i++, data += dimensions) {

            float obj_conf = data[4];

            if (obj_conf < CONF_THRESHOLD)
                continue;

            if (obj_conf <= best.confidence)
                continue;

            best.confidence = obj_conf;

std::cout << "\n========== KEYPOINT RAW ==========\n";

for (int j = 0; j < dimensions; j++) {
    std::cout << "[" << j << "]=" << data[j] << " ";
}

std::cout << "\n==================================\n";

            for (int k = 0; k < 8; k++) {
                int base = 5 + k * 3;

                best.p[k].point.x =
                    data[base + 0] * x_scale + roi_rect.x;

                best.p[k].point.y =
                    data[base + 1] * y_scale + roi_rect.y;

                best.p[k].visibility =
                    data[base + 2];
            }
        }

if (best.confidence > 0.0f) {
    draw_keypoints(img, best);
}

std::cout << "[PUSH] confidence="
          << best.confidence << std::endl;

results.push_back(best);
    }

std::cout << "[RETURN] results.size()="
          << results.size() << std::endl;

// [GHOST-STAGE3-KEYPOINTS] valid keypoint detections (confidence>0, i.e. something matched):
{
    int valid_kp = 0;
    for (const auto& r : results) if (r.confidence > 0.0f) ++valid_kp;
    std::cout << "[GHOST-STAGE3-KEYPOINTS] valid_keypoints=" << valid_kp
              << " / " << results.size() << std::endl;
}

return results;
}
