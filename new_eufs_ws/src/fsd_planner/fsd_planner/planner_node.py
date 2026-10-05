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
        """Generate local blue-left/yellow-right centreline gates.

        Pairing is performed in the vehicle frame so the planner only uses
        gates that actually straddle the current vehicle corridor. Global
        track sections behind/alongside the car cannot become the next gate.
        """
        blue_points = self._deduplicate_points(
            [(cone.x, cone.y) for cone in cones.blue_cones])
        yellow_points = self._deduplicate_points(
            [(cone.x, cone.y) for cone in cones.yellow_cones])

        c_yaw = math.cos(self._yaw)
        s_yaw = math.sin(self._yaw)

        def local(point: Point2) -> Tuple[float, float]:
            dx = point[0] - self._position[0]
            dy = point[1] - self._position[1]
            return (
                dx * c_yaw + dy * s_yaw,
                -dx * s_yaw + dy * c_yaw,
            )

        # Keep the complete local mapped cone set.  The old planner
        # discarded every cone with local x < -1 m, which is exactly what
        # happens to the next gate when the car enters a tight right turn.
        # We still limit the working set spatially so distant/old map sections
        # cannot become the next gate.
        local_radius = 35.0
        local_half_width = 18.0

        blue = [
            (point, *local(point))
            for point in blue_points
            if math.hypot(*local(point)) <= local_radius
            and abs(local(point)[1]) <= local_half_width
        ]
        yellow = [
            (point, *local(point))
            for point in yellow_points
            if math.hypot(*local(point)) <= local_radius
            and abs(local(point)[1]) <= local_half_width
        ]

        def build_candidates(require_side_geometry: bool) -> List[Tuple[float, float, float, int, int]]:
            """Score blue/yellow pairs inside the local cone window."""
            candidates: List[Tuple[float, float, float, int, int]] = []

            for blue_index, (_, blue_s, blue_l) in enumerate(blue):
                for yellow_index, (_, yellow_s, yellow_l) in enumerate(yellow):
                    # Prefer the known EUFS colour orientation when there are
                    # multiple nearby track sections.  This remains a local
                    # gate check: it does not use the persistent map or a
                    # global route.
                    if require_side_geometry:
                        if blue_l < 0.10 or yellow_l > -0.10:
                            continue

                    midpoint_s = 0.5 * (blue_s + yellow_s)
                    midpoint_l = 0.5 * (blue_l + yellow_l)
                    gap = abs(blue_s - yellow_s)

                    # A real corner can put the two cones at noticeably
                    # different longitudinal stations.  Do not reject that
                    # pairing merely because it is no longer "in front" of
                    # the original vehicle heading.
                    if gap > 3.5:
                        continue

                    width = distance(
                        blue[blue_index][0],
                        yellow[yellow_index][0],
                    )
                    if not (self._min_track_width <= width <= self._max_track_width):
                        continue

                    # Pair by cross-track geometry first.  Forward distance is
                    # deliberately weak here; route ordering below decides which
                    # gate is the next gate.  This prevents a straight-ahead gate
                    # from winning simply because a genuine right-hand gate has
                    # a large heading change.
                    score = (
                        1.0 * gap
                        + 0.40 * abs(width - 3.5)
                        + 0.15 * abs(midpoint_l)
                    )

                    candidates.append((
                        score,
                        midpoint_s,
                        midpoint_l,
                        blue_index,
                        yellow_index,
                    ))

            return candidates

        # First use the physically correct blue-left / yellow-right geometry.
        # At least two gates are needed to define a driveable path.  On startup
        # or during a very noisy frame, fall back to the proven unrestricted
        # local-window pairing rather than stopping the car.
        candidates = build_candidates(require_side_geometry=True)
        if len(candidates) < max(2, int(self._min_points)):
            candidates = build_candidates(require_side_geometry=False)

        candidates.sort()

        # Greedy one-to-one pairing using the local physical gates.
        used_blue: Set[int] = set()
        used_yellow: Set[int] = set()
        gates: List[Tuple[Point2, float]] = []

        for score, midpoint_s, midpoint_l, blue_index, yellow_index in candidates:
            if blue_index in used_blue or yellow_index in used_yellow:
                continue

            blue_point = blue[blue_index][0]
            yellow_point = yellow[yellow_index][0]
            midpoint = (
                0.5 * (blue_point[0] + yellow_point[0]),
                0.5 * (blue_point[1] + yellow_point[1]),
            )

            used_blue.add(blue_index)
            used_yellow.add(yellow_index)
            gates.append((midpoint, midpoint_s))

        # Follow the gates as a local track graph. This is the key fix for the
        # right-hand turn: after the first gate, direction is allowed to rotate
        # with the track instead of forcing every candidate to remain aligned
        # with the original vehicle heading.
        if not gates:
            return []

        unused = [point for point, _ in gates]

        # Start from the nearest gate that is still in the current forward
        # neighbourhood.  After that first gate, the route is allowed to
        # rotate with the track; this is what lets the planner enter a right
        # hand corner instead of repeatedly selecting a straight continuation.
        forward_gates = [
            point for point in unused
            if (
                (point[0] - self._position[0]) * c_yaw
                + (point[1] - self._position[1]) * s_yaw
            ) >= -3.0
        ]
        if not forward_gates:
            return []

        first = min(
            forward_gates,
            key=lambda point: distance(point, self._position))
        ordered = [first]
        unused.remove(first)

        direction = (c_yaw, s_yaw)

        while unused and len(ordered) < 30:
            previous = ordered[-1]
            best = None
            best_score = float('inf')

            for point in unused:
                dx = point[0] - previous[0]
                dy = point[1] - previous[1]
                segment = math.hypot(dx, dy)

                if segment < 0.75 or segment > 6.0:
                    continue

                unit = (dx / segment, dy / segment)
                alignment = (
                    unit[0] * direction[0] +
                    unit[1] * direction[1]
                )
                turn_angle = abs(
                    math.atan2(
                        direction[0] * unit[1] - direction[1] * unit[0],
                        alignment,
                    )
                )

                # Permit genuine Formula Student corners.  The previous
                # cost made heading change three times more expensive than
                # distance, so a straight-looking branch won over the actual
                # right-hand continuation.  Use heading only as a continuity
                # term and allow up to 135 degrees for a tight corner.
                if turn_angle > math.radians(135.0):
                    continue

                score = (
                    1.0 * segment
                    + 0.75 * turn_angle
                    + 2.0 * max(0.0, -alignment)
                )

                if score < best_score:
                    best_score = score
                    best = point

            if best is None:
                # Final-bend fallback: keep the proven local planner and only
                # look globally when the local gate graph has ended. The
                # candidate is still constrained by the current endpoint,
                # route direction, track width, and cone station, so this
                # cannot create the cross-track pairing seen with a fully
                # global planner.
                continuation = None
                continuation_score = float('inf')

                for blue_point in blue_points:
                    for yellow_point in yellow_points:
                        blue_station = (
                            (blue_point[0] - previous[0]) * direction[0]
                            + (blue_point[1] - previous[1]) * direction[1]
                        )
                        yellow_station = (
                            (yellow_point[0] - previous[0]) * direction[0]
                            + (yellow_point[1] - previous[1]) * direction[1]
                        )
                        station_gap = abs(blue_station - yellow_station)
                        if station_gap > 2.0:
                            continue

                        width = distance(blue_point, yellow_point)
                        if not (
                            self._min_track_width
                            <= width
                            <= self._max_track_width
                        ):
                            continue

                        midpoint = (
                            0.5 * (blue_point[0] + yellow_point[0]),
                            0.5 * (blue_point[1] + yellow_point[1]),
                        )
                        dx = midpoint[0] - previous[0]
                        dy = midpoint[1] - previous[1]
                        segment = math.hypot(dx, dy)

                        if segment < 0.9 or segment > 8.0:
                            continue

                        unit_x = dx / segment
                        unit_y = dy / segment
                        alignment = (
                            unit_x * direction[0]
                            + unit_y * direction[1]
                        )
                        if alignment < -0.35:
                            continue

                        turn_angle = abs(math.atan2(
                            direction[0] * unit_y - direction[1] * unit_x,
                            alignment,
                        ))
                        if turn_angle > math.radians(145.0):
                            continue

                        if any(
                            distance(midpoint, existing) < 0.75
                            for existing in ordered
                        ):
                            continue

                        score = (
                            1.0 * segment
                            + 0.8 * turn_angle
                            + 0.8 * abs(width - 3.5)
                            + 0.8 * station_gap
                        )
                        if score < continuation_score:
                            continuation_score = score
                            continuation = midpoint

                if continuation is None:
                    break

                dx = continuation[0] - previous[0]
                dy = continuation[1] - previous[1]
                segment = math.hypot(dx, dy)
                direction = (dx / segment, dy / segment)
                ordered.append(continuation)
                continue

            dx = best[0] - previous[0]
            dy = best[1] - previous[1]
            segment = math.hypot(dx, dy)
            direction = (dx / segment, dy / segment)

            ordered.append(best)
            unused.remove(best)

        return ordered

    def _order_midpoints(self, midpoints: Sequence[Point2]) -> List[Point2]:
        # Pairing already produces a locally ordered track graph. Keep only
        # forward gates and preserve that graph order; do not run a second
        # nearest-neighbour reorder which can jump across a hairpin.
        if not midpoints:
            return []

        c_yaw = math.cos(self._yaw)
        s_yaw = math.sin(self._yaw)

        return [
            point for point in midpoints
            if (
                (point[0] - self._position[0]) * c_yaw
                + (point[1] - self._position[1]) * s_yaw
            ) >= -3.0
        ]

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
            centreline = self._order_midpoints(
                self._matched_midpoints(self._cones))

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
