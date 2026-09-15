# Uncertainty-Aware Perception and Safe Corridor

## Actual runtime path

The active launch is `my_controller_pkg/launch/bringup.launch.py`. It starts
`perception_pkg/perception_node`, `my_slam_pkg/eufs_slam_node`, and
`fsd_planner/planner_node`. `eufs_slam_node` is the repository's FastSLAM 2.0
implementation, despite the package description mentioning EKF-SLAM. The
root-level `new_eufs_ws/src/Slam.cpp` is an unlaunched EKF candidate.

The baseline path is preserved:

`/zed/{left,right}/image_rect_color` -> `/perception/cones` (`eufs_msgs/ConeArray`)
-> `/planning/cones` -> `fsd_planner` -> `/target_path` -> controller.

The uncertainty extension is:

`/perception/cones_with_covariance` (`eufs_msgs/ConeArrayWithCovariance`)
-> FastSLAM -> `/slam/landmarks` (`ConeArrayWithCovariance`) and
`/slam/odom` (`nav_msgs/Odometry` with planar pose covariance) ->
`/uncertainty/corridor` (`std_msgs/Float64MultiArray`) and
`/uncertainty/markers` (`visualization_msgs/MarkerArray`).

`/camera_0/cones` has no publisher, subscriber, launch reference, or message
definition in this branch's checked-in source. It is not part of the active
runtime path. It must be checked with `ros2 topic info -v /camera_0/cones` on a
running simulator before treating it as an interface.

`ConeArray` contains a header plus arrays of `geometry_msgs/Point` for blue,
yellow, orange, large-orange, and unknown cones. `ConeArrayWithCovariance` has
the same colour groups using `ConeWithCovariance`, whose `covariance` is a
row-major `[xx, xy, yx, yy]` 2D matrix.

## Measurement and SLAM uncertainty

The perception node retains the existing detector, keypoint model, NCC stereo
matching, and temporal filter. It does not apply one fixed covariance to every
cone. For a valid cone, it uses the actual matched disparity values, NCC scores,
keypoint visibilities, valid match count, and calibration values. The two named ROS
parameters `pixel_sigma_at_unit_quality` and `min_disparity_sigma_px` are
documented model assumptions for the sub-pixel floor when image evidence alone
cannot determine it.

With `Z = fx B / d`, `X = (u-cx)Z/fx`, and `Y = (v-cy)Z/fy`, the output uses
the first-order Jacobian with respect to `(u, v, d)`. In particular,
`dX/du = Z/fx`, `dX/dd = -X/d`, `dY/dv = Z/fy`, and `dY/dd = -Y/d`.
The resulting finite, symmetric PSD 2x2 covariance travels on the companion
topic; invalid results are rejected rather than converted to a made-up value.

FastSLAM converts it to range-bearing covariance with the Jacobian of
`(r, beta) = (hypot(x,y), atan2(y,x))`, then uses that matrix in both the
innovation covariance and new-landmark initialization. New landmark covariance
is `Gz R Gz^T + Gpose P Gpose^T`. This uses the estimator's own particle pose
covariance and does not invent a landmark/pose cross-covariance.

For pose output, `/slam/odom` now reports a weighted particle mean, with yaw
from weighted sine/cosine, and total covariance
`sum_i w_i (P_i + (mu_i-mu)(mu_i-mu)^T)`. Landmark aggregation follows the
same total-covariance rule over same-colour particle-map matches near the
reference landmark.

## Boundary and corridor model

`corridor_node` transforms map landmarks into the current vehicle frame. For
the inverse transform it propagates map-landmark covariance and pose covariance
with the SE(2) Jacobians; no cross term is used because FastSLAM does not
provide one. Blue and yellow landmarks supply the two observed boundaries.
At each local longitudinal coordinate `s`, the lower and upper boundaries are
chosen by geometry, not an assumed colour-side convention. Piecewise-linear
interpolation gives mean lateral position and first-order variance
`(1-a)^2 sigma_0^2 + a^2 sigma_1^2`.

For each `k` in 1, 2, 3, the inner limits are
`y_min = lower_mean + k lower_sigma` and
`y_max = upper_mean - k upper_sigma`. A sample is invalid when any value is
non-finite or `y_min >= y_max`; invalid regions are published explicitly and
are omitted from the corridor centreline. The existing deterministic planner
is left connected to `/planning/cones` until it is deliberately adapted to
consume this safety-gated centreline.

## RViz and data

`/uncertainty/markers` displays blue/yellow boundary means and all three
`k=1,2,3` corridor envelopes. The selected `safety_k` is available on
`/uncertainty/corridor` as repeated rows:

`[s, lower_mean, lower_sigma, upper_mean, upper_sigma, y_min, y_max, valid]`.

`experiment_logger` records only received ROS data in
`experiments/raw/corridor.csv`. `analyse_corridor_csv.py` computes sample
count, feasible percentage, and observed corridor/envelope widths from that
CSV. `plot_corridor_csv.py` renders a figure only when supplied with real
logged rows. The repository intentionally contains no generated measurements,
tables, or figures.

## Reproduction on a ROS 2 Humble / EUFS host

```bash
cd new_eufs_ws
source /opt/ros/humble/setup.bash
colcon build --packages-select perception_pkg my_slam_pkg uncertainty_pipeline my_controller_pkg
source install/setup.bash
ros2 launch my_controller_pkg bringup.launch.py
```

This checkout records `eufs_msgs` and `eufs_sim` as gitlinks, but does not
include a `.gitmodules` URL mapping and the local directories are empty. A
build host must first populate those exact dependency revisions from the
project's approved source; this implementation does not guess or rewrite that
dependency configuration.

In another terminal, inspect live interfaces before recording:

```bash
ros2 topic info -v /perception/cones
ros2 topic info -v /perception/cones_with_covariance
ros2 topic info -v /slam/landmarks
ros2 topic echo --once /slam/odom
```

For experiment A or C, run the logger while the simulation is active:

```bash
ros2 run uncertainty_pipeline experiment_logger --ros-args \
  -p output_csv:=experiments/raw/corridor_k2.csv -p safety_k:=2.0 -p noise_level:=0.0
python3 src/uncertainty_pipeline/scripts/analyse_corridor_csv.py experiments/raw/corridor_k2.csv
python3 src/uncertainty_pipeline/scripts/plot_corridor_csv.py experiments/raw/corridor_k2.csv experiments/figures/corridor_k2.png
```

For experiment B, run `noise_injector` with `noise_fraction` of 0.0, 0.1,
0.2, 0.3, and 0.4, then remap the FastSLAM covariance input to
`/experiments/perception/cones_with_covariance` only for that experiment. The
normal production launch remains on `/perception/cones_with_covariance`.

The only ground-truth topics referenced by checked-in code are
`/ground_truth/odom` and `/ground_truth/cones`. The former is currently read
by the existing controller for speed, and the latter is only consumed by the
unlaunched root-level EKF candidate. Neither is an input to this uncertainty
extension; use live topic introspection before using either as an evaluation
reference.
