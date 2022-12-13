import ast
from functools import cmp_to_key
from itertools import chain
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path: str) -> list:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return [l.strip() for l in lines]


def parse_input(lines: list[str]) -> list[list]:
    return [
        [ast.literal_eval(lines[i]), ast.literal_eval(lines[i + 1])]
        for i in range(0, len(lines), 3)
    ]


def compare(left, right):
    if isinstance(left, list) and isinstance(right, list):
        for pair in zip(left, right):
            result = compare(pair[0], pair[1])
            if result in [-1, 1]:
                return result

        if len(left) < len(right):
            return -1
        elif len(left) > len(right):
            return 1
        else:
            return 0

    elif isinstance(left, int) and isinstance(right, int):
        if left < right:
            return -1
        if left > right:
            return 1
        else:
            return 0

    elif isinstance(left, int) and isinstance(right, list):
        return compare([left], right)

    elif isinstance(left, list) and isinstance(right, int):
        return compare(left, [right])


def validate_pairs(pairs):
    return list(map(lambda p: compare(p[0], p[1]), pairs))


def solve_part_1(input_lines: list[str]) -> int:
    pairs = parse_input(input_lines)
    in_order = validate_pairs(pairs)
    return sum(map(lambda e: e[0] + 1 if e[1] == -1 else 0, enumerate(in_order)))


def solve_part_2(input_lines):
    dp1 = [[2]]
    dp2 = [[6]]
    pairs = parse_input(input_lines)
    all_packets = list(chain(*pairs)) + [dp1] + [dp2]
    sorted_packets = sorted(all_packets, key=cmp_to_key(compare))
    return (sorted_packets.index(dp1) + 1) * (sorted_packets.index(dp2) + 1)


@app.command()
def part1(input_file: str):
    result = solve_part_1(read_input_file(input_file))
    print(f"The sum of the indices of the pairs in correct order is {result}")


@app.command()
def part2(input_file: str):
    result = solve_part_2(read_input_file(input_file))
    print(f"The decoder key is {result}")


if __name__ == "__main__":
    app()
