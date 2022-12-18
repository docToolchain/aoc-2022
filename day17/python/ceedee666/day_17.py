from itertools import groupby
from pathlib import Path
from typing import Generator

import typer

app = typer.Typer()


def next_shape() -> Generator[set, None, None]:
    shapes = [
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 0), (1, 0), (0, 1), (1, 1)],
    ]
    i = 0
    while True:
        yield set(shapes[i])
        i += 1
        i %= len(shapes)


def read_input_file(input_file_path: str) -> str:
    p = Path(input_file_path)

    with p.open() as f:
        line = f.readline().strip()

    return line


def generate_jets(line: str) -> Generator[str, None, None]:
    def jets():
        i = 0
        while True:
            yield line[i]
            i += 1
            i %= len(line)

    return jets()


def maximum_hight(coords) -> int:
    return max([c[1] for c in coords]) if len(coords) > 0 else -1


def calc_shape_pos(x, y, shape) -> set:
    positions = [(x + c[0], y + c[1]) for c in shape]
    return set(positions)


def fall(shape) -> set:
    return set([(c[0], c[1] - 1) for c in shape])


def jet_push(shape: set, jets: Generator[str, None, None], width: int = 7) -> set:
    d_x = 0

    match next(jets):
        case "<":
            d_x = -1
        case ">":
            d_x = +1

    next_shape = set([(c[0] + d_x, c[1]) for c in shape])
    x_coords = [c[0] for c in next_shape]
    min_x = min(x_coords)
    max_x = max(x_coords)

    if min_x < 0 or max_x > width - 1:
        return shape
    else:
        return next_shape


def simulate_rocks(
    shapes, jets, width: int = 7, num_of_rocks: int = 2022, draw: bool = False
) -> set[tuple[int, int]]:
    chamber = set()

    for _ in range(num_of_rocks):
        shape = next(shapes)
        start_y = maximum_hight(chamber) + 4
        start_x = 2

        current_s = calc_shape_pos(start_x, start_y, shape)
        current_s = jet_push(current_s, jets, width)

        falling = True

        while falling:
            next_s = fall(current_s)
            if any([c in chamber or c[1] < 0 for c in next_s]):
                falling = False
            else:
                current_s = next_s
                next_s = jet_push(current_s, jets, width)
                if not any([c in chamber for c in next_s]):
                    current_s = next_s

        chamber |= current_s

    return chamber


def solve_part_1(line: str):
    jets = generate_jets(line)
    shapes = next_shape()

    chamber = simulate_rocks(shapes, jets)
    return maximum_hight(chamber) + 1


@app.command()
def part1(input_file: str):
    line = read_input_file(input_file)
    result = solve_part_1(line)
    print(f"After 2022 rocks the tower is {result} units tall")

if __name__ == "__main__":
    app()
