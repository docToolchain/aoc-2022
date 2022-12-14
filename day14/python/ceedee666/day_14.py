from functools import reduce
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path: str) -> list[str]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return [l.strip() for l in lines]


def parse_input(lines: list[str]) -> dict[tuple[int, int], str]:
    cave = dict()
    for l in lines:
        coords = l.split(" -> ")
        path = [(int(p[0]), int(p[1])) for p in [c.split(",") for c in coords]]
        for i in range(len(path) - 1):
            x1, y1 = path[i]
            x2, y2 = path[i + 1]

            if x1 == x2:
                if y2 < y1:
                    y1, y2 = y2, y1
                cave |= {(x1, y): "R" for y in range(y1, y2 + 1)}
            else:
                if x2 < x1:
                    x1, x2 = x2, x1
                cave |= {(x, y1): "R" for x in range(x1, x2 + 1)}
    return cave


def min_and_max_x_coord(cave: dict[tuple[int, int], str]) -> tuple[int, int]:
    min_x = reduce(lambda m, k: k[0] if k[0] < m else m, cave, 1000000)
    max_x = reduce(lambda m, k: k[0] if k[0] > m else m, cave, -1000000)
    return (min_x, max_x)


def max_y_coord(cave: dict[tuple[int, int], str]) -> int:
    return reduce(lambda m, k: k[1] if k[1] > m else m, cave, 0)


def add_floor(cave: dict[tuple[int, int], str]) -> dict[tuple[int, int], str]:
    min_x, max_x = min_and_max_x_coord(cave)
    max_y = max_y_coord(cave)

    cave |= {(x, max_y + 2): "R" for x in range(min_x - 2 * max_y, max_x + 2 * max_y)}
    return cave


def simulate_sand(
    cave: dict[tuple[int, int], str], start=(500, 0)
) -> dict[tuple[int, int], str]:

    stop_simu = False
    min_x, max_x = min_and_max_x_coord(cave)
    max_y = max_y_coord(cave)

    while not stop_simu:
        x, y = start
        stop_step = False

        while not stop_step:

            if (x, y + 1) in cave and (x - 1, y + 1) in cave and (x + 1, y + 1) in cave:
                cave[(x, y)] = "S"
                stop_step = True
            else:
                if (x, y + 1) not in cave:
                    y = y + 1
                elif (x - 1, y + 1) not in cave:
                    x = x - 1
                    y = y + 1
                elif (x + 1, y + 1) not in cave:
                    x = x + 1
                    y = y + 1

            if x < min_x or x > max_x or y > max_y or start in cave:
                stop_step = True
                stop_simu = True

    return cave


def solve_part_1(input_lines: list[str]) -> int:
    cave = parse_input(input_lines)
    cave = simulate_sand(cave)
    return sum([1 if cave[k] == "S" else 0 for k in cave])


def solve_part_2(input_lines):
    cave = parse_input(input_lines)
    cave = add_floor(cave)
    cave = simulate_sand(cave)
    return sum([1 if cave[k] == "S" else 0 for k in cave])


@app.command()
def part1(input_file: str):
    result = solve_part_1(read_input_file(input_file))
    print(f"{result} units of sand come to a rest.")


@app.command()
def part2(input_file: str):
    result = solve_part_2(read_input_file(input_file))
    print(f"{result} units of sand come to a rest.")


if __name__ == "__main__":
    app()
