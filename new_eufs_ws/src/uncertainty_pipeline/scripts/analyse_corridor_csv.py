#!/usr/bin/env python3
"""Summarise a CSV produced by experiment_logger from real ROS data."""

import csv
import math
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 2:
        print('usage: analyse_corridor_csv.py experiments/raw/corridor.csv')
        return 2
    path = Path(sys.argv[1])
    widths = []
    envelope_widths = []
    feasible = 0
    total = 0
    with path.open(newline='', encoding='ascii') as input_file:
        for row in csv.DictReader(input_file):
            y_min = float(row['corridor_min'])
            y_max = float(row['corridor_max'])
            if not math.isfinite(y_min) or not math.isfinite(y_max):
                continue
            total += 1
            if int(row['valid']):
                feasible += 1
                widths.append(y_max - y_min)
            envelope_widths.append(float(row['lower_sigma']) + float(row['upper_sigma']))
    if not total:
        print('No finite rows found; no result is reported.')
        return 1
    print(f'samples={total}')
    print(f'feasible_track_percent={100.0 * feasible / total:.6f}')
    print(f'mean_corridor_width_m={sum(widths) / len(widths):.6f}' if widths else 'mean_corridor_width_m=unavailable')
    print(f'min_corridor_width_m={min(widths):.6f}' if widths else 'min_corridor_width_m=unavailable')
    print(f'mean_boundary_envelope_width_m={sum(envelope_widths) / len(envelope_widths):.6f}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
