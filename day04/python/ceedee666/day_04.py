from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    ranges = [[l.split(",")[0], l.split(",")[1]] for l in lines]
    ranges = [
        [list(map(int, r[0].split("-"))), list(map(int, r[1].split("-")))]
        for r in ranges
    ]

    return ranges


def contains(a, b):
    return ((a[0] <= b[0]) and (a[1] >= b[1])) or ((b[0] <= a[0]) and (b[1] >= a[1]))


def overlaps(a, b):
    return not ((a[1] < b[0]) or (a[0] > b[1]))


def containing_ranges(ranges):
    return list(filter(lambda r: contains(r[0], r[1]), ranges))


def overlapping_ranges(ranges):
    return list(filter(lambda r: overlaps(r[0], r[1]), ranges))


@app.command()
def part1(input_file: str):
    ranges = read_input_file(input_file)
    containing = containing_ranges(ranges)
    print(f"The number of ranges containing the other range is: {len(containing)}")


@app.command()
def part2(input_file: str):
    ranges = read_input_file(input_file)
    overlap = overlapping_ranges(ranges)
    print(f"The number of overlapping ranges is: {len(overlap)}")


if __name__ == "__main__":
    app()
