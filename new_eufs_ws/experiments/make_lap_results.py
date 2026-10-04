#!/usr/bin/env python3
"""Generate presentation-ready trajectory and performance results."""

from __future__ import annotations

import csv
import json
import math
from pathlib import Path
import sys

import matplotlib.pyplot as plt


def load_rows(csv_path: Path):
    with csv_path.open("r", encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f))


def f(row, key):
    value = row.get(key, "")
    if value in ("", None):
        return float("nan")
    return float(value)


def finite(values):
    return [v for v in values if math.isfinite(v)]


def rms(values):
    values = finite(values)
    return math.sqrt(sum(v * v for v in values) / len(values)) if values else None


def percentile(values, fraction):
    values = sorted(finite(values))
    if not values:
        return None
    index = min(len(values) - 1, int(round((len(values) - 1) * fraction)))
    return values[index]


def path_length(xs, ys):
    total = 0.0
    for i in range(1, len(xs)):
        total += math.hypot(xs[i] - xs[i - 1], ys[i] - ys[i - 1])
    return total


def main() -> int:
    csv_path = Path(
        sys.argv[1]
        if len(sys.argv) > 1
        else "experiments/raw/run_metrics.csv"
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
    times = [
        (int(float(r["timestamp_ns"])) - t0) / 1e9
        for r in rows
    ]

    gt_x = [f(r, "ground_truth_x") for r in rows]
    gt_y = [f(r, "ground_truth_y") for r in rows]
    slam_x = [f(r, "x") for r in rows]
    slam_y = [f(r, "y") for r in rows]

    actual_speed = [
        f(r, "ground_truth_speed_mps")
        for r in rows
    ]
    target_speed = [
        f(r, "target_speed_mps")
        for r in rows
    ]
    command_speed = [
        f(r, "command_speed_mps")
        for r in rows
    ]
    command_accel = [
        f(r, "command_acceleration_mps2")
        for r in rows
    ]
    steering = [
        f(r, "steering_angle_rad")
        for r in rows
    ]
    cte = [
        f(r, "cross_track_error_m")
        for r in rows
    ]
    slam_gt = [
        f(r, "slam_gt_error_m")
        for r in rows
    ]
    corridor_width = [
        f(r, "mean_corridor_width_m")
        for r in rows
    ]
    corridor_fraction = [
        f(r, "corridor_valid_fraction")
        for r in rows
    ]

    distance_travelled = max(
        f(rows[-1], "distance_travelled_m"),
        0.0,
    )
    duration = times[-1]

    actual_valid = finite(actual_speed)
    target_valid = finite(target_speed)
    paired_speed = [
        a - b
        for a, b in zip(actual_speed, target_speed)
        if math.isfinite(a) and math.isfinite(b)
    ]

    cte_valid = finite(cte)
    slam_valid = finite(slam_gt)
    steering_valid = finite(steering)
    accel_valid = finite(command_accel)

    lap_completed = int(float(rows[-1]["lap_completed"]))

    reference_history = csv_path.parent / "generated_reference_history.csv"
    ref_x = []
    ref_y = []
    if reference_history.exists():
        with reference_history.open(
            "r", encoding="utf-8", newline=""
        ) as fref:
            for row in csv.DictReader(fref):
                ref_x.append(float(row["x"]))
                ref_y.append(float(row["y"]))

    # If no complete reference history was saved, plot the instantaneous
    # target projections as a fallback.
    target_x = [f(r, "target_x") for r in rows]
    target_y = [f(r, "target_y") for r in rows]

    reference_distance = (
        path_length(ref_x, ref_y)
        if len(ref_x) >= 2
        else path_length(
            finite(target_x),
            finite(target_y),
        )
    )

    summary = {
        "lap_completed": bool(lap_completed),
        "duration_s": duration,
        "distance_travelled_m": distance_travelled,
        "reference_path_length_m": reference_distance,
        "max_speed_mps": max(actual_valid) if actual_valid else None,
        "mean_speed_mps": (
            sum(actual_valid) / len(actual_valid)
            if actual_valid else None
        ),
        "mean_target_speed_mps": (
            sum(target_valid) / len(target_valid)
            if target_valid else None
        ),
        "mean_abs_speed_error_mps": (
            sum(abs(v) for v in paired_speed) / len(paired_speed)
            if paired_speed else None
        ),
        "speed_rmse_mps": rms(paired_speed),
        "max_command_speed_mps": (
            max(finite(command_speed))
            if finite(command_speed)
            else None
        ),
        "max_command_acceleration_mps2": (
            max(accel_valid) if accel_valid else None
        ),
        "max_abs_steering_rad": (
            max(abs(v) for v in steering_valid)
            if steering_valid else None
        ),
        "steering_rms_rad": rms(steering_valid),
        "mean_cross_track_error_m": (
            sum(cte_valid) / len(cte_valid)
            if cte_valid else None
        ),
        "cross_track_rmse_m": rms(cte_valid),
        "max_cross_track_error_m": (
            max(cte_valid) if cte_valid else None
        ),
        "cross_track_95th_percentile_m": percentile(cte_valid, 0.95),
        "mean_slam_ground_truth_error_m": (
            sum(slam_valid) / len(slam_valid)
            if slam_valid else None
        ),
        "slam_rmse_m": rms(slam_valid),
        "max_slam_ground_truth_error_m": (
            max(slam_valid) if slam_valid else None
        ),
        "mean_corridor_width_m": (
            sum(finite(corridor_width)) / len(finite(corridor_width))
            if finite(corridor_width) else None
        ),
        "mean_corridor_validity_fraction": (
            sum(finite(corridor_fraction)) / len(finite(corridor_fraction))
            if finite(corridor_fraction) else None
        ),
        "min_corridor_validity_fraction": (
            min(finite(corridor_fraction))
            if finite(corridor_fraction) else None
        ),
        "mean_target_path_points": (
            sum(f(r, "target_path_points") for r in rows)
            / len(rows)
        ),
        "max_target_path_points": max(
            f(r, "target_path_points") for r in rows
        ),
        "blue_landmarks_mean": sum(
            f(r, "blue_landmarks") for r in rows
        ) / len(rows),
        "yellow_landmarks_mean": sum(
            f(r, "yellow_landmarks") for r in rows
        ) / len(rows),
        "logged_samples": len(rows),
    }

    (out_dir / "summary.json").write_text(
        json.dumps(summary, indent=2),
        encoding="utf-8",
    )

    with (out_dir / "summary.csv").open(
        "w", encoding="utf-8", newline=""
    ) as fsummary:
        writer = csv.writer(fsummary)
        writer.writerow(["metric", "value"])
        for key, value in summary.items():
            writer.writerow([key, value])

    # 1. Full generated reference vs vehicle trajectory.
    plt.figure(figsize=(10, 7))
    if ref_x:
        plt.plot(ref_x, ref_y, label="Generated reference trajectory")
    else:
        valid_ref = [
            i for i, (x, y) in enumerate(zip(target_x, target_y))
            if math.isfinite(x) and math.isfinite(y)
        ]
        if valid_ref:
            plt.plot(
                [target_x[i] for i in valid_ref],
                [target_y[i] for i in valid_ref],
                label="Generated target trajectory",
            )

    plt.plot(gt_x, gt_y, label="Vehicle trajectory (ground truth)")
    plt.plot(slam_x, slam_y, label="SLAM trajectory")
    plt.xlabel("X position (m)")
    plt.ylabel("Y position (m)")
    plt.title("Generated Trajectory vs Vehicle Trajectory")
    plt.axis("equal")
    plt.grid(True, alpha=0.3)
    plt.legend()
    plt.tight_layout()
    plt.savefig(
        out_dir / "trajectory_comparison.png",
        dpi=200,
    )
    plt.close()

    # 2. Speed tracking.
    plt.figure(figsize=(10, 5))
    plt.plot(times, actual_speed, label="Actual speed")
    plt.plot(times, target_speed, label="Target speed")
    plt.plot(times, command_speed, label="Commanded speed", alpha=0.7)
    plt.xlabel("Time (s)")
    plt.ylabel("Speed (m/s)")
    plt.title("Speed Tracking")
    plt.grid(True, alpha=0.3)
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_dir / "speed_tracking.png", dpi=200)
    plt.close()

    # 3. Tracking/localization error.
    plt.figure(figsize=(10, 5))
    plt.plot(times, cte, label="Cross-track error")
    plt.plot(times, slam_gt, label="SLAM vs ground-truth error")
    plt.xlabel("Time (s)")
    plt.ylabel("Error (m)")
    plt.title("Tracking and Localization Error")
    plt.grid(True, alpha=0.3)
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_dir / "tracking_errors.png", dpi=200)
    plt.close()

    # 4. Steering behaviour.
    plt.figure(figsize=(10, 5))
    plt.plot(times, steering)
    plt.xlabel("Time (s)")
    plt.ylabel("Steering angle (rad)")
    plt.title("Steering Command")
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_dir / "steering_behavior.png", dpi=200)
    plt.close()

    # 5. Uncertainty/corridor behaviour.
    plt.figure(figsize=(10, 5))
    plt.plot(
        times,
        corridor_width,
        label="Mean corridor width",
    )
    plt.plot(
        times,
        corridor_fraction,
        label="Corridor validity fraction",
    )
    plt.xlabel("Time (s)")
    plt.ylabel("Value")
    plt.title("Uncertainty Corridor Quality")
    plt.grid(True, alpha=0.3)
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_dir / "corridor_quality.png", dpi=200)
    plt.close()

    print("\n=== FINAL EXPERIMENT RESULTS ===")
    for key, value in summary.items():
        print(f"{key:38s}: {value}")
    print(f"\nResults written to: {out_dir}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
