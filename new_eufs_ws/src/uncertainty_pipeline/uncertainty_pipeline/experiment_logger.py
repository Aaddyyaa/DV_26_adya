"""Lightweight CSV logger for actual corridor data from a ROS run."""

import csv
from pathlib import Path as FilePath

import rclpy
from rclpy.node import Node
from std_msgs.msg import Float64MultiArray


class ExperimentLogger(Node):
    """Writes actual /uncertainty/corridor samples; it does not synthesize data."""

    _FIELDS = (
        'timestamp_ns', 's', 'lower_mean', 'lower_sigma', 'upper_mean',
        'upper_sigma', 'corridor_min', 'corridor_max', 'valid', 'noise_level', 'k', 'frame')

    def __init__(self) -> None:
        super().__init__('uncertainty_experiment_logger')
        output_csv = self.declare_parameter(
            'output_csv', 'experiments/raw/corridor.csv').value
        self._noise_level = float(self.declare_parameter('noise_level', 0.0).value)
        self._k = float(self.declare_parameter('safety_k', 2.0).value)
        self._output_path = FilePath(output_csv)
        self._output_path.parent.mkdir(parents=True, exist_ok=True)
        self._write_header = not self._output_path.exists() or self._output_path.stat().st_size == 0
        self.create_subscription(Float64MultiArray, '/uncertainty/corridor', self._callback, 10)
        self.get_logger().info(f'Logging actual corridor samples to {self._output_path}')

    def _callback(self, message: Float64MultiArray) -> None:
        if len(message.data) % 8 != 0:
            self.get_logger().warning('Ignoring malformed corridor array')
            return
        timestamp_ns = self.get_clock().now().nanoseconds
        with self._output_path.open('a', newline='', encoding='ascii') as output_file:
            writer = csv.DictWriter(output_file, fieldnames=self._FIELDS)
            if self._write_header:
                writer.writeheader()
                self._write_header = False
            for index in range(0, len(message.data), 8):
                s, lower_mean, lower_sigma, upper_mean, upper_sigma, y_min, y_max, valid = (
                    message.data[index:index + 8])
                writer.writerow({
                    'timestamp_ns': timestamp_ns,
                    's': s,
                    'lower_mean': lower_mean,
                    'lower_sigma': lower_sigma,
                    'upper_mean': upper_mean,
                    'upper_sigma': upper_sigma,
                    'corridor_min': y_min,
                    'corridor_max': y_max,
                    'valid': int(valid),
                    'noise_level': self._noise_level,
                    'k': self._k,
                    'frame': 'base_footprint',
                })


def main(args=None) -> None:
    rclpy.init(args=args)
    node = ExperimentLogger()
    try:
        rclpy.spin(node)
    finally:
        node.destroy_node()
        rclpy.shutdown()
