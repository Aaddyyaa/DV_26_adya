#!/usr/bin/env python3
"""Generate a report figure only from a CSV written by experiment_logger."""

import csv
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 3:
        print('usage: plot_corridor_csv.py experiments/raw/corridor.csv experiments/figures/corridor.png')
        return 2
    try:
        import matplotlib.pyplot as plot
    except ImportError:
        print('matplotlib is required to render figures; no figure was generated.')
        return 1
    source, output = Path(sys.argv[1]), Path(sys.argv[2])
    rows = []
    with source.open(newline='', encoding='ascii') as input_file:
        for row in csv.DictReader(input_file):
            if int(row['valid']):
                rows.append(row)
    if not rows:
        print('No valid measured rows found; no figure was generated.')
        return 1
    s = [float(row['s']) for row in rows]
    lower = [float(row['lower_mean']) for row in rows]
    upper = [float(row['upper_mean']) for row in rows]
    y_min = [float(row['corridor_min']) for row in rows]
    y_max = [float(row['corridor_max']) for row in rows]
    figure, axis = plot.subplots(figsize=(8, 4.5))
    axis.plot(s, lower, label='lower boundary mean', color='tab:blue')
    axis.plot(s, upper, label='upper boundary mean', color='goldenrod')
    axis.fill_between(s, y_min, y_max, label='probabilistic safe corridor', color='tab:green', alpha=0.25)
    axis.set(xlabel='local longitudinal coordinate s (m)', ylabel='lateral coordinate y (m)')
    axis.legend()
    axis.grid(True, alpha=0.3)
    output.parent.mkdir(parents=True, exist_ok=True)
    figure.tight_layout()
    figure.savefig(output, dpi=180)
    print(f'figure={output}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
