#!/usr/bin/env python3
"""Generate presentation-ready trajectory and performance results from run_metrics.csv."""

from __future__ import annotations

import csv
import json
from pathlib import Path
import sys

import matplotlib.pyplot as plt


def load_rows(csv_path: Path):
    with csv_path.open("r", encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f))


def f(row, key):
    value = row.get(key, "")
    return float(value) if value not in ("", None) else float("nan")


def main() -> int:
    csv_path = Path(
        sys.argv[1] if len(sys.argv) > 1
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
    times = [(int(float(r["timestamp_ns"])) - t0) / 1e9 for r in rows]

    gt_x = [f(r, "ground_truth_x") for r in rows]
    gt_y = [f(r, "ground_truth_y") for r in rows]
    slam_x = [f(r, "x") for r in rows]
    slam_y = [f(r, "y") for r in rows]
    target_x = [f(r, "target_x") for r in rows]
    target_y = [f(r, "target_y") for r in rows]

    actual_speed = [f(r, "ground_truth_speed_mps") for r in rows]
    target_speed = [f(r, "target_speed_mps") for r in rows]
    cte = [f(r, "cross_track_error_m") for r in rows]
    slam_gt = [f(r, "slam_gt_error_m") for r in rows]

    valid_ref = [
        i for i, (x, y) in enumerate(zip(target_x, target_y))
        if x == x and y == y
    ]
    speed_err = [
        abs(a - b)
        for a, b in zip(actual_speed, target_speed)
        if a == a and b == b
    ]
    cte_valid = [v for v in cte if v == v]
    slam_valid = [v for v in slam_gt if v == v]

    distance_travelled = max(
        f(rows[-1], "distance_travelled_m"), 0.0
    )
    duration = times[-1]
    lap_completed = int(float(rows[-1]["lap_completed"]))

    summary = {
        "lap_completed": bool(lap_completed),
        "duration_s": duration,
        "distance_travelled_m": distance_travelled,
        "max_ground_truth_speed_mps": max(actual_speed),
        "mean_ground_truth_speed_mps": sum(actual_speed) / len(actual_speed),
        "mean_abs_speed_error_mps": (
            sum(speed_err) / len(speed_err) if speed_err else None
        ),
        "mean_cross_track_error_m": (
            sum(cte_valid) / len(cte_valid) if cte_valid else None
        ),
        "max_cross_track_error_m": (
            max(cte_valid) if cte_valid else None
        ),
        "mean_slam_ground_truth_error_m": (
            sum(slam_valid) / len(slam_valid) if slam_valid else None
        ),
        "max_slam_ground_truth_error_m": (
            max(slam_valid) if slam_valid else None
        ),
        "logged_samples": len(rows),
    }

    (out_dir / "summary.json").write_text(
        json.dumps(summary, indent=2),
        encoding="utf-8",
    )

    with (out_dir / "summary.csv").open(
        "w", encoding="utf-8", newline=""
    ) as f:
        writer = csv.writer(f)
        writer.writerow(["metric", "value"])
        for key, value in summary.items():
            writer.writerow([key, value])

    plt.figure(figsize=(10, 7))
    plt.plot(gt_x, gt_y, label="Car trajectory (ground truth)")
    if valid_ref:
        plt.plot(
            [target_x[i] for i in valid_ref],
            [target_y[i] for i in valid_ref],
            label="Generated target trajectory",
        )
    plt.plot(slam_x, slam_y, label="SLAM trajectory")
    plt.xlabel("X position (m)")
    plt.ylabel("Y position (m)")
    plt.title("Generated Trajectory vs Vehicle Trajectory")
    plt.axis("equal")
    plt.grid(True, alpha=0.3)
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_dir / "trajectory_comparison.png", dpi=180)
    plt.close()

    plt.figure(figsize=(10, 5))
    plt.plot(times, actual_speed, label="Actual speed")
    plt.plot(times, target_speed, label="Target speed")
    plt.xlabel("Time (s)")
    plt.ylabel("Speed (m/s)")
    plt.title("Speed Tracking")
    plt.grid(True, alpha=0.3)
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_dir / "speed_tracking.png", dpi=180)
    plt.close()

    plt.figure(figsize=(10, 5))
    plt.plot(times, cte, label="Cross-track error")
    plt.plot(times, slam_gt, label="SLAM vs ground-truth error")
    plt.xlabel("Time (s)")
    plt.ylabel("Error (m)")
    plt.title("Tracking and Localization Error")
    plt.grid(True, alpha=0.3)
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_dir / "tracking_errors.png", dpi=180)
    plt.close()

    print("\n=== FINAL LAP RESULTS ===")
    print(f"Lap completed:             {summary['lap_completed']}")
    print(f"Duration:                  {duration:.2f} s")
    print(f"Distance travelled:        {distance_travelled:.2f} m")
    print(f"Maximum speed:             {summary['max_ground_truth_speed_mps']:.3f} m/s")
    print(f"Mean speed:                {summary['mean_ground_truth_speed_mps']:.3f} m/s")
    print(f"Mean cross-track error:    {summary['mean_cross_track_error_m']}")
    print(f"Maximum cross-track error: {summary['max_cross_track_error_m']}")
    print(f"Mean SLAM-GT error:        {summary['mean_slam_ground_truth_error_m']}")

    print(f"\nResults written to: {out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
