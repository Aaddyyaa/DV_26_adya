"""Conservative local centreline planner for the EUFS cone track."""

import math
from functools import lru_cache
from typing import List, Optional, Sequence, Set, Tuple

from eufs_msgs.msg import ConeArray
from geometry_msgs.msg import PoseStamped
from nav_msgs.msg import Odometry, Path
from rclpy.node import Node
from std_msgs.msg import Float64MultiArray


Point2 = Tuple[float, float]
LocalCone = Tuple[Point2, float, float, float]


def distance(a: Point2, b: Point2) -> float:
    return math.hypot(a[0] - b[0], a[1] - b[1])


def wrap_to_pi(angle: float) -> float:
    return math.atan2(math.sin(angle), math.cos(angle))


def _unit(dx: float, dy: float) -> Optional[Point2]:
    length = math.hypot(dx, dy)
    if length <= 1e-9:
        return None
    return dx / length, dy / length


class CentrelinePlanner(Node):
    """Build a conservative, ordered centreline from local cone observations."""

    def __init__(self) -> None:
        super().__init__('path_planner')

        cones_topic = self.declare_parameter(
            'planning_cones_topic', '/planning/cones').value
        odom_topic = self.declare_parameter('odom_topic', '/slam/odom').value
        path_topic = self.declare_parameter(
            'target_path_topic', '/target_path').value
        speeds_topic = self.declare_parameter(
            'target_speeds_topic', '/target_speeds').value

        self._min_track_width = float(
            self.declare_parameter('min_track_width_m', 2.0).value)
        self._max_track_width = float(
            self.declare_parameter('max_track_width_m', 5.5).value)
        self._nominal_track_width = float(
            self.declare_parameter('nominal_track_width_m', 4.5).value)
        self._min_points = int(
            self.declare_parameter('min_centerline_points', 2).value)
        self._max_speed = float(
            self.declare_parameter('max_speed_mps', 1.5).value)
        self._min_speed = float(
            self.declare_parameter('min_speed_mps', 0.6).value)
        self._max_lateral_accel = float(
            self.declare_parameter('max_lateral_accel', 2.0).value)
        self._max_segment = float(
            self.declare_parameter('max_segment_length_m', 6.0).value)
        self._max_gate_turn = math.radians(float(
            self.declare_parameter('max_gate_turn_deg', 95.0).value))
        self._max_pair_station_gap = float(
            self.declare_parameter('max_pair_station_gap_m', 2.5).value)
        self._local_range = float(
            self.declare_parameter('local_range_m', 18.0).value)
        self._local_half_width = float(
            self.declare_parameter('local_half_width_m', 9.0).value)
        self._path_sample_step = float(
            self.declare_parameter('path_sample_step_m', 0.5).value)
        self._path_hold_sec = float(
            self.declare_parameter('path_hold_time_sec', 8.0).value)
        self._max_path_jump = float(
            self.declare_parameter('max_path_jump_m', 3.5).value)
        self._cone_dedup_distance = float(
            self.declare_parameter('cone_dedup_distance_m', 0.75).value)

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
        self._speeds_pub = self.create_publisher(
            Float64MultiArray, speeds_topic, 10)
        self._timer = self.create_timer(0.2, self._publish_plan)

        self.get_logger().info(
            f'Conservative planner: {cones_topic} + {odom_topic} -> {path_topic}')

    def _cones_callback(self, msg: ConeArray) -> None:
        self._cones = msg

    def _odom_callback(self, msg: Odometry) -> None:
        self._position = (
            msg.pose.pose.position.x,
            msg.pose.pose.position.y,
        )
        q = msg.pose.pose.orientation
        self._yaw = math.atan2(
            2.0 * (q.w * q.z + q.x * q.y),
            1.0 - 2.0 * (q.y * q.y + q.z * q.z))
        self._have_odom = True

    def _deduplicate_points(self, points: Sequence[Point2]) -> List[Point2]:
        unique: List[Point2] = []
        threshold = max(0.10, self._cone_dedup_distance)
        for point in sorted(points, key=lambda p: distance(p, self._position)):
            if any(distance(point, old) <= threshold for old in unique):
                continue
            if math.isfinite(point[0]) and math.isfinite(point[1]):
                unique.append(point)
        return unique

    def _localise(
        self,
        points: Sequence[Point2],
        c_yaw: float,
        s_yaw: float,
    ) -> List[LocalCone]:
        output: List[LocalCone] = []
        for point in points:
            dx = point[0] - self._position[0]
            dy = point[1] - self._position[1]
            local_s = dx * c_yaw + dy * s_yaw
            local_l = -dx * s_yaw + dy * c_yaw
            local_range = math.hypot(local_s, local_l)

            if local_range > self._local_range:
                continue
            if local_s < -2.0:
                continue
            if abs(local_l) > self._local_half_width:
                continue

            output.append((point, local_s, local_l, local_range))

        output.sort(key=lambda item: item[3])
        return output

    def _pair_cones(
        self,
        blue: Sequence[LocalCone],
        yellow: Sequence[LocalCone],
    ) -> List[Tuple[Point2, float]]:
        """Find an order-preserving blue/yellow association."""

        if not blue or not yellow:
            return []

        heading = (math.cos(self._yaw), math.sin(self._yaw))
        candidate_cost = {}

        for i, (_, blue_s, blue_l, blue_range) in enumerate(blue):
            for j, (_, yellow_s, yellow_l, yellow_range) in enumerate(yellow):
                station_gap = abs(blue_s - yellow_s)
                range_gap = abs(blue_range - yellow_range)
                if station_gap > self._max_pair_station_gap:
                    continue

                width = distance(blue[i][0], yellow[j][0])
                if not self._min_track_width <= width <= self._max_track_width:
                    continue

                midpoint_s = 0.5 * (blue_s + yellow_s)
                midpoint_l = 0.5 * (blue_l + yellow_l)
                if midpoint_s < -2.0 or abs(midpoint_l) > 7.0:
                    continue

                gate_dx = yellow[j][0][0] - blue[i][0][0]
                gate_dy = yellow[j][0][1] - blue[i][0][1]
                gate_unit = _unit(gate_dx, gate_dy)
                if gate_unit is None:
                    continue

                # A real gate crosses the track. A pair whose connecting
                # segment runs almost parallel to the car's travel direction
                # is a likely cross-section/shortcut pair.
                along_track = abs(
                    gate_unit[0] * heading[0]
                    + gate_unit[1] * heading[1])
                if along_track > 0.80:
                    continue

                candidate_cost[(i, j)] = (
                    1.4 * station_gap
                    + 0.7 * range_gap
                    + 0.9 * abs(width - self._nominal_track_width)
                    + 0.15 * abs(midpoint_l)
                )

        if not candidate_cost:
            return []

        skip_cost = 2.8

        @lru_cache(maxsize=None)
        def solve(i: int, j: int) -> Tuple[
            float, Tuple[Tuple[int, int], ...]
        ]:
            if i >= len(blue) or j >= len(yellow):
                return 0.0, ()

            best_cost, best_pairs = solve(i + 1, j)
            best_cost += skip_cost

            alt_cost, alt_pairs = solve(i, j + 1)
            alt_cost += skip_cost
            if alt_cost < best_cost:
                best_cost, best_pairs = alt_cost, alt_pairs

            pair_cost = candidate_cost.get((i, j))
            if pair_cost is not None:
                tail_cost, tail_pairs = solve(i + 1, j + 1)
                pair_total = pair_cost + tail_cost
                if (
                    pair_total < best_cost
                    or (
                        abs(pair_total - best_cost) < 1e-9
                        and len(tail_pairs) + 1 > len(best_pairs)
                    )
                ):
                    best_cost = pair_total
                    best_pairs = ((i, j),) + tail_pairs

            return best_cost, best_pairs

        _, pairs = solve(0, 0)
        if len(pairs) < self._min_points:
            return []

        gates: List[Tuple[Point2, float]] = []
        for i, j in pairs:
            midpoint = (
                0.5 * (blue[i][0][0] + yellow[j][0][0]),
                0.5 * (blue[i][0][1] + yellow[j][0][1]),
            )
            route_order = 0.5 * (blue[i][3] + yellow[j][3])
            gates.append((midpoint, route_order))

        gates.sort(key=lambda item: item[1])
        return gates

    def _build_route(
        self,
        gates: Sequence[Tuple[Point2, float]],
    ) -> List[Point2]:
        if len(gates) < self._min_points:
            return []

        first_index = None
        for index, (point, _) in enumerate(gates):
            dx = point[0] - self._position[0]
            dy = point[1] - self._position[1]
            rng = math.hypot(dx, dy)
            forward = dx * math.cos(self._yaw) + dy * math.sin(self._yaw)
            if 0.8 <= rng <= 10.0 and forward >= -1.5:
                first_index = index
                break

        if first_index is None:
            return []

        ordered = [gates[first_index][0]]
        first_vector = _unit(
            ordered[0][0] - self._position[0],
            ordered[0][1] - self._position[1],
        )
        direction = first_vector or (math.cos(self._yaw), math.sin(self._yaw))

        # Once an ordered gate sequence is established, never choose an
        # arbitrary shorter chord from the remaining points. This is the core
        # shortcut prevention rule.
        for point, _ in gates[first_index + 1:]:
            dx = point[0] - ordered[-1][0]
            dy = point[1] - ordered[-1][1]
            segment = math.hypot(dx, dy)

            if segment < 0.8:
                continue
            if segment > self._max_segment:
                break

            unit = (dx / segment, dy / segment)
            alignment = unit[0] * direction[0] + unit[1] * direction[1]
            turn_angle = abs(math.atan2(
                direction[0] * unit[1] - direction[1] * unit[0],
                alignment,
            ))

            if turn_angle > self._max_gate_turn:
                break
            if alignment < -0.05:
                break

            ordered.append(point)
            direction = unit

            if len(ordered) >= 24:
                break

        if len(ordered) < self._min_points:
            return []

        route_length = sum(
            distance(a, b)
            for a, b in zip(ordered, ordered[1:])
        )
        if route_length < 2.5:
            return []

        return ordered

    def _densify(self, points: Sequence[Point2]) -> List[Point2]:
        if not points:
            return []

        output: List[Point2] = [points[0]]
        step = max(0.2, self._path_sample_step)

        for start, end in zip(points, points[1:]):
            dx = end[0] - start[0]
            dy = end[1] - start[1]
            length = math.hypot(dx, dy)
            if length <= 1e-6:
                continue

            count = max(1, int(math.ceil(length / step)))
            for k in range(1, count + 1):
                alpha = k / count
                output.append((
                    start[0] + alpha * dx,
                    start[1] + alpha * dy,
                ))

        return output

    def _path_is_safe(self, points: Sequence[Point2]) -> bool:
        """Reject only physically discontinuous paths.

        Tight Formula Student corners are expected. Continuity is enforced by
        the ordered cone graph in _build_track_route; the final path check
        therefore focuses on impossible jumps/cusps instead of comparing a
        corner against the previous path.
        """
        if len(points) < self._min_points:
            return False

        for first, second in zip(points, points[1:]):
            segment = distance(first, second)
            if not math.isfinite(segment):
                return False
            if segment > max(1.25, self._path_sample_step * 2.5):
                return False

        for a, b, c in zip(points, points[1:], points[2:]):
            h1 = math.atan2(b[1] - a[1], b[0] - a[0])
            h2 = math.atan2(c[1] - b[1], c[0] - b[0])
            if abs(wrap_to_pi(h2 - h1)) > math.radians(120.0):
                return False

        return True


    def _build_track_route(self, cones: ConeArray) -> List[Point2]:
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
        local_radius = max(30.0, self._local_range * 2.0)
        local_half_width = max(12.0, self._local_half_width * 2.0)

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

        candidates: List[Tuple[float, float, float, int, int]] = []

        for blue_index, (_, blue_s, blue_l) in enumerate(blue):
            for yellow_index, (_, yellow_s, yellow_l) in enumerate(yellow):
                midpoint_s = 0.5 * (blue_s + yellow_s)
                midpoint_l = 0.5 * (blue_l + yellow_l)
                gap = abs(blue_s - yellow_s)

                # A real corner can put the two cones at noticeably
                # different longitudinal stations.  Do not reject that
                # pairing merely because it is no longer "in front" of the
                # original vehicle heading.
                if gap > max(3.5, self._max_pair_station_gap):
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
            ) >= -1.0
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
                if turn_angle > max(math.radians(120.0), self._max_gate_turn):
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
                        if turn_angle > max(math.radians(130.0), self._max_gate_turn):
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


    def _speed_profile(self, points: Sequence[Point2]) -> List[float]:
        if not points:
            return []

        speeds = [self._max_speed] * len(points)

        for index in range(1, len(points) - 1):
            before = points[index - 1]
            current = points[index]
            after = points[index + 1]

            heading_before = math.atan2(
                current[1] - before[1],
                current[0] - before[0],
            )
            heading_after = math.atan2(
                after[1] - current[1],
                after[0] - current[0],
            )
            step = max(
                0.2,
                0.5 * (
                    distance(before, current)
                    + distance(current, after)
                ),
            )
            curvature = abs(
                wrap_to_pi(heading_after - heading_before)
            ) / step

            speed = math.sqrt(
                self._max_lateral_accel
                / max(curvature, 1e-3)
            )
            speeds[index] = min(
                self._max_speed,
                max(self._min_speed, speed),
            )

        if len(speeds) >= 2:
            speeds[0] = min(speeds[0], speeds[1])
            speeds[-1] = min(speeds[-1], speeds[-2])

        return speeds

    def _make_path(self, points: Sequence[Point2]) -> Path:
        path = Path()
        path.header.stamp = self.get_clock().now().to_msg()
        path.header.frame_id = 'map'

        for index, point in enumerate(points):
            pose = PoseStamped()
            pose.header = path.header
            pose.pose.position.x = point[0]
            pose.pose.position.y = point[1]

            if index + 1 < len(points):
                next_point = points[index + 1]
                yaw = math.atan2(
                    next_point[1] - point[1],
                    next_point[0] - point[0],
                )
            elif index > 0:
                previous = points[index - 1]
                yaw = math.atan2(
                    point[1] - previous[1],
                    point[0] - previous[0],
                )
            else:
                yaw = self._yaw

            pose.pose.orientation.z = math.sin(yaw * 0.5)
            pose.pose.orientation.w = math.cos(yaw * 0.5)
            path.poses.append(pose)

        return path

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
            route = self._build_track_route(self._cones)

            if route:
                raw_points = [self._position]
                if distance(self._position, route[0]) > 0.2:
                    raw_points.append(route[0])
                raw_points.extend(route[1:])

                candidate = self._densify(raw_points)

                if self._path_is_safe(candidate):
                    path = self._make_path(candidate)
                    speeds.data = self._speed_profile(candidate)

                    self._last_path = path
                    self._last_speeds = list(speeds.data)
                    self._last_valid_plan_time = (
                        self.get_clock().now().nanoseconds * 1e-9
                    )
                else:
                    self.get_logger().warn(
                        'Rejected discontinuous candidate path; holding last valid path.',
                        throttle_duration_sec=2.0,
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
