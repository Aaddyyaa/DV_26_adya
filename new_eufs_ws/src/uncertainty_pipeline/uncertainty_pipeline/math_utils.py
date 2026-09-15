"""Pure-Python uncertainty operations shared by ROS nodes and offline tests."""

from dataclasses import dataclass
import math
from typing import Iterable, List, Optional, Sequence, Tuple


Matrix2 = Tuple[Tuple[float, float], Tuple[float, float]]
Matrix3 = Tuple[Tuple[float, float, float], Tuple[float, float, float], Tuple[float, float, float]]
Point2 = Tuple[float, float]
Pose2 = Tuple[float, float, float]


@dataclass(frozen=True)
class Landmark:
    mean: Point2
    covariance: Matrix2
    color: int


@dataclass(frozen=True)
class BoundarySample:
    s: float
    mean: float
    sigma: float


@dataclass(frozen=True)
class CorridorSample:
    s: float
    lower_mean: float
    lower_sigma: float
    upper_mean: float
    upper_sigma: float
    y_min: float
    y_max: float
    valid: bool


def wrap_to_pi(angle: float) -> float:
    return math.atan2(math.sin(angle), math.cos(angle))


def _finite(values: Iterable[float]) -> bool:
    return all(math.isfinite(value) for value in values)


def is_valid_covariance_2(covariance: Matrix2, tolerance: float = 1e-10) -> bool:
    """Validate a finite, symmetric positive-semidefinite 2x2 covariance."""
    a, b = covariance[0]
    c, d = covariance[1]
    return (
        _finite((a, b, c, d))
        and abs(b - c) <= tolerance
        and a >= -tolerance
        and d >= -tolerance
        and a * d - b * c >= -tolerance
    )


def symmetrize_covariance_2(covariance: Matrix2) -> Matrix2:
    a, b = covariance[0]
    c, d = covariance[1]
    return ((a, 0.5 * (b + c)), (0.5 * (b + c), d))


def matmul_2(a: Matrix2, b: Matrix2) -> Matrix2:
    return (
        (a[0][0] * b[0][0] + a[0][1] * b[1][0],
         a[0][0] * b[0][1] + a[0][1] * b[1][1]),
        (a[1][0] * b[0][0] + a[1][1] * b[1][0],
         a[1][0] * b[0][1] + a[1][1] * b[1][1]),
    )


def transpose_2(a: Matrix2) -> Matrix2:
    return ((a[0][0], a[1][0]), (a[0][1], a[1][1]))


def add_2(a: Matrix2, b: Matrix2) -> Matrix2:
    return ((a[0][0] + b[0][0], a[0][1] + b[0][1]),
            (a[1][0] + b[1][0], a[1][1] + b[1][1]))


def quadratic_2(jacobian: Matrix2, covariance: Matrix2) -> Matrix2:
    return matmul_2(matmul_2(jacobian, covariance), transpose_2(jacobian))


def se2_transform_covariance(
    local_point: Point2,
    point_covariance: Matrix2,
    pose: Pose2,
    pose_covariance: Matrix3,
) -> Tuple[Point2, Matrix2]:
    """Transform a local point into world and propagate independent covariance.

    p_w = [x, y] + R(theta) p_l, and Sigma_w is J_pose Sigma_pose
    J_pose^T + R Sigma_point R^T. Cross covariance is intentionally absent:
    FastSLAM does not expose one.
    """
    if not is_valid_covariance_2(point_covariance):
        raise ValueError('invalid point covariance')
    if not _finite(value for row in pose_covariance for value in row):
        raise ValueError('non-finite pose covariance')
    x, y, theta = pose
    px, py = local_point
    c = math.cos(theta)
    s = math.sin(theta)
    world = (x + c * px - s * py, y + s * px + c * py)
    rotation = ((c, -s), (s, c))
    point_term = quadratic_2(rotation, point_covariance)
    j_pose = ((1.0, 0.0, -s * px - c * py),
              (0.0, 1.0, c * px - s * py))
    pose_term = _quadratic_2x3(j_pose, pose_covariance)
    covariance = symmetrize_covariance_2(add_2(point_term, pose_term))
    if not is_valid_covariance_2(covariance):
        raise ValueError('invalid propagated covariance')
    return world, covariance


def world_to_vehicle_covariance(
    world_point: Point2,
    point_covariance: Matrix2,
    pose: Pose2,
    pose_covariance: Matrix3,
) -> Tuple[Point2, Matrix2]:
    """Express a map landmark in the current vehicle frame with covariance."""
    if not is_valid_covariance_2(point_covariance):
        raise ValueError('invalid landmark covariance')
    if not _finite(value for row in pose_covariance for value in row):
        raise ValueError('non-finite pose covariance')
    x, y, theta = pose
    dx, dy = world_point[0] - x, world_point[1] - y
    c = math.cos(theta)
    s = math.sin(theta)
    local = (c * dx + s * dy, -s * dx + c * dy)
    rotation = ((c, s), (-s, c))
    point_term = quadratic_2(rotation, point_covariance)
    # d(R(-theta)(p_w - t))/d[x,y,theta]
    j_pose = ((-c, -s, local[1]), (s, -c, -local[0]))
    covariance = symmetrize_covariance_2(
        add_2(point_term, _quadratic_2x3(j_pose, pose_covariance)))
    if not is_valid_covariance_2(covariance):
        raise ValueError('invalid propagated covariance')
    return local, covariance


def _quadratic_2x3(jacobian: Tuple[Tuple[float, float, float], Tuple[float, float, float]],
                    covariance: Matrix3) -> Matrix2:
    output = [[0.0, 0.0], [0.0, 0.0]]
    for row in range(2):
        for column in range(2):
            output[row][column] = sum(
                jacobian[row][i] * covariance[i][j] * jacobian[column][j]
                for i in range(3) for j in range(3))
    return ((output[0][0], output[0][1]), (output[1][0], output[1][1]))


def weighted_pose_distribution(
    particles: Sequence[Tuple[float, float, float, float, Matrix3]],
) -> Tuple[Pose2, Matrix3]:
    """Weighted pose mean and total covariance for a particle distribution."""
    if not particles:
        raise ValueError('no particles')
    raw_weights = [particle[0] for particle in particles]
    if not _finite(raw_weights) or sum(raw_weights) <= 1e-12:
        weights = [1.0 / len(particles)] * len(particles)
    else:
        total = sum(raw_weights)
        weights = [weight / total for weight in raw_weights]
    x = sum(weight * particle[1] for weight, particle in zip(weights, particles))
    y = sum(weight * particle[2] for weight, particle in zip(weights, particles))
    yaw = math.atan2(
        sum(weight * math.sin(particle[3]) for weight, particle in zip(weights, particles)),
        sum(weight * math.cos(particle[3]) for weight, particle in zip(weights, particles)))
    covariance = [[0.0] * 3 for _ in range(3)]
    for weight, particle in zip(weights, particles):
        _, px, py, pyaw, internal_covariance = particle
        delta = (px - x, py - y, wrap_to_pi(pyaw - yaw))
        for row in range(3):
            for column in range(3):
                covariance[row][column] += weight * (
                    internal_covariance[row][column] + delta[row] * delta[column])
    output = tuple(tuple(row) for row in covariance)
    return (x, y, yaw), output  # type: ignore[return-value]


def local_boundary_samples(
    landmarks: Sequence[Landmark], pose: Pose2, pose_covariance: Matrix3,
    color: int,
) -> List[BoundarySample]:
    samples: List[BoundarySample] = []
    for landmark in landmarks:
        if landmark.color != color:
            continue
        local, covariance = world_to_vehicle_covariance(
            landmark.mean, landmark.covariance, pose, pose_covariance)
        lateral_variance = covariance[1][1]
        if lateral_variance < 0.0 or not math.isfinite(lateral_variance):
            continue
        samples.append(BoundarySample(local[0], local[1], math.sqrt(lateral_variance)))
    return sorted(samples, key=lambda sample: sample.s)


def interpolate_boundary(samples: Sequence[BoundarySample], s: float) -> Optional[BoundarySample]:
    """Piecewise-linear boundary mean and first-order propagated variance."""
    if len(samples) < 2 or s < samples[0].s or s > samples[-1].s:
        return None
    for first, second in zip(samples, samples[1:]):
        if first.s <= s <= second.s:
            span = second.s - first.s
            if span <= 1e-9:
                continue
            alpha = (s - first.s) / span
            mean = (1.0 - alpha) * first.mean + alpha * second.mean
            variance = ((1.0 - alpha) * first.sigma) ** 2 + (alpha * second.sigma) ** 2
            return BoundarySample(s, mean, math.sqrt(max(0.0, variance)))
    return None


def build_corridor(
    first_boundary: Sequence[BoundarySample], second_boundary: Sequence[BoundarySample],
    safety_k: float, step_m: float,
) -> List[CorridorSample]:
    """Build local longitudinal corridor samples and flag collapsed regions."""
    if safety_k < 0.0 or step_m <= 0.0 or len(first_boundary) < 2 or len(second_boundary) < 2:
        return []
    start = max(first_boundary[0].s, second_boundary[0].s)
    end = min(first_boundary[-1].s, second_boundary[-1].s)
    if end < start:
        return []
    output: List[CorridorSample] = []
    index = 0
    while True:
        s = min(end, start + index * step_m)
        first = interpolate_boundary(first_boundary, s)
        second = interpolate_boundary(second_boundary, s)
        if first is None or second is None:
            continue
        lower, upper = sorted((first, second), key=lambda sample: sample.mean)
        y_min = lower.mean + safety_k * lower.sigma
        y_max = upper.mean - safety_k * upper.sigma
        output.append(CorridorSample(
            s, lower.mean, lower.sigma, upper.mean, upper.sigma,
            y_min, y_max, math.isfinite(y_min) and math.isfinite(y_max) and y_min < y_max))
        if s >= end:
            break
        index += 1
    return output
