from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return list(map(lambda l: l.strip(), lines))


def parse_input(lines):
    return set(map(lambda l: tuple(map(int, l.split(","))), lines))


def all_neighbours(coord):
    x, y, z = coord
    neighbours = set(
        [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
    )
    return neighbours


def count_neighbours(coord, scan):
    existing_neighbours = all_neighbours(coord)
    existing_neighbours &= scan
    return len(existing_neighbours)


def max_xyz(scan):
    max_x = max([c[0] for c in scan])
    max_y = max([c[1] for c in scan])
    max_z = max([c[2] for c in scan])
    return max(max_x, max_y, max_z)


def fill_outside(scan):
    visited = set()
    queue = [(-1, -1, -1)]

    while len(queue) > 0:
        current = queue.pop()

        neighbours = all_neighbours(current)
        neighbours = set(
            filter(
                lambda coord: all(-1 <= c <= max_xyz(scan) + 1 for c in coord),
                neighbours,
            )
        )
        neighbours -= scan
        neighbours -= visited

        queue += list(neighbours)
        visited.add(current)

    return visited


def solve_part_1(lines):
    scan = parse_input(lines)
    all_scan_neighbours = [n for c in scan for n in all_neighbours(c)]
    return len(list(filter(lambda n: n not in scan, all_scan_neighbours)))


def solve_part_2(lines):
    scan = parse_input(lines)
    outside = fill_outside(scan)
    all_scan_neighbours = [n for c in scan for n in all_neighbours(c)]
    return len(list(filter(lambda n: n in outside, all_scan_neighbours)))


@app.command()
def part1(input_file: str):
    data = read_input_file(input_file)
    r = solve_part_1(data)
    print(f"The surface area of the scanned lava droplet is {r}")


@app.command()
def part2(input_file: str):
    data = read_input_file(input_file)
    r = solve_part_2(data)
    print(f"The surface area of the scanned lava droplet is {r}")


if __name__ == "__main__":
    app()
