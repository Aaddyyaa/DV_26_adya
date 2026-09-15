"""A small, deterministic centreline planner for coloured EUFS cone maps."""

import math
from typing import List, Optional, Sequence, Set, Tuple

from eufs_msgs.msg import ConeArray
from geometry_msgs.msg import PoseStamped
from nav_msgs.msg import Odometry, Path
from rclpy.node import Node
from std_msgs.msg import Float64MultiArray


Point2 = Tuple[float, float]


def distance(a: Point2, b: Point2) -> float:
    return math.hypot(a[0] - b[0], a[1] - b[1])


def wrap_to_pi(angle: float) -> float:
    return math.atan2(math.sin(angle), math.cos(angle))


class CentrelinePlanner(Node):
    """Matches blue/yellow boundaries and publishes a driveable centreline."""

    def __init__(self) -> None:
        super().__init__('path_planner')
        cones_topic = self.declare_parameter(
            'planning_cones_topic', '/planning/cones').value
        odom_topic = self.declare_parameter('odom_topic', '/slam/odom').value
        path_topic = self.declare_parameter('target_path_topic', '/target_path').value
        speeds_topic = self.declare_parameter('target_speeds_topic', '/target_speeds').value
        self._min_track_width = self.declare_parameter('min_track_width_m', 1.5).value
        self._max_track_width = self.declare_parameter('max_track_width_m', 6.0).value
        self._min_points = self.declare_parameter('min_centerline_points', 3).value
        self._max_speed = self.declare_parameter('max_speed_mps', 3.0).value
        self._min_speed = self.declare_parameter('min_speed_mps', 1.0).value
        self._max_lateral_accel = self.declare_parameter('max_lateral_accel', 3.0).value
        self._max_segment = self.declare_parameter('max_segment_length_m', 12.0).value
        self._cones: Optional[ConeArray] = None
        self._position: Point2 = (0.0, 0.0)
        self._yaw = 0.0
        self._have_odom = False

        self._cones_sub = self.create_subscription(
            ConeArray, cones_topic, self._cones_callback, 10)
        self._odom_sub = self.create_subscription(
            Odometry, odom_topic, self._odom_callback, 20)
        self._path_pub = self.create_publisher(Path, path_topic, 10)
        self._speeds_pub = self.create_publisher(Float64MultiArray, speeds_topic, 10)
        self._timer = self.create_timer(0.2, self._publish_plan)
        self.get_logger().info(
            f'Planner: {cones_topic} + {odom_topic} -> {path_topic}')

    def _cones_callback(self, msg: ConeArray) -> None:
        self._cones = msg

    def _odom_callback(self, msg: Odometry) -> None:
        self._position = (msg.pose.pose.position.x, msg.pose.pose.position.y)
        q = msg.pose.pose.orientation
        self._yaw = math.atan2(
            2.0 * (q.w * q.z + q.x * q.y),
            1.0 - 2.0 * (q.y * q.y + q.z * q.z))
        self._have_odom = True

    def _matched_midpoints(self, cones: ConeArray) -> List[Point2]:
        blue = [(cone.x, cone.y) for cone in cones.blue_cones]
        yellow = [(cone.x, cone.y) for cone in cones.yellow_cones]
        candidates: List[Tuple[float, int, int]] = []
        for blue_index, blue_point in enumerate(blue):
            for yellow_index, yellow_point in enumerate(yellow):
                width = distance(blue_point, yellow_point)
                if self._min_track_width <= width <= self._max_track_width:
                    candidates.append((width, blue_index, yellow_index))
        candidates.sort()
        used_blue: Set[int] = set()
        used_yellow: Set[int] = set()
        midpoints: List[Point2] = []
        for _, blue_index, yellow_index in candidates:
            if blue_index in used_blue or yellow_index in used_yellow:
                continue
            used_blue.add(blue_index)
            used_yellow.add(yellow_index)
            blue_point = blue[blue_index]
            yellow_point = yellow[yellow_index]
            midpoints.append(((blue_point[0] + yellow_point[0]) * 0.5,
                              (blue_point[1] + yellow_point[1]) * 0.5))
        return midpoints

    def _order_midpoints(self, midpoints: Sequence[Point2]) -> List[Point2]:
        if not midpoints:
            return []
        heading = (math.cos(self._yaw), math.sin(self._yaw))
        forward = [point for point in midpoints
                   if ((point[0] - self._position[0]) * heading[0] +
                       (point[1] - self._position[1]) * heading[1]) > -0.5]
        remaining = list(forward if forward else midpoints)
        start = min(remaining, key=lambda point: distance(point, self._position))
        ordered = [start]
        remaining.remove(start)
        direction = heading
        while remaining:
            previous = ordered[-1]
            best_point = None
            best_score = float('inf')
            for point in remaining:
                dx = point[0] - previous[0]
                dy = point[1] - previous[1]
                segment = math.hypot(dx, dy)
                if segment < 0.2 or segment > self._max_segment:
                    continue
                alignment = (dx * direction[0] + dy * direction[1]) / segment
                score = segment + 4.0 * max(0.0, -alignment)
                if score < best_score:
                    best_score = score
                    best_point = point
            if best_point is None:
                break
            dx = best_point[0] - previous[0]
            dy = best_point[1] - previous[1]
            segment = math.hypot(dx, dy)
            direction = (dx / segment, dy / segment)
            ordered.append(best_point)
            remaining.remove(best_point)
        return ordered

    def _speed_profile(self, centreline: Sequence[Point2]) -> List[float]:
        speeds = [self._max_speed] * len(centreline)
        for index in range(1, len(centreline) - 1):
            before = centreline[index - 1]
            current = centreline[index]
            after = centreline[index + 1]
            heading_before = math.atan2(current[1] - before[1], current[0] - before[0])
            heading_after = math.atan2(after[1] - current[1], after[0] - current[0])
            average_step = max(0.1, 0.5 * (distance(before, current) + distance(current, after)))
            curvature = abs(wrap_to_pi(heading_after - heading_before)) / average_step
            speed = math.sqrt(self._max_lateral_accel / max(curvature, 1e-3))
            speeds[index] = min(self._max_speed, max(self._min_speed, speed))
        if speeds:
            speeds[0] = min(speeds[0], speeds[1] if len(speeds) > 1 else self._max_speed)
            speeds[-1] = min(speeds[-1], speeds[-2] if len(speeds) > 1 else self._max_speed)
        return speeds

    def _publish_plan(self) -> None:
        path = Path()
        path.header.stamp = self.get_clock().now().to_msg()
        path.header.frame_id = 'map'
        speeds = Float64MultiArray()
        if self._cones is not None and self._have_odom:
            centreline = self._order_midpoints(self._matched_midpoints(self._cones))
            if len(centreline) >= self._min_points:
                profile = self._speed_profile(centreline)
                for index, point in enumerate(centreline):
                    pose = PoseStamped()
                    pose.header = path.header
                    pose.pose.position.x = point[0]
                    pose.pose.position.y = point[1]
                    if index + 1 < len(centreline):
                        next_point = centreline[index + 1]
                        yaw = math.atan2(next_point[1] - point[1], next_point[0] - point[0])
                    else:
                        previous_point = centreline[index - 1]
                        yaw = math.atan2(point[1] - previous_point[1], point[0] - previous_point[0])
                    pose.pose.orientation.z = math.sin(yaw * 0.5)
                    pose.pose.orientation.w = math.cos(yaw * 0.5)
                    path.poses.append(pose)
                speeds.data = profile
        self._path_pub.publish(path)
        self._speeds_pub.publish(speeds)


def main(args=None) -> None:
    import rclpy

    rclpy.init(args=args)
    node = CentrelinePlanner()
    try:
        rclpy.spin(node)
    finally:
        node.destroy_node()
        rclpy.shutdown()
