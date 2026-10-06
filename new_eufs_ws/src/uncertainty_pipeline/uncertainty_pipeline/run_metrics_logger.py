"""ROS 2 logger for trajectory and lap-completion results."""

import csv
import math
from pathlib import Path as FilePath
from typing import Optional, List, Tuple

import rclpy
from ackermann_msgs.msg import AckermannDriveStamped
from eufs_msgs.msg import CarState, ConeArrayWithCovariance
from nav_msgs.msg import Odometry, Path
from rclpy.node import Node
from std_msgs.msg import Bool, Float64MultiArray, String


Point2 = Tuple[float, float]


class RunMetricsLogger(Node):
    """Record measured vehicle/reference trajectories without synthesizing data."""

    _FIELDS = (
        "timestamp_ns",
        "x",
        "y",
        "yaw",
        "actual_speed_mps",
        "ground_truth_x",
        "ground_truth_y",
        "ground_truth_speed_mps",
        "slam_gt_error_m",
        "target_x",
        "target_y",
        "cross_track_error_m",
        "distance_travelled_m",
        "target_speed_mps",
        "steering_angle_rad",
        "command_speed_mps",
        "command_acceleration_mps2",
        "target_path_points",
        "target_path_length_m",
        "blue_landmarks",
        "yellow_landmarks",
        "total_landmarks",
        "corridor_samples",
        "corridor_valid_samples",
        "corridor_valid_fraction",
        "mean_corridor_width_m",
        "corridor_status",
        "lap_completed",
    )

    def __init__(self) -> None:
        super().__init__("run_metrics_logger")

        output_csv = self.declare_parameter(
            "output_csv", "experiments/raw/run_metrics.csv"
        ).value
        self._path = FilePath(output_csv).expanduser()
        self._path.parent.mkdir(parents=True, exist_ok=True)

        self._x = self._y = self._yaw = 0.0
        self._ground_truth_x = self._ground_truth_y = 0.0
        self._actual_speed = 0.0
        self._ground_truth_speed = 0.0
        self._target_speed: Optional[float] = None
        self._target_speeds: List[float] = []
        self._steering = 0.0
        self._command_speed = 0.0
        self._command_acceleration = 0.0
        self._blue = self._yellow = 0
        self._target_path_length = 0.0
        self._reference_history: list[Point2] = []

        self._target_path: List[Point2] = []
        self._corridor_samples = 0
        self._corridor_valid = 0
        self._corridor_width = 0.0
        self._corridor_status = "no data"
        self._lap_completed = False

        self._distance_travelled = 0.0
        self._previous_gt: Optional[Point2] = None

        # /slam/odom is expressed in the SLAM map frame, while
        # /ground_truth/state is expressed in the simulator world frame.
        # Anchor the two frames once at startup before calculating SLAM-vs-GT
        # error or CTE. Comparing the raw coordinates directly was the source
        # of the very large, physically meaningless graph errors.
        self._frame_aligned = False
        self._slam0: Optional[Tuple[float, float, float]] = None
        self._gt0: Optional[Tuple[float, float, float]] = None
        self._gt_in_map_x = 0.0
        self._gt_in_map_y = 0.0
        self._frame_cos = 1.0
        self._frame_sin = 0.0

        self.create_subscription(
            Odometry, "/slam/odom", self._odom_callback, 20
        )
        self.create_subscription(
            CarState, "/ground_truth/state", self._ground_truth_callback, 20
        )
        self.create_subscription(
            Path, "/target_path", self._target_path_callback, 10
        )
        self.create_subscription(
            AckermannDriveStamped, "/cmd", self._cmd_callback, 20
        )
        self.create_subscription(
            Float64MultiArray, "/target_speeds", self._speed_callback, 10
        )
        self.create_subscription(
            ConeArrayWithCovariance,
            "/slam/landmarks",
            self._landmark_callback,
            10,
        )
        self.create_subscription(
            String, "/uncertainty/status", self._status_callback, 10
        )
        self.create_subscription(
            Float64MultiArray,
            "/uncertainty/corridor",
            self._corridor_callback,
            10,
        )
        self.create_subscription(
            Bool,
            "/ros_can/mission_completed",
            self._mission_callback,
            10,
        )

        self._timer = self.create_timer(0.1, self._write_row)
        self.get_logger().info(f"Logging run metrics to {self._path}")

    @staticmethod
    def _point_segment_distance(
        point: Point2,
        a: Point2,
        b: Point2,
    ) -> Tuple[float, Point2]:
        sx = b[0] - a[0]
        sy = b[1] - a[1]
        segment_sq = sx * sx + sy * sy

        if segment_sq <= 1e-12:
            return math.hypot(point[0] - a[0], point[1] - a[1]), a

        t = (
            (point[0] - a[0]) * sx
            + (point[1] - a[1]) * sy
        ) / segment_sq
        t = max(0.0, min(1.0, t))

        projection = (
            a[0] + t * sx,
            a[1] + t * sy,
        )
        return (
            math.hypot(
                point[0] - projection[0],
                point[1] - projection[1],
            ),
            projection,
        )

    def _odom_callback(self, message: Odometry) -> None:
        self._x = message.pose.pose.position.x
        self._y = message.pose.pose.position.y

        q = message.pose.pose.orientation
        self._yaw = math.atan2(
            2.0 * (q.w * q.z + q.x * q.y),
            1.0 - 2.0 * (q.y * q.y + q.z * q.z),
        )

        self._actual_speed = message.twist.twist.linear.x
        self._slam0 = (self._x, self._y, self._yaw)

    def _ground_truth_callback(self, message: CarState) -> None:
        x = message.pose.pose.position.x
        y = message.pose.pose.position.y

        q = message.pose.pose.orientation
        gt_yaw = math.atan2(
            2.0 * (q.w * q.z + q.x * q.y),
            1.0 - 2.0 * (q.y * q.y + q.z * q.z),
        )

        self._ground_truth_x = x
        self._ground_truth_y = y
        self._ground_truth_speed = message.twist.twist.linear.x

        if self._gt0 is None:
            self._gt0 = (x, y, gt_yaw)

        if not self._frame_aligned and self._slam0 is not None and self._gt0 is not None:
            # R maps simulator-world displacements into the SLAM-map frame.
            delta_yaw = self._slam0[2] - self._gt0[2]
            self._frame_cos = math.cos(delta_yaw)
            self._frame_sin = math.sin(delta_yaw)
            self._frame_aligned = True

        if self._frame_aligned and self._gt0 is not None and self._slam0 is not None:
            dx = x - self._gt0[0]
            dy = y - self._gt0[1]
            self._gt_in_map_x = (
                self._slam0[0] +
                self._frame_cos * dx -
                self._frame_sin * dy
            )
            self._gt_in_map_y = (
                self._slam0[1] +
                self._frame_sin * dx +
                self._frame_cos * dy
            )

        current = (x, y)
        if self._previous_gt is not None:
            self._distance_travelled += math.hypot(
                x - self._previous_gt[0],
                y - self._previous_gt[1],
            )
        self._previous_gt = current

    def _target_path_callback(self, message: Path) -> None:
        self._target_path = [
            (
                pose.pose.position.x,
                pose.pose.position.y,
            )
            for pose in message.poses
        ]

        self._target_path_length = 0.0
        for index in range(1, len(self._target_path)):
            self._target_path_length += math.hypot(
                self._target_path[index][0] - self._target_path[index - 1][0],
                self._target_path[index][1] - self._target_path[index - 1][1],
            )

        # /target_speeds is indexed to /target_path. Keep the full profile so
        # the logger can report the speed target at the vehicle's current
        # path index instead of always reporting element zero.
        # Keep a global union of generated reference points for a complete
        # planned-vs-travelled plot after the local path moves with the car.
        for point in self._target_path:
            if not self._reference_history or math.hypot(
                point[0] - self._reference_history[-1][0],
                point[1] - self._reference_history[-1][1],
            ) > 0.25:
                self._reference_history.append(point)  


    def _cmd_callback(self, message: AckermannDriveStamped) -> None:
        self._steering = message.drive.steering_angle
        self._command_speed = message.drive.speed
        self._command_acceleration = message.drive.acceleration

    def _speed_callback(self, message: Float64MultiArray) -> None:
        self._target_speeds = [float(value) for value in message.data]
        self._target_speed = (
            self._target_speeds[0] if self._target_speeds else None
        )

    def _landmark_callback(self, message: ConeArrayWithCovariance) -> None:
        self._blue = len(message.blue_cones)
        self._yellow = len(message.yellow_cones)

    def _status_callback(self, message: String) -> None:
        self._corridor_status = message.data

    def _corridor_callback(self, message: Float64MultiArray) -> None:
        if len(message.data) % 8 != 0:
            return

        self._corridor_samples = len(message.data) // 8
        self._corridor_valid = 0
        widths = []

        for index in range(0, len(message.data), 8):
            y_min = message.data[index + 5]
            y_max = message.data[index + 6]

            if message.data[index + 7] > 0.5:
                self._corridor_valid += 1
                widths.append(y_max - y_min)

        self._corridor_width = (
            sum(widths) / len(widths) if widths else 0.0
        )

    def _mission_callback(self, message: Bool) -> None:
        if message.data:
            self._lap_completed = True

    def _nearest_target_index(self) -> int:
        if not self._target_path:
            return 0

        best_index = 0
        best_distance = float("inf")
        for index, point in enumerate(self._target_path):
            distance = math.hypot(
                point[0] - self._x,
                point[1] - self._y,
            )
            if distance < best_distance:
                best_distance = distance
                best_index = index
        return best_index

    def _reference_error(self) -> Tuple[float, float, float]:
        if len(self._target_path) < 2 or not self._frame_aligned:
            return 0.0, 0.0, 0.0

        # Compare the reference in SLAM/map coordinates. Ground truth itself
        # remains in simulator coordinates for distance-travelled logging.
        point = (self._gt_in_map_x, self._gt_in_map_y)
        best_error = float("inf")
        best_projection = self._target_path[0]

        for index in range(len(self._target_path) - 1):
            error, projection = self._point_segment_distance(
                point,
                self._target_path[index],
                self._target_path[index + 1],
            )
            if error < best_error:
                best_error = error
                best_projection = projection

        return best_projection[0], best_projection[1], best_error

    def _write_row(self) -> None:
        target = (
            ""
            if self._target_speed is None
            else self._target_speed
        )

        target_x, target_y, cte = self._reference_error()

        # Report the target speed associated with the closest current path
        # point, matching the controller's path indexing.
        target_index = self._nearest_target_index()
        if self._target_speeds and target_index < len(self._target_speeds):
            self._target_speed = self._target_speeds[target_index]
        elif not self._target_speeds:
            self._target_speed = None

        valid_fraction = (
            self._corridor_valid / self._corridor_samples
            if self._corridor_samples
            else 0.0
        )

        slam_gt_error = (
            math.hypot(
                self._x - self._gt_in_map_x,
                self._y - self._gt_in_map_y,
            )
            if self._frame_aligned else 0.0
        )

        with self._path.open(
            "a", newline="", encoding="utf-8"
        ) as output_file:
            writer = csv.DictWriter(
                output_file,
                fieldnames=self._FIELDS,
            )

            if self._path.stat().st_size == 0:
                writer.writeheader()

            writer.writerow(
                {
                    "timestamp_ns": self.get_clock().now().nanoseconds,
                    "x": self._x,
                    "y": self._y,
                    "yaw": self._yaw,
                    "actual_speed_mps": self._actual_speed,
                    "ground_truth_x": self._ground_truth_x,
                    "ground_truth_y": self._ground_truth_y,
                    "ground_truth_speed_mps": self._ground_truth_speed,
                    "slam_gt_error_m": slam_gt_error,
                    "target_x": target_x,
                    "target_y": target_y,
                    "cross_track_error_m": cte,
                    "distance_travelled_m": self._distance_travelled,
                    "target_speed_mps": target,
                    "steering_angle_rad": self._steering,
                    "command_speed_mps": self._command_speed,
                    "command_acceleration_mps2": self._command_acceleration,
                    "target_path_points": len(self._target_path),
                    "target_path_length_m": self._target_path_length,
                    "blue_landmarks": self._blue,
                    "yellow_landmarks": self._yellow,
                    "total_landmarks": self._blue + self._yellow,
                    "corridor_samples": self._corridor_samples,
                    "corridor_valid_samples": self._corridor_valid,
                    "corridor_valid_fraction": valid_fraction,
                    "mean_corridor_width_m": self._corridor_width,
                    "corridor_status": self._corridor_status,
                    "lap_completed": int(self._lap_completed),
                }
            )


def main(args=None) -> None:
    rclpy.init(args=args)
    node = RunMetricsLogger()
    try:
        rclpy.spin(node)
    finally:
        reference_path = node._path.parent / "generated_reference_history.csv"
        with reference_path.open("w", newline="", encoding="utf-8") as output_file:
            writer = csv.writer(output_file)
            writer.writerow(["x", "y"])
            writer.writerows(node._reference_history)

        node.destroy_node()
        rclpy.shutdown()
