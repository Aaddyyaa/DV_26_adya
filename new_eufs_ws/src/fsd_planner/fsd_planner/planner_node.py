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
        self._min_points = self.declare_parameter('min_centerline_points', 2).value
        self._max_speed = self.declare_parameter('max_speed_mps', 2.0).value
        self._min_speed = self.declare_parameter('min_speed_mps', 0.6).value
        self._max_lateral_accel = self.declare_parameter('max_lateral_accel', 3.0).value
        self._max_segment = self.declare_parameter('max_segment_length_m', 12.0).value
        self._path_hold_sec = self.declare_parameter(
            'path_hold_time_sec', 1.5).value
        self._allow_pair_reuse_fallback = self.declare_parameter(
            'allow_pair_reuse_fallback', False).value
        self._cone_dedup_distance = self.declare_parameter(
            'cone_dedup_distance_m', 0.75).value
        self._cones: Optional[ConeArray] = None
        self._position: Point2 = (0.0, 0.0)
        self._yaw = 0.0
        self._have_odom = False
        self._last_path: Optional[Path] = None
        self._last_speeds: List[float] = []
        self._last_valid_plan_time: Optional[float] = None

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

    def _deduplicate_points(self, points: Sequence[Point2]) -> List[Point2]:
        """Remove repeated estimates of the same physical boundary cone."""
        unique: List[Point2] = []
        threshold = max(0.10, float(self._cone_dedup_distance))
        for point in sorted(points, key=lambda p: distance(p, self._position)):
            if any(distance(point, existing) <= threshold for existing in unique):
                continue
            unique.append(point)
        return unique

    def _matched_midpoints(self, cones: ConeArray) -> List[Point2]:
        """Create a continuous track route from the global blue/yellow map.

        Do not require a future cone to be on the current vehicle left/right
        side. At a right turn those future cones naturally move across the
        current ego frame before the vehicle has rotated. We therefore:
        1) form physically valid blue/yellow midpoint gates globally;
        2) pick the first forward gate;
        3) walk the gate graph using the direction of the route itself.
        """
        blue = self._deduplicate_points(
            [(cone.x, cone.y) for cone in cones.blue_cones])
        yellow = self._deduplicate_points(
            [(cone.x, cone.y) for cone in cones.yellow_cones])

        if not blue or not yellow:
            return []

        heading_x = math.cos(self._yaw)
        heading_y = math.sin(self._yaw)

        candidates: List[Tuple[
            float, int, int, Point2]] = []

        for blue_index, blue_point in enumerate(blue):
            blue_forward = (
                (blue_point[0] - self._position[0]) * heading_x
                + (blue_point[1] - self._position[1]) * heading_y
            )

            for yellow_index, yellow_point in enumerate(yellow):
                yellow_forward = (
                    (yellow_point[0] - self._position[0]) * heading_x
                    + (yellow_point[1] - self._position[1]) * heading_y
                )

                midpoint = (
                    0.5 * (blue_point[0] + yellow_point[0]),
                    0.5 * (blue_point[1] + yellow_point[1]),
                )

                midpoint_forward = (
                    (midpoint[0] - self._position[0]) * heading_x
                    + (midpoint[1] - self._position[1]) * heading_y
                )

                if midpoint_forward < -8.0 or midpoint_forward > 30.0:
                    continue

                station_gap = abs(blue_forward - yellow_forward)
                if station_gap > 3.0:
                    continue

                width = distance(blue_point, yellow_point)
                if not (
                    self._min_track_width
                    <= width
                    <= self._max_track_width
                ):
                    continue

                # Track-wide pairing score. Width/longitudinal consistency
                # establish the gate; current side is deliberately NOT used.
                cost = (
                    station_gap
                    + 0.45 * abs(width - 4.5)
                    + 0.015 * max(0.0, midpoint_forward)
                )

                candidates.append((
                    cost,
                    blue_index,
                    yellow_index,
                    midpoint,
                ))

        candidates.sort(key=lambda item: item[0])

        # One-to-one blue/yellow association.
        used_blue: Set[int] = set()
        used_yellow: Set[int] = set()
        gates: List[Point2] = []

        for _, blue_index, yellow_index, midpoint in candidates:
            if blue_index in used_blue or yellow_index in used_yellow:
                continue

            used_blue.add(blue_index)
            used_yellow.add(yellow_index)
            gates.append(midpoint)

        if not gates:
            return []

        # Pick the first gate ahead of the vehicle. Subsequent gates are found
        # by following the evolving route direction, not the original heading.
        first_options = []
        for point in gates:
            dx = point[0] - self._position[0]
            dy = point[1] - self._position[1]
            d = math.hypot(dx, dy)

            if d < 0.8 or d > 8.0:
                continue

            alignment = (dx * heading_x + dy * heading_y) / d
            if alignment < -0.35:
                continue

            first_options.append((
                d - 2.0 * alignment,
                point,
            ))

        if not first_options:
            return []

        first_options.sort(key=lambda item: item[0])
        ordered: List[Point2] = [first_options[0][1]]
        remaining = [
            point for point in gates
            if point != ordered[0]
        ]

        direction_x = heading_x
        direction_y = heading_y

        while remaining and len(ordered) < 40:
            previous = ordered[-1]
            best_point = None
            best_score = float('inf')

            # Normal local connection.
            for point in remaining:
                dx = point[0] - previous[0]
                dy = point[1] - previous[1]
                segment = math.hypot(dx, dy)

                if segment < 0.75 or segment > 8.0:
                    continue

                unit_x = dx / segment
                unit_y = dy / segment

                alignment = (
                    unit_x * direction_x
                    + unit_y * direction_y
                )

                turn_angle = abs(math.atan2(
                    direction_x * unit_y - direction_y * unit_x,
                    alignment,
                ))

                if turn_angle > math.radians(125.0):
                    continue

                score = (
                    segment
                    + 2.2 * turn_angle
                    + 8.0 * max(0.0, -alignment)
                )

                if score < best_score:
                    best_score = score
                    best_point = point

            # If a sparse observation leaves a gap, allow one longer connection
            # only when it continues substantially forward. This is specifically
            # for the final bend where intermediate boundary cones can be hidden.
            if best_point is None:
                for point in remaining:
                    dx = point[0] - previous[0]
                    dy = point[1] - previous[1]
                    segment = math.hypot(dx, dy)

                    if segment < 3.0 or segment > 12.0:
                        continue

                    unit_x = dx / segment
                    unit_y = dy / segment
                    alignment = (
                        unit_x * direction_x
                        + unit_y * direction_y
                    )

                    if alignment < -0.05:
                        continue

                    turn_angle = abs(math.atan2(
                        direction_x * unit_y - direction_y * unit_x,
                        alignment,
                    ))

                    if turn_angle > math.radians(150.0):
                        continue

                    score = (
                        segment
                        + 1.5 * turn_angle
                        + 5.0 * max(0.0, -alignment)
                    )

                    if score < best_score:
                        best_score = score
                        best_point = point

            if best_point is None:
                break

            dx = best_point[0] - previous[0]
            dy = best_point[1] - previous[1]
            segment = math.hypot(dx, dy)

            direction_x = dx / segment
            direction_y = dy / segment

            ordered.append(best_point)
            remaining.remove(best_point)

        return ordered

    def _order_midpoints(self, midpoints: Sequence[Point2]) -> List[Point2]:
        return list(midpoints)

    def _densify_centerline(
        self,
        centreline: Sequence[Point2],
    ) -> List[Point2]:
        """Densify with straight interpolation; never overshoot a gate."""
        if len(centreline) < 2:
            return list(centreline)

        dense: List[Point2] = [centreline[0]]
        step = 0.5

        for index in range(len(centreline) - 1):
            p0 = centreline[index]
            p1 = centreline[index + 1]
            segment_length = distance(p0, p1)
            samples = max(1, int(math.ceil(segment_length / step)))

            for sample_index in range(1, samples + 1):
                u = sample_index / samples
                dense.append((
                    p0[0] + u * (p1[0] - p0[0]),
                    p0[1] + u * (p1[1] - p0[1]),
                ))

        return dense

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

    def _restamp_path(self, source: Path) -> Path:
        path = Path()
        path.header.stamp = self.get_clock().now().to_msg()
        path.header.frame_id = source.header.frame_id or 'map'
        for source_pose in source.poses:
            pose = PoseStamped()
            pose.header = path.header
            pose.pose = source_pose.pose
            path.poses.append(pose)
        return path

    def _publish_plan(self) -> None:
        path = Path()
        path.header.stamp = self.get_clock().now().to_msg()
        path.header.frame_id = 'map'
        speeds = Float64MultiArray()

        if self._cones is not None and self._have_odom:
            gates = self._order_midpoints(
                self._matched_midpoints(self._cones))
            centreline = self._densify_centerline(gates)

            if len(centreline) >= self._min_points:
                profile = self._speed_profile(centreline)

                for index, point in enumerate(centreline):
                    pose = PoseStamped()
                    pose.header = path.header
                    pose.pose.position.x = point[0]
                    pose.pose.position.y = point[1]

                    if index + 1 < len(centreline):
                        next_point = centreline[index + 1]
                        yaw = math.atan2(
                            next_point[1] - point[1],
                            next_point[0] - point[0])
                    else:
                        previous_point = centreline[index - 1]
                        yaw = math.atan2(
                            point[1] - previous_point[1],
                            point[0] - previous_point[0])

                    pose.pose.orientation.z = math.sin(yaw * 0.5)
                    pose.pose.orientation.w = math.cos(yaw * 0.5)
                    path.poses.append(pose)

                speeds.data = profile
                self._last_path = path
                self._last_speeds = list(profile)
                self._last_valid_plan_time = (
                    self.get_clock().now().nanoseconds * 1e-9
                )

        if not path.poses:
            now_sec = self.get_clock().now().nanoseconds * 1e-9
            if (
                self._last_path is not None
                and self._last_valid_plan_time is not None
                and now_sec - self._last_valid_plan_time <= self._path_hold_sec
            ):
                path = self._restamp_path(self._last_path)
                speeds.data = list(self._last_speeds)

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
