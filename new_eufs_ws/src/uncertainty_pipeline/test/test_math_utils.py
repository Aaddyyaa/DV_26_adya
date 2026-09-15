import math
import unittest

from uncertainty_pipeline.math_utils import (
    BoundarySample,
    build_corridor,
    interpolate_boundary,
    is_valid_covariance_2,
    se2_transform_covariance,
    weighted_pose_distribution,
    wrap_to_pi,
)


class MathUtilsTest(unittest.TestCase):
    def test_covariance_validation_rejects_non_psd_and_asymmetric_inputs(self):
        self.assertTrue(is_valid_covariance_2(((1.0, 0.2), (0.2, 1.0))))
        self.assertFalse(is_valid_covariance_2(((1.0, 2.0), (2.0, 1.0))))
        self.assertFalse(is_valid_covariance_2(((1.0, 0.1), (0.2, 1.0))))
        self.assertFalse(is_valid_covariance_2(((math.nan, 0.0), (0.0, 1.0))))

    def test_angle_wrapping_and_weighted_heading(self):
        self.assertAlmostEqual(wrap_to_pi(3.0 * math.pi), math.pi)
        mean, _ = weighted_pose_distribution([
            (0.5, 0.0, 0.0, math.pi - 0.1, ((0.0, 0.0, 0.0),) * 3),
            (0.5, 0.0, 0.0, -math.pi + 0.1, ((0.0, 0.0, 0.0),) * 3),
        ])
        self.assertAlmostEqual(abs(mean[2]), math.pi, places=6)

    def test_se2_covariance_propagation(self):
        world, covariance = se2_transform_covariance(
            (1.0, 0.0), ((0.04, 0.0), (0.0, 0.01)),
            (0.0, 0.0, math.pi / 2.0),
            ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0), (0.0, 0.0, 0.0)))
        self.assertAlmostEqual(world[0], 0.0, places=6)
        self.assertAlmostEqual(world[1], 1.0, places=6)
        self.assertAlmostEqual(covariance[0][0], 0.01, places=6)
        self.assertAlmostEqual(covariance[1][1], 0.04, places=6)

    def test_corridor_and_collapse_detection(self):
        lower = [BoundarySample(0.0, -2.0, 0.1), BoundarySample(2.0, -2.0, 0.1)]
        upper = [BoundarySample(0.0, 2.0, 0.1), BoundarySample(2.0, 2.0, 0.1)]
        safe = build_corridor(lower, upper, 2.0, 1.0)
        self.assertTrue(all(sample.valid for sample in safe))
        self.assertAlmostEqual(safe[0].y_min, -1.8)
        self.assertAlmostEqual(safe[0].y_max, 1.8)
        collapsed = build_corridor(
            [BoundarySample(0.0, -1.0, 1.0), BoundarySample(2.0, -1.0, 1.0)],
            [BoundarySample(0.0, 1.0, 1.0), BoundarySample(2.0, 1.0, 1.0)], 2.0, 1.0)
        self.assertTrue(all(not sample.valid for sample in collapsed))

    def test_boundary_interpolation_propagates_covariance(self):
        boundary = [BoundarySample(0.0, -2.0, 1.0), BoundarySample(2.0, 2.0, 1.0)]
        sample = interpolate_boundary(boundary, 1.0)
        self.assertIsNotNone(sample)
        assert sample is not None
        self.assertAlmostEqual(sample.mean, 0.0)
        self.assertAlmostEqual(sample.sigma, math.sqrt(0.5))

    def test_zero_covariance_is_deterministic(self):
        lower = [BoundarySample(0.0, -2.0, 0.0), BoundarySample(1.0, -2.0, 0.0)]
        upper = [BoundarySample(0.0, 2.0, 0.0), BoundarySample(1.0, 2.0, 0.0)]
        sample = build_corridor(lower, upper, 3.0, 1.0)[0]
        self.assertEqual((sample.y_min, sample.y_max), (-2.0, 2.0))


if __name__ == '__main__':
    unittest.main()
