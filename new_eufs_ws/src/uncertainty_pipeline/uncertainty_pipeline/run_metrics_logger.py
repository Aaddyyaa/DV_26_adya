"""ROS 2 experiment logger for reproducible autonomous-lap results."""

import csv
import math
from pathlib import Path as FilePath
from typing import Optional

import rclpy
from ackermann_msgs.msg import AckermannDriveStamped
from eufs_msgs.msg import ConeArrayWithCovariance
from nav_msgs.msg import Odometry
from rclpy.node import Node
from std_msgs.msg import Float64MultiArray, String


class RunMetricsLogger(Node):
    """Records measured runtime metrics; it never synthesizes experiment data."""

    _FIELDS = (
        'timestamp_ns', 'x', 'y', 'yaw', 'actual_speed_mps',
        'target_speed_mps', 'steering_angle_rad',
        'blue_landmarks', 'yellow_landmarks', 'total_landmarks',
        'corridor_samples', 'corridor_valid_samples', 'corridor_valid_fraction',
        'mean_corridor_width_m', 'corridor_status',
    )

    def __init__(self) -> None:
        super().__init__('run_metrics_logger')
        output_csv = self.declare_parameter(
            'output_csv', 'experiments/raw/run_metrics.csv').value
        self._path = FilePath(output_csv).expanduser()
        self._path.parent.mkdir(parents=True, exist_ok=True)
        self._x = self._y = self._yaw = 0.0
        self._actual_speed = 0.0
        self._target_speed: Optional[float] = None
        self._steering = 0.0
        self._blue = self._yellow = 0
        self._corridor_samples = 0
        self._corridor_valid = 0
        self._corridor_width = 0.0
        self._corridor_status = 'no data'
        self.create_subscription(Odometry, '/slam/odom', self._odom_callback, 20)
        self.create_subscription(AckermannDriveStamped, '/cmd', self._cmd_callback, 20)
        self.create_subscription(Float64MultiArray, '/target_speeds', self._speed_callback, 10)
        self.create_subscription(ConeArrayWithCovariance, '/slam/landmarks', self._landmark_callback, 10)
        self.create_subscription(String, '/uncertainty/status', self._status_callback, 10)
        self.create_subscription(Float64MultiArray, '/uncertainty/corridor', self._corridor_callback, 10)
        self._timer = self.create_timer(0.1, self._write_row)
        self.get_logger().info(f'Logging run metrics to {self._path}')

    def _odom_callback(self, message: Odometry) -> None:
        self._x = message.pose.pose.position.x
        self._y = message.pose.pose.position.y
        q = message.pose.pose.orientation
        self._yaw = math.atan2(2.0 * (q.w * q.z + q.x * q.y), 1.0 - 2.0 * (q.y * q.y + q.z * q.z))
        self._actual_speed = message.twist.twist.linear.x

    def _cmd_callback(self, message: AckermannDriveStamped) -> None:
        self._steering = message.drive.steering_angle

    def _speed_callback(self, message: Float64MultiArray) -> None:
        self._target_speed = float(message.data[0]) if message.data else None

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
        self._corridor_width = sum(widths) / len(widths) if widths else 0.0

    def _write_row(self) -> None:
        target = '' if self._target_speed is None else self._target_speed
        fraction = self._corridor_valid / self._corridor_samples if self._corridor_samples else 0.0
        with self._path.open('a', newline='', encoding='utf-8') as output_file:
            writer = csv.DictWriter(output_file, fieldnames=self._FIELDS)
            if self._path.stat().st_size == 0:
                writer.writeheader()
            writer.writerow({
                'timestamp_ns': self.get_clock().now().nanoseconds,
                'x': self._x, 'y': self._y, 'yaw': self._yaw,
                'actual_speed_mps': self._actual_speed,
                'target_speed_mps': target,
                'steering_angle_rad': self._steering,
                'blue_landmarks': self._blue,
                'yellow_landmarks': self._yellow,
                'total_landmarks': self._blue + self._yellow,
                'corridor_samples': self._corridor_samples,
                'corridor_valid_samples': self._corridor_valid,
                'corridor_valid_fraction': fraction,
                'mean_corridor_width_m': self._corridor_width,
                'corridor_status': self._corridor_status,
            })


def main(args=None) -> None:
    rclpy.init(args=args)
    node = RunMetricsLogger()
    try:
        rclpy.spin(node)
    finally:
        node.destroy_node()
        rclpy.shutdown()
