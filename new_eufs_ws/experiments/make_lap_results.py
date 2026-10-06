#!/usr/bin/env python3
"""Generate trustworthy EUFS lap metrics and research-ready figures.

The logger records a rolling local target path, so this script does not treat
the concatenation of local paths as a physical closed-track reference length.
Trajectory, speed, steering, localization and uncertainty metrics are computed
only from finite logged samples.
"""

from __future__ import annotations

import csv
import json
import math
from pathlib import Path
import sys

import matplotlib.pyplot as plt


def load_rows(path: Path):
    with path.open("r", encoding="utf-8", newline="") as handle:
        return list(csv.DictReader(handle))


def value(row, key):
    raw = row.get(key, "")
    if raw in ("", None):
        return float("nan")
    try:
        return float(raw)
    except (TypeError, ValueError):
        return float("nan")


def finite(values):
    return [v for v in values if math.isfinite(v)]


def mean(values):
    values = finite(values)
    return sum(values) / len(values) if values else None


def rms(values):
    values = finite(values)
    return math.sqrt(sum(v * v for v in values) / len(values)) if values else None


def percentile(values, fraction):
    values = sorted(finite(values))
    if not values:
        return None
    index = int(round((len(values) - 1) * fraction))
    return values[index]


def polyline_length(xs, ys):
    total = 0.0
    for i in range(1, min(len(xs), len(ys))):
        if all(math.isfinite(v) for v in (xs[i - 1], ys[i - 1], xs[i], ys[i])):
            total += math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1])
    return total


def savefig(out_dir, name):
    plt.tight_layout()
    plt.savefig(out_dir / name, dpi=220, bbox_inches="tight")
    plt.close()


def main() -> int:
    csv_path = Path(
        sys.argv[1] if len(sys.argv) > 1 else "experiments/raw/run_metrics.csv"
    )

    if not csv_path.exists():
        print(f"ERROR: results file not found: {csv_path}")
        return 1

    rows = load_rows(csv_path)
    if len(rows) < 2:
        print("ERROR: not enough logged samples.")
        return 1

    out_dir = csv_path.parent.parent / "results"
    out_dir.mkdir(parents=True, exist_ok=True)

    t0 = int(float(rows[0]["timestamp_ns"]))
    times = [(int(float(r["timestamp_ns"])) - t0) / 1e9 for r in rows]

    gt_x = [value(r, "ground_truth_x") for r in rows]
    gt_y = [value(r, "ground_truth_y") for r in rows]
    slam_x = [value(r, "x") for r in rows]
    slam_y = [value(r, "y") for r in rows]
    actual_speed = [value(r, "ground_truth_speed_mps") for r in rows]
    target_speed = [value(r, "target_speed_mps") for r in rows]
    command_speed = [value(r, "command_speed_mps") for r in rows]
    command_accel = [value(r, "command_acceleration_mps2") for r in rows]
    steering = [value(r, "steering_angle_rad") for r in rows]
    cte = [value(r, "cross_track_error_m") for r in rows]
    slam_error = [value(r, "slam_gt_error_m") for r in rows]
    corridor_width = [value(r, "mean_corridor_width_m") for r in rows]
    corridor_fraction = [value(r, "corridor_valid_fraction") for r in rows]
    path_points = [value(r, "target_path_points") for r in rows]
    blue = [value(r, "blue_landmarks") for r in rows]
    yellow = [value(r, "yellow_landmarks") for r in rows]

    distance_travelled = max(value(rows[-1], "distance_travelled_m"), 0.0)
    duration = max(times[-1], 0.0)
    completed = bool(int(value(rows[-1], "lap_completed") or 0))

    speed_pairs = [
        a - b for a, b in zip(actual_speed, target_speed)
        if math.isfinite(a) and math.isfinite(b)
    ]

    reference_history = csv_path.parent / "generated_reference_history.csv"
    ref_x, ref_y = [], []
    if reference_history.exists():
        with reference_history.open("r", encoding="utf-8", newline="") as handle:
            for row in csv.DictReader(handle):
                x = value(row, "x")
                y = value(row, "y")
                if math.isfinite(x) and math.isfinite(y):
                    ref_x.append(x)
                    ref_y.append(y)

    # The latest local path is a planning horizon, not a full circuit.
    local_reference_length = polyline_length(ref_x, ref_y)

    summary = {
        "lap_completed": completed,
        "duration_s": duration,
        "distance_travelled_m": distance_travelled,
        "local_reference_path_length_m": local_reference_length,
        "max_speed_mps": max(finite(actual_speed), default=float("nan")),
        "mean_speed_mps": mean(actual_speed),
        "mean_target_speed_mps": mean(target_speed),
        "mean_abs_speed_error_mps": mean([abs(v) for v in speed_pairs]),
        "speed_rmse_mps": rms(speed_pairs),
        "max_command_speed_mps": max(finite(command_speed), default=float("nan")),
        "max_command_acceleration_mps2": max(finite(command_accel), default=float("nan")),
        "min_command_acceleration_mps2": min(finite(command_accel), default=float("nan")),
        "max_abs_steering_rad": max([abs(v) for v in finite(steering)], default=float("nan")),
        "steering_rms_rad": rms(steering),
        "mean_cross_track_error_m": mean(cte),
        "cross_track_rmse_m": rms(cte),
        "max_cross_track_error_m": max(finite(cte), default=float("nan")),
        "cross_track_95th_percentile_m": percentile(cte, 0.95),
        "mean_slam_ground_truth_error_m": mean(slam_error),
        "slam_rmse_m": rms(slam_error),
        "max_slam_ground_truth_error_m": max(finite(slam_error), default=float("nan")),
        "mean_corridor_width_m": mean(corridor_width),
        "mean_corridor_validity_fraction": mean(corridor_fraction),
        "min_corridor_validity_fraction": min(finite(corridor_fraction), default=float("nan")),
        "mean_target_path_points": mean(path_points),
        "max_target_path_points": max(finite(path_points), default=float("nan")),
        "blue_landmarks_mean": mean(blue),
        "yellow_landmarks_mean": mean(yellow),
        "logged_samples": len(rows),
    }

    # JSON cannot represent NaN reliably for research pipelines; convert
    # non-finite values to null.
    clean_summary = {
        key: (None if isinstance(val, float) and not math.isfinite(val) else val)
        for key, val in summary.items()
    }

    (out_dir / "summary.json").write_text(
        json.dumps(clean_summary, indent=2), encoding="utf-8"
    )
    with (out_dir / "summary.csv").open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["metric", "value"])
        writer.writerows(clean_summary.items())

    # 1. Ground-truth and SLAM trajectories.
    plt.figure(figsize=(10, 7))
    if ref_x:
        plt.plot(ref_x, ref_y, label="Latest local planner horizon")
    plt.plot(gt_x, gt_y, label="Ground-truth trajectory")
    plt.plot(slam_x, slam_y, label="SLAM trajectory")
    plt.xlabel("X position (m)")
    plt.ylabel("Y position (m)")
    plt.title("EUFS Trajectory and Local Planning Horizon")
    plt.axis("equal")
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "trajectory_comparison.png")

    # 2. Speed tracking.
    plt.figure(figsize=(10, 5))
    plt.plot(times, actual_speed, label="Actual speed")
    plt.plot(times, target_speed, label="Planner target speed")
    plt.plot(times, command_speed, label="Command speed")
    plt.xlabel("Time (s)")
    plt.ylabel("Speed (m/s)")
    plt.title("Speed Tracking")
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "speed_tracking.png")

    # 3. Speed error.
    plt.figure(figsize=(10, 5))
    plt.plot(times, [v if math.isfinite(v) else float("nan") for v in speed_pairs] + [float("nan")] * max(0, len(times) - len(speed_pairs)))
    plt.axhline(0.0, linewidth=0.8)
    plt.xlabel("Time (s)")
    plt.ylabel("Actual - target speed (m/s)")
    plt.title("Speed Tracking Error")
    plt.grid(True, alpha=0.3)
    savefig(out_dir, "speed_error.png")

    # 4. Cross-track and SLAM error.
    plt.figure(figsize=(10, 5))
    plt.plot(times, cte, label="Cross-track error")
    plt.plot(times, slam_error, label="SLAM vs ground truth")
    plt.xlabel("Time (s)")
    plt.ylabel("Error (m)")
    plt.title("Tracking and Localization Error")
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "tracking_errors.png")

    # 5. Steering.
    plt.figure(figsize=(10, 5))
    plt.plot(times, steering, label="Steering")
    plt.axhline(0.5, linewidth=0.8)
    plt.axhline(-0.5, linewidth=0.8)
    plt.xlabel("Time (s)")
    plt.ylabel("Steering angle (rad)")
    plt.title("Steering Command and Saturation")
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "steering_behavior.png")

    # 6. Longitudinal commands.
    plt.figure(figsize=(10, 5))
    plt.plot(times, command_accel, label="Command acceleration")
    plt.xlabel("Time (s)")
    plt.ylabel("Acceleration (m/s²)")
    plt.title("Longitudinal Control Command")
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "acceleration_command.png")

    # 7. Uncertainty corridor.
    plt.figure(figsize=(10, 5))
    plt.plot(times, corridor_width, label="Mean corridor width (m)")
    plt.xlabel("Time (s)")
    plt.ylabel("Width (m)")
    plt.title("Uncertainty Corridor Width")
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "corridor_width.png")

    plt.figure(figsize=(10, 5))
    plt.plot(times, corridor_fraction, label="Valid corridor fraction")
    plt.axhline(1.0, linewidth=0.8)
    plt.xlabel("Time (s)")
    plt.ylabel("Valid fraction")
    plt.title("Uncertainty Corridor Validity")
    plt.ylim(-0.05, 1.05)
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "corridor_validity.png")

    # 8. Perception/planning observability.
    plt.figure(figsize=(10, 5))
    plt.plot(times, blue, label="Blue landmarks")
    plt.plot(times, yellow, label="Yellow landmarks")
    plt.xlabel("Time (s)")
    plt.ylabel("Detected landmarks")
    plt.title("Boundary Landmark Availability")
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "landmark_counts.png")

    plt.figure(figsize=(10, 5))
    plt.plot(times, path_points, label="Target path points")
    plt.xlabel("Time (s)")
    plt.ylabel("Points")
    plt.title("Local Planner Horizon Size")
    plt.grid(True, alpha=0.3)
    plt.legend()
    savefig(out_dir, "planner_horizon.png")

    print("\n=== FINAL EXPERIMENT RESULTS ===")
    for key, val in clean_summary.items():
        print(f"{key:38s}: {val}")
    print(f"\nResults written to: {out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
