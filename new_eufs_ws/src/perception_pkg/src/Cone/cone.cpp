//  Pre-defined headers
#include <opencv2/opencv.hpp>
#include <fstream>
#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

//  User-defined headers
#include "cone.h"
#include "Main/main.h"
#include "Keypoints/keypoints.h"


cv::dnn::Net load_net_cone(const std::string& path, bool use_cuda){
    std::cout << "[CONE] Loading ONNX: " << path << std::endl;
    std::cout << "\n=========================\n";
std::cout << "Loading cone model:\n";
std::cout << path << std::endl;
std::cout << "=========================\n";
    cv::dnn::Net net = cv::dnn::readNetFromONNX(path);
    if (net.empty()) {
        std::cerr << "[ERROR] Failed to load model: " << path << "\n";
        exit(EXIT_FAILURE);
    }
    if (use_cuda) {
        net.setPreferableBackend(cv::dnn::DNN_BACKEND_CUDA);
        net.setPreferableTarget(cv::dnn::DNN_TARGET_CUDA);
        std::cout << "[INFO] Backend: CUDA\n";
    } else {
        net.setPreferableBackend(cv::dnn::DNN_BACKEND_OPENCV);
        net.setPreferableTarget(cv::dnn::DNN_TARGET_CPU);
        std::cout << "[INFO] Backend: CPU\n";
    }
    return net;
}

// ─── Detect ───────────────────────────────────────────────────────────────────
// Ultralytics YOLOv8 Python export → ONNX output shape: [1, NUM_FEATURES, 8400]
// Layout: [1, 8, 8400] for 4-class model
// After reshape(1, NUM_FEATURES) + transpose → [8400, NUM_FEATURES]
// Each row: [cx, cy, w, h, score_0, score_1, score_2, score_3]
// Coordinates are in model pixel space: 0 → 640

std::vector<Detection> detect(const cv::Mat& img, cv::dnn::Net& net){

    // ── Preprocess ────────────────────────────────────────────────────────────
    cv::Mat blob;

cv::dnn::blobFromImage(
    img,
    blob,
    1.0/255.0,
    MODEL_SIZE,
    cv::Scalar(),
    true,      // <-- swapRB
    false
);

std::cout << "[DEBUG] Input image: "
          << img.cols << "x" << img.rows << std::endl;

std::cout << "[DEBUG] Blob: "
          << blob.size[0] << " "
          << blob.size[1] << " "
          << blob.size[2] << " "
          << blob.size[3] << std::endl;



net.setInput(blob);

std::cout << "[DEBUG] Before cone forward" << std::endl;

std::vector<cv::String> layer_names = net.getLayerNames();
std::cout << "[DEBUG] Total layers: " << layer_names.size() << std::endl;

std::vector<cv::String> out_names = net.getUnconnectedOutLayersNames();

std::cout << "[DEBUG] Output layers: " << out_names.size() << std::endl;


for (const auto &n : out_names)
{
    std::cout << "[DEBUG] Output: " << n << std::endl;
}

std::vector<cv::Mat> outputs;
net.forward(outputs, out_names);

cv::Mat raw = outputs[0];

std::cout << "[DEBUG] raw.dims = " << raw.dims << std::endl;

for (int i = 0; i < raw.dims; i++)
{
    std::cout << "[DEBUG] raw.size[" << i << "] = "
              << raw.size[i] << std::endl;
}

// ── Forward pass ──────────────────────────────────────────

if (raw.empty()) {
    std::cerr << "[ERROR] Empty network output\n";
    return {};
}

// ── Validate tensor shape ─────────────────────────────────
if (raw.dims != 3) {
    std::cerr << "[ERROR] Expected 3D tensor, got "
              << raw.dims << "D\n";
    return {};
}


    const int d1 = raw.size[1];   // should be NUM_FEATURES (8)
    const int d2 = raw.size[2];   // should be 8400

    std::cout << "\n\n\n\n\n\n" << d1 << d2 << std::endl;

    std::cout << "[DEBUG] Tensor: 1 x " << d1 << " x " << d2 << "\n";

    if (d1 != NUM_FEATURES) {
        std::cerr << "[ERROR] Expected dim[1]=" << NUM_FEATURES
                  << " but got " << d1
                  << ". Check that config has exactly " << NUM_CLASSES
                  << " classes and matches training order.\n";
        return {};
    }

    const int num_anchors = d2;   // 8400

    // ── Reshape [1, 8, 8400] → [8, 8400] → transpose → [8400, 8] ─────────────
cv::Mat out = raw.reshape(1, raw.size[1]);

if (out.rows != num_anchors){

    cv::transpose(out, out);
}
    // ── Scale from model space (0–640) to original image space ────────────────
    const float x_scale = (float)img.cols / MODEL_SIZE.width;
    const float y_scale = (float)img.rows / MODEL_SIZE.height;

std::cout << "\n========== RAW ==========\n";
std::cout << "raw.dims = " << raw.dims << std::endl;
for (int i = 0; i < raw.dims; i++)
    std::cout << "raw.size[" << i << "] = "
              << raw.size[i] << std::endl;

std::cout << "\n========== OUT ==========\n";
std::cout << "out.rows = " << out.rows << std::endl;
std::cout << "out.cols = " << out.cols << std::endl;

for (int i = 0; i < 5; i++)
{
    float* r = out.ptr<float>(i);

    std::cout
        << "[" << i << "] "
        << r[0] << " "
        << r[1] << " "
        << r[2] << " "
        << r[3] << " "
        << r[4] << " "
        << r[5] << " "
        << r[6] << " "
        << r[7] << " "
        << r[8]
        << std::endl;
}

    std::vector<int>      class_ids;
    std::vector<float>    confidences;
    std::vector<cv::Rect> boxes;

    for (int i = 0; i < num_anchors; i++) {
        float* row = (float*)out.ptr(i);  // [cx, cy, w, h, s0, s1, s2, s3]

        // Find best class score
        float best_score = -1.0f;
        int   best_cls   = 0;
        for (int c = 0; c < NUM_CLASSES; c++) {
            if (row[4 + c] > best_score) {
                best_score = row[4 + c];
                best_cls   = c;
            }
        }


        if (best_score < CONE_CONF_THRESHOLD)
            continue;

        std::cout << "\n========== DETECTION ==========\n";
        std::cout << "Anchor: " << i << std::endl;

        for (int c = 0; c < NUM_CLASSES; c++)
        {
            std::cout
            << "  "
            << CLASS_NAMES[c]
            << " = "
            << row[4 + c]
            << std::endl;
       }

       std::cout
           << "Predicted class = "
           << best_cls
           << " ("
           << CLASS_NAMES[best_cls]
           << ")"
           << std::endl;

       std::cout << "===============================\n";

float cx = row[0];
float cy = row[1];
float bw = row[2];
float bh = row[3];

int left   = (int)((cx - bw * 0.5f) * x_scale);
int top    = (int)((cy - bh * 0.5f) * y_scale);
int width  = (int)(bw * x_scale);
int height = (int)(bh * y_scale);

std::cout
    << "[BOX] "
    << "cx=" << cx
    << " cy=" << cy
    << " bw=" << bw
    << " bh=" << bh
    << " --> "
    << "left=" << left
    << " top=" << top
    << " width=" << width
    << " height=" << height
    << std::endl;
        // Clamp to image bounds
        left   = std::max(0, left);
        top    = std::max(0, top);
        width  = std::min(width,  img.cols - left);
        height = std::min(height, img.rows - top);

        // ── Stage 1/2 fix: reject tiny / degenerate boxes (edge-split half-boxes ──
        //   that survive the confidence gate but aren't whole cones).  A real cone
        //   at driving distance is at least ~6 px on each side and ~120 px² area.
        constexpr int    MIN_BOX_DIM  = 6;
        constexpr int    MIN_BOX_AREA = 120;
        if (width < MIN_BOX_DIM || height < MIN_BOX_DIM) continue;
        if (width * height < MIN_BOX_AREA) continue;

        if (width < 2 || height < 2) continue;


std::cout
    << "[PRE NMS] "
    << "class=" << best_cls
    << " label=" << CLASS_NAMES[best_cls]
    << " box=("
    << left << ", "
    << top << ", "
    << width << ", "
    << height << ")"
    << std::endl;

class_ids.push_back(best_cls);
confidences.push_back(best_score);
boxes.push_back(cv::Rect(left, top, width, height));

    }

    std::cout << "[INFO] Boxes before NMS: " << boxes.size() << "\n";

    // ─── STAGE INSTRUMENTATION (no logic change) ───────────────────────────────
    // [STAGE1] YOLO raw conf-passed detections (pre-NMS):
    std::cout << "[GHOST-STAGE1-RAW] conf_passed=" << boxes.size() << std::endl;

    // ── NMS ───────────────────────────────────────────────────────────────────
    // Stage 1/2 fix: lower the NMS IoU threshold from 0.45 → 0.40 so that more
    //   overlapping detections of the SAME physical cone are suppressed.  Lower
    //   threshold = more aggressive suppression.  (NMSBoxes' nms_threshold arg:
    //   keep box B if IoU(A,B) <= nms_threshold, else drop B.)
    std::vector<int> nms_idx;
    cv::dnn::NMSBoxes(boxes, confidences, CONE_CONF_THRESHOLD, 0.40f, nms_idx);

    std::vector<Detection> detections;
    detections.reserve(nms_idx.size());


std::cout << "\n===== NMS INDEXES =====\n";
for (int idx : nms_idx)
    std::cout << idx << " ";
std::cout << "\n=======================\n";

for (int idx : nms_idx)
{
    Detection d;
    d.class_id   = class_ids[idx];
    d.confidence = confidences[idx];
    d.box        = boxes[idx];
    d.label      = CLASS_NAMES[d.class_id];

    std::cout
        << "\n========== FINAL DETECTION ==========\n"
        << "label = " << d.label << "\n"
        << "class = " << d.class_id << "\n"
        << "conf = " << d.confidence << "\n"
        << "box = ("
        << d.box.x << ", "
        << d.box.y << ", "
        << d.box.width << ", "
        << d.box.height << ")"
        << "\n=====================================\n";

    detections.push_back(d);
}

    // [GHOST-STAGE2a-NMS] detections immediately after NMS:
    std::cout << "[GHOST-STAGE2a-NMS] after_nms_raw=" << detections.size() << std::endl;

    // ── Stage 1/2 fix: same-class center-distance dedup ────────────────────────
    //   NMS above is GLOBAL (suppresses across all classes). Two boxes on the
    //   SAME cone can survive if their IoU is just below the NMS gate (e.g.
    //   offset centres) even though both centres lie on one physical cone.
    //   Here we merge, per class, any two detections whose CENTRE distance is
    //   below a fraction of the average box diagonal, keeping the higher-conf one.
    //   This is the direct one-cone->one-box pass the instrumentation
    //   (after_nms=12 for far fewer real cones) asked for.
    {
        constexpr float DEDUP_FRAC = 0.45f; // centre dist < 0.45 * avg diagonal → same cone

        // Greedy: sort indices by confidence desc, mark suppressed.
        std::vector<int> order(detections.size());
        for (size_t i = 0; i < detections.size(); ++i) order[i] = (int)i;
        std::stable_sort(order.begin(), order.end(),
            [&](int a, int b){ return detections[a].confidence > detections[b].confidence; });

        std::vector<char> suppressed(detections.size(), 0);
        std::vector<Detection> deduped;
        deduped.reserve(detections.size());

        for (size_t oi = 0; oi < order.size(); ++oi)
        {
            const int i = order[oi];
            if (suppressed[i]) continue;

            const Detection& di = detections[i];
            const float di_diag =
                std::hypot((float)di.box.width, (float)di.box.height);

            for (size_t oj = oi + 1; oj < order.size(); ++oj)
            {
                const int j = order[oj];
                if (suppressed[j]) continue;

                // Only merge detections of the same class.
                if (detections[j].class_id != di.class_id) continue;

                const Detection& dj = detections[j];
                const float dj_diag =
                    std::hypot((float)dj.box.width, (float)dj.box.height);
                const float avg_diag = 0.5f * (di_diag + dj_diag);
                if (avg_diag <= 0.0f) continue;

                const float cx_i = di.box.x + di.box.width  * 0.5f;
                const float cy_i = di.box.y + di.box.height * 0.5f;
                const float cx_j = dj.box.x + dj.box.width  * 0.5f;
                const float cy_j = dj.box.y + dj.box.height * 0.5f;
                const float centre_dist = std::hypot(cx_i - cx_j, cy_i - cy_j);

                if (centre_dist < DEDUP_FRAC * avg_diag)
                    suppressed[j] = 1;           // drop the lower-conf duplicate
            }
            deduped.push_back(di);
        }
        detections = std::move(deduped);
    }

    // [GHOST-STAGE2b-DEDUP] detections after same-class centre dedup:
    std::cout << "[GHOST-STAGE2b-DEDUP] after_dedup=" << detections.size() << std::endl;

std::cout << "[INFO] Detections after NMS: "
          << detections.size()
          << "\n";

// [GHOST-STAGE2-NMS] detections after NMS:
std::cout << "[GHOST-STAGE2-NMS] after_nms=" << detections.size() << std::endl;

return detections;
}

// ─── Draw ─────────────────────────────────────────────────────────────────────

void draw_detections(cv::Mat& img, const std::vector<Detection>& dets)
{
    const std::vector<cv::Scalar> COLOURS = {
        {0,255,255},
        {255,128,0},
        {0,128,255},
        {0,0,200}
    };

    // <-- Replace from here
    for (const auto& d : dets)
    {
        std::cout
            << "\n========== DRAW ==========\n"
            << "label = " << d.label << "\n"
            << "class = " << d.class_id << "\n"
            << "conf  = " << d.confidence << "\n"
            << "box   = ("
            << d.box.x << ", "
            << d.box.y << ", "
            << d.box.width << ", "
            << d.box.height << ")\n";

        cv::Scalar col = COLOURS[d.class_id];

        std::cout
            << "BGR = ("
            << col[0] << ", "
            << col[1] << ", "
            << col[2] << ")\n"
            << "==========================\n";

        cv::rectangle(img, d.box, col, 2);

        std::string text = d.label + " " +
            std::to_string((int)(d.confidence * 100)) + "%";

        int baseline = 0;
        cv::Size ts = cv::getTextSize(
            text,
            cv::FONT_HERSHEY_SIMPLEX,
            0.5,
            1,
            &baseline);

        cv::Rect bg(
            d.box.x,
            d.box.y - ts.height - 8,
            ts.width + 4,
            ts.height + 8);

        bg &= cv::Rect(0, 0, img.cols, img.rows);

        cv::rectangle(img, bg, col, cv::FILLED);

        cv::putText(
            img,
            text,
            cv::Point(d.box.x + 2, d.box.y - 4),
            cv::FONT_HERSHEY_SIMPLEX,
            0.5,
            cv::Scalar(0,0,0),
            1,
            cv::LINE_AA);
    }
    // <-- Replace until here
}

// ─── Main ─────────────────────────────────────────────────────────────────────

/*
int main() {
    std::cout << "[INFO] Classes: ";
    for (const auto& c : CLASS_NAMES) std::cout << c << " ";
    std::cout << "\n";

    cv::Mat img = cv::imread(IMAGE_PATH, cv::IMREAD_COLOR);
    if (img.empty()) {
        std::cerr << "[ERROR] Cannot read image: " << IMAGE_PATH << "\n";
        return EXIT_FAILURE;
    }
    std::cout << "[INFO] Image: " << img.cols << "x" << img.rows << "\n";

    cv::dnn::Net net = load_net(MODEL_PATH, USE_CUDA);
    auto dets = detect(img, net);
    draw_detections(img, dets);

    cv::imshow("Detections", img);
    cv::waitKey(0);
    cv::destroyAllWindows();
    return EXIT_SUCCESS;
}

*/
