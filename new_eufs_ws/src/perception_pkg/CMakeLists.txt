cmake_minimum_required(VERSION 3.8)
project(perception_pkg)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(sensor_msgs REQUIRED)
find_package(std_msgs REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(cone_msgs REQUIRED)
find_package(eufs_msgs REQUIRED)
find_package(cv_bridge REQUIRED)
find_package(image_transport REQUIRED)
find_package(OpenCV REQUIRED)

include_directories(
  src
  src/Main
  src/Cone
  src/Keypoints
  src/Depth
  src/Calibration
  ${OpenCV_INCLUDE_DIRS}
)

add_executable(perception_node
  src/Main/main.cpp
  src/Main/perception_node.cpp
  src/Cone/cone.cpp
  src/Keypoints/keypoints.cpp
  src/Depth/depth.cpp
  src/Calibration/calibration.cpp
  src/Calibration/rectification.cpp
)

ament_target_dependencies(
  perception_node
  rclcpp
  sensor_msgs
  std_msgs
  geometry_msgs
  cone_msgs
  eufs_msgs
  cv_bridge
  image_transport
)

target_link_libraries(
  perception_node
  ${OpenCV_LIBRARIES}
)
install(
  TARGETS perception_node
  DESTINATION lib/${PROJECT_NAME}
)

ament_package()
