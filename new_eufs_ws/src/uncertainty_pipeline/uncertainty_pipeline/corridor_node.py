"""ROS 2 node that turns covariance-bearing SLAM landmarks into safe corridors."""

import math
from typing import List, Optional, Sequence, Tuple

from eufs_msgs.msg import ConeArrayWithCovariance
from geometry_msgs.msg import Point, PoseStamped
from nav_msgs.msg import Odometry, Path
import rclpy
from rclpy.node import Node
from std_msgs.msg import Float64MultiArray, String
from visualization_msgs.msg import Marker, MarkerArray

from .math_utils import (
    BoundarySample,
    Landmark,
    Matrix3,
    Pose2,
    build_corridor,
    is_valid_covariance_2,
    local_boundary_samples,
)


BLUE = 0
YELLOW = 1


class CorridorNode(Node):
    """Constructs an observable local safe corridor without changing the baseline planner."""

    def __init__(self) -> None:
        super().__init__('uncertainty_corridor')
        self._safety_k = float(self.declare_parameter('safety_k', 2.0).value)
        self._grid_step_m = float(self.declare_parameter('grid_step_m', 0.5).value)
        self._landmarks_topic = self.declare_parameter(
            'landmarks_topic', '/slam/landmarks').value
        self._odom_topic = self.declare_parameter('odom_topic', '/slam/odom').value
        self._landmarks: List[Landmark] = []
        self._pose: Optional[Pose2] = None
        self._pose_covariance: Optional[Matrix3] = None

        if self._safety_k not in (1.0, 2.0, 3.0):
            raise ValueError('safety_k must be one of 1, 2, or 3')
        if self._grid_step_m <= 0.0:
            raise ValueError('grid_step_m must be positive')

        self.create_subscription(
            ConeArrayWithCovariance, self._landmarks_topic,
            self._landmarks_callback, 10)
        self.create_subscription(Odometry, self._odom_topic, self._odom_callback, 20)
        self._corridor_pub = self.create_publisher(
            Float64MultiArray, '/uncertainty/corridor', 10)
        self._lower_path_pub = self.create_publisher(
            Path, '/uncertainty/lower_boundary_mean', 10)
        self._upper_path_pub = self.create_publisher(
            Path, '/uncertainty/upper_boundary_mean', 10)
        self._centre_path_pub = self.create_publisher(
            Path, '/uncertainty/corridor_centerline', 10)
        self._markers_pub = self.create_publisher(
            MarkerArray, '/uncertainty/markers', 10)
        self._status_pub = self.create_publisher(String, '/uncertainty/status', 10)
        self.get_logger().info(
            f'Corridor: {self._landmarks_topic} + {self._odom_topic} -> /uncertainty/corridor')

    def _landmarks_callback(self, message: ConeArrayWithCovariance) -> None:
        landmarks: List[Landmark] = []
        for color, cones in (
            (BLUE, message.blue_cones),
            (YELLOW, message.yellow_cones),
            (2, message.orange_cones),
            (2, message.big_orange_cones),
            (3, message.unknown_color_cones),
        ):
            for cone in cones:
                covariance = ((cone.covariance[0], cone.covariance[1]),
                              (cone.covariance[2], cone.covariance[3]))
                if not is_valid_covariance_2(covariance):
                    continue
                landmarks.append(Landmark((cone.point.x, cone.point.y), covariance, color))
        self._landmarks = landmarks
        self._publish_if_ready()

    def _odom_callback(self, message: Odometry) -> None:
        quaternion = message.pose.pose.orientation
        yaw = math.atan2(
            2.0 * (quaternion.w * quaternion.z + quaternion.x * quaternion.y),
            1.0 - 2.0 * (quaternion.y * quaternion.y + quaternion.z * quaternion.z))
        covariance = message.pose.covariance
        planar_covariance: Matrix3 = (
            (covariance[0], covariance[1], covariance[5]),
            (covariance[6], covariance[7], covariance[11]),
            (covariance[30], covariance[31], covariance[35]),
        )
        if not all(math.isfinite(value) for row in planar_covariance for value in row):
            self._publish_status('rejected non-finite pose covariance')
            return
        self._pose = (message.pose.pose.position.x, message.pose.pose.position.y, yaw)
        self._pose_covariance = planar_covariance
        self._publish_if_ready()

    def _publish_if_ready(self) -> None:
        if self._pose is None or self._pose_covariance is None or not self._landmarks:
            return
        blue = local_boundary_samples(
            self._landmarks, self._pose, self._pose_covariance, BLUE)
        yellow = local_boundary_samples(
            self._landmarks, self._pose, self._pose_covariance, YELLOW)
        corridor = build_corridor(blue, yellow, self._safety_k, self._grid_step_m)
        if not corridor:
            self._publish_status('no overlapping blue/yellow boundary support')
            return
        self._publish_corridor(corridor)
        self._publish_paths(corridor)
        self._publish_markers(blue, yellow, corridor)
        invalid = sum(1 for sample in corridor if not sample.valid)
        self._publish_status(
            f'k={self._safety_k:g}; samples={len(corridor)}; collapsed={invalid}')

    def _publish_corridor(self, corridor) -> None:
        # Each row is [s, lower_mu, lower_sigma, upper_mu, upper_sigma,
        #              y_min, y_max, valid]. Coordinates are vehicle-local.
        message = Float64MultiArray()
        for sample in corridor:
            message.data.extend((
                sample.s, sample.lower_mean, sample.lower_sigma,
                sample.upper_mean, sample.upper_sigma,
                sample.y_min, sample.y_max, 1.0 if sample.valid else 0.0))
        self._corridor_pub.publish(message)

    def _world_point(self, s: float, y: float) -> Tuple[float, float]:
        assert self._pose is not None
        x, world_y, yaw = self._pose
        c = math.cos(yaw)
        sine = math.sin(yaw)
        return x + c * s - sine * y, world_y + sine * s + c * y

    def _path_from_samples(self, values: Sequence[Tuple[float, float]]) -> Path:
        path = Path()
        path.header.stamp = self.get_clock().now().to_msg()
        path.header.frame_id = 'map'
        for s, lateral in values:
            pose = PoseStamped()
            pose.header = path.header
            pose.pose.position.x, pose.pose.position.y = self._world_point(s, lateral)
            pose.pose.orientation.w = 1.0
            path.poses.append(pose)
        return path

    def _publish_paths(self, corridor) -> None:
        self._lower_path_pub.publish(self._path_from_samples(
            [(sample.s, sample.lower_mean) for sample in corridor]))
        self._upper_path_pub.publish(self._path_from_samples(
            [(sample.s, sample.upper_mean) for sample in corridor]))
        self._centre_path_pub.publish(self._path_from_samples([
            (sample.s, 0.5 * (sample.y_min + sample.y_max))
            for sample in corridor if sample.valid]))

    def _line_marker(self, marker_id: int, namespace: str,
                     values: Sequence[Tuple[float, float]], rgb: Tuple[float, float, float],
                     width: float = 0.06) -> Marker:
        marker = Marker()
        marker.header.stamp = self.get_clock().now().to_msg()
        marker.header.frame_id = 'map'
        marker.ns = namespace
        marker.id = marker_id
        marker.type = Marker.LINE_STRIP
        marker.action = Marker.ADD
        marker.scale.x = width
        marker.color.r, marker.color.g, marker.color.b = rgb
        marker.color.a = 1.0
        for s, lateral in values:
            point = Point()
            point.x, point.y = self._world_point(s, lateral)
            marker.points.append(point)
        return marker

    def _ellipse_marker(self, marker_id: int, namespace: str, point_xy: Tuple[float, float],
                        covariance, rgb: Tuple[float, float, float]) -> Optional[Marker]:
        if not is_valid_covariance_2(covariance):
            return None
        a, b = covariance[0]
        _, d = covariance[1]
        discriminant = max(0.0, (a - d) * (a - d) + 4.0 * b * b)
        major = 0.5 * (a + d + math.sqrt(discriminant))
        minor = 0.5 * (a + d - math.sqrt(discriminant))
        if major < 0.0 or minor < 0.0:
            return None
        angle = 0.5 * math.atan2(2.0 * b, a - d)
        marker = Marker()
        marker.header.stamp = self.get_clock().now().to_msg()
        marker.header.frame_id = 'map'
        marker.ns = namespace
        marker.id = marker_id
        marker.type = Marker.CYLINDER
        marker.action = Marker.ADD
        marker.pose.position.x, marker.pose.position.y = point_xy
        marker.pose.position.z = 0.02
        marker.pose.orientation.z = math.sin(angle * 0.5)
        marker.pose.orientation.w = math.cos(angle * 0.5)
        # The marker diameters represent a two-standard-deviation ellipse.
        marker.scale.x = 4.0 * math.sqrt(major)
        marker.scale.y = 4.0 * math.sqrt(minor)
        marker.scale.z = 0.02
        marker.color.r, marker.color.g, marker.color.b = rgb
        marker.color.a = 0.35
        return marker

    def _publish_markers(self, blue: Sequence[BoundarySample], yellow: Sequence[BoundarySample],
                         corridor) -> None:
        markers = MarkerArray()
        clear = Marker()
        clear.action = Marker.DELETEALL
        markers.markers.append(clear)
        for index, landmark in enumerate(self._landmarks):
            color = (0.1, 0.2, 1.0) if landmark.color == BLUE else (1.0, 0.9, 0.1)
            ellipse = self._ellipse_marker(
                100 + index, 'landmark_covariance', landmark.mean, landmark.covariance, color)
            if ellipse is not None:
                markers.markers.append(ellipse)
        assert self._pose is not None and self._pose_covariance is not None
        pose_covariance = (
            (self._pose_covariance[0][0], self._pose_covariance[0][1]),
            (self._pose_covariance[1][0], self._pose_covariance[1][1]))
        pose_ellipse = self._ellipse_marker(
            200, 'pose_covariance', (self._pose[0], self._pose[1]),
            pose_covariance, (0.9, 0.1, 0.8))
        if pose_ellipse is not None:
            markers.markers.append(pose_ellipse)
        markers.markers.append(self._line_marker(
            1, 'boundary_mean', [(sample.s, sample.mean) for sample in blue], (0.1, 0.2, 1.0)))
        markers.markers.append(self._line_marker(
            2, 'boundary_mean', [(sample.s, sample.mean) for sample in yellow], (1.0, 0.9, 0.1)))
        markers.markers.append(self._line_marker(
            3, 'corridor_mean', [(sample.s, sample.lower_mean) for sample in corridor], (0.8, 0.8, 0.8)))
        markers.markers.append(self._line_marker(
            4, 'corridor_mean', [(sample.s, sample.upper_mean) for sample in corridor], (0.8, 0.8, 0.8)))
        colors = {1.0: (0.1, 0.8, 0.2), 2.0: (1.0, 0.55, 0.1), 3.0: (0.9, 0.1, 0.1)}
        for index, k in enumerate((1.0, 2.0, 3.0)):
            samples = build_corridor(blue, yellow, k, self._grid_step_m)
            valid_samples = [sample for sample in samples if sample.valid]
            if not valid_samples:
                continue
            color = colors[k]
            markers.markers.append(self._line_marker(
                10 + index * 2, f'corridor_k_{k:g}',
                [(sample.s, sample.y_min) for sample in valid_samples], color, 0.035))
            markers.markers.append(self._line_marker(
                11 + index * 2, f'corridor_k_{k:g}',
                [(sample.s, sample.y_max) for sample in valid_samples], color, 0.035))
        self._markers_pub.publish(markers)

    def _publish_status(self, text: str) -> None:
        message = String()
        message.data = text
        self._status_pub.publish(message)


def main(args=None) -> None:
    rclpy.init(args=args)
    node = CorridorNode()
    try:
        rclpy.spin(node)
    finally:
        node.destroy_node()
        rclpy.shutdown()
