import re
from functools import reduce
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path: str) -> list[str]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return [l.strip() for l in lines]


def parse_input(lines: list[str]):
    int_pattern = re.compile(r"-?\d+")
    raw_sensor_data = [re.findall(int_pattern, line) for line in lines]

    sensors = list(map(lambda l: (int(l[0]), int(l[1])), raw_sensor_data))
    beacons = list(map(lambda l: (int(l[2]), int(l[3])), raw_sensor_data))
    distances = list(map(lambda x, y: manhatten_distance(x, y), sensors, beacons))

    return sensors, beacons, distances


def manhatten_distance(x, y):
    return abs(x[0] - y[0]) + abs(x[1] - y[1])


def delta_row(sensor, distance, row):
    return distance - abs(sensor[1] - row)


def min_max_col(sensors, distances, row):
    min_col = min(s[0] - delta_row(s, d, row) for s, d in zip(sensors, distances))
    max_col = max(s[0] + delta_row(s, d, row) for s, d in zip(sensors, distances))
    return min_col, max_col


def possible_beacon_positions(sensors, beacons, distances, row):
    ranges = []
    for s, d in zip(sensors, distances):
        d_row = delta_row(s, d, row)
        if d_row > 0:
            ranges.append((s[0] - d_row, s[0] + d_row))

    ranges = sorted(ranges, key=lambda r: r[0])

    possible_positions = []
    max_col = ranges[0][1]

    for i in range(len(ranges) - 1):
        max_col_prev = ranges[i][1]
        min_col_next = ranges[i + 1][0]

        if max_col_prev > max_col:
            max_col = max_col_prev

        if max_col < min_col_next:
            for c in range(max_col + 1, min_col_next):
                possible_positions.append((c, row))

    return possible_positions


def solve_part_1(input_lines: list[str], row: int) -> int:
    sensors, beacons, distances = parse_input(input_lines)
    min_col, max_col = min_max_col(sensors, distances, row)
    return (
        max_col
        - min_col
        - len(possible_beacon_positions(sensors, beacons, distances, row))
    )


def solve_part_2(input_lines: list[str], max_col: int, max_row: int) -> int:
    sensors, beacons, distances = parse_input(input_lines)
    for r in range(max_col + 1):
        positions = possible_beacon_positions(sensors, beacons, distances, r)
        positions = list(
            filter(lambda p: 0 <= p[0] <= max_col and 0 <= p[1] <= max_row, positions)
        )
        if len(positions) > 0:
            p = positions[0]
            return p[0] * 4000000 + p[1]

    return 0


@app.command()
def part1(input_file: str):
    result = solve_part_1(read_input_file(input_file), 2000000)
    print(f"In the row 2000000 {result} positions cannot contain a beacon")


@app.command()
def part2(input_file: str):
    result = solve_part_2(read_input_file(input_file), 4000000, 4000000)
    print(f"The tuning frequency is {result}")


if __name__ == "__main__":
    app()
