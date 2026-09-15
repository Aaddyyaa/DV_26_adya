"""Isolated, reproducible measurement perturbation for experiment B."""

import math
import random

from eufs_msgs.msg import ConeArrayWithCovariance, ConeWithCovariance
import rclpy
from rclpy.node import Node

from .math_utils import is_valid_covariance_2


class NoiseInjector(Node):
    """Adds zero-mean covariance-scaled noise on a separate topic only."""

    def __init__(self) -> None:
        super().__init__('uncertainty_noise_injector')
        self._noise_fraction = float(self.declare_parameter('noise_fraction', 0.0).value)
        self._seed = int(self.declare_parameter('seed', 20260916).value)
        input_topic = self.declare_parameter(
            'input_topic', '/perception/cones_with_covariance').value
        output_topic = self.declare_parameter(
            'output_topic', '/experiments/perception/cones_with_covariance').value
        if self._noise_fraction < 0.0:
            raise ValueError('noise_fraction must be non-negative')
        self._random = random.Random(self._seed)
        self.create_subscription(ConeArrayWithCovariance, input_topic, self._callback, 10)
        self._publisher = self.create_publisher(ConeArrayWithCovariance, output_topic, 10)
        self.get_logger().info(
            f'Experiment-only noise: {input_topic} -> {output_topic}; '
            f'fraction={self._noise_fraction:g}; seed={self._seed}')

    def _perturb(self, cone: ConeWithCovariance) -> ConeWithCovariance:
        output = ConeWithCovariance()
        output.point = cone.point
        covariance = ((cone.covariance[0], cone.covariance[1]),
                      (cone.covariance[2], cone.covariance[3]))
        if not is_valid_covariance_2(covariance):
            return output
        a = math.sqrt(max(0.0, covariance[0][0]))
        b = covariance[1][0] / a if a > 0.0 else 0.0
        c = math.sqrt(max(0.0, covariance[1][1] - b * b))
        scale = self._noise_fraction
        output.point.x += scale * (a * self._random.gauss(0.0, 1.0))
        output.point.y += scale * (
            b * self._random.gauss(0.0, 1.0) + c * self._random.gauss(0.0, 1.0))
        # The added independent noise is f^2 Sigma, so output covariance is
        # (1 + f^2) Sigma. This path is never used in normal operation.
        multiplier = 1.0 + scale * scale
        output.covariance = [
            multiplier * cone.covariance[0], multiplier * cone.covariance[1],
            multiplier * cone.covariance[2], multiplier * cone.covariance[3],
        ]
        return output

    def _callback(self, message: ConeArrayWithCovariance) -> None:
        output = ConeArrayWithCovariance()
        output.header = message.header
        for field in (
            'blue_cones', 'yellow_cones', 'orange_cones',
            'big_orange_cones', 'unknown_color_cones'):
            getattr(output, field).extend(self._perturb(cone) for cone in getattr(message, field))
        self._publisher.publish(output)


def main(args=None) -> None:
    rclpy.init(args=args)
    node = NoiseInjector()
    try:
        rclpy.spin(node)
    finally:
        node.destroy_node()
        rclpy.shutdown()
