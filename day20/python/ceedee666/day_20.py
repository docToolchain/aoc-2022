from collections import deque
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path) -> list[int]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    return [int(l) for l in lines]


def solve_part_1(coords: list[int], key=1, rounds=1) -> int:
    numbers = list(enumerate([c * key for c in coords]))
    queue = deque(numbers)
    for e in numbers * rounds:
        idx = queue.index(e)
        queue.rotate(-idx)
        queue.popleft()
        queue.rotate(-e[1])
        queue.insert(0, e)

    new_coords = [n for _, n in queue]
    idx = new_coords.index(0)
    l = len(new_coords)

    return (
        new_coords[(idx + 1000) % l]
        + new_coords[(idx + 2000) % l]
        + new_coords[(idx + 3000) % l]
    )


def solve_part_2(coords):
    return solve_part_1(coords, 811589153, 10)


@app.command()
def part1(input_file: str):
    data = read_input_file(input_file)
    result = solve_part_1(data)
    print(result)


@app.command()
def part2(input_file: str):
    data = read_input_file(input_file)
    result = solve_part_2(data)
    print(result)


if __name__ == "__main__":
    app()
