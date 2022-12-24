from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path) -> list[str]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    return [l.strip("\n") for l in lines]


def parse_input(lines: list) -> tuple[set, set]:
    walls = set()
    blizzards = set()
    for x, l in enumerate(lines):
        for y, c in enumerate(l):
            match c:
                case "#":
                    walls.add((x - 1, y - 1))
                case ">":
                    blizzards.add(((x - 1, y - 1), (0, 1)))
                case "<":
                    blizzards.add(((x - 1, y - 1), (0, -1)))
                case "^":
                    blizzards.add(((x - 1, y - 1), (-1, 0)))
                case "v":
                    blizzards.add(((x - 1, y - 1), (1, 0)))
    return walls, blizzards


def search_path(start, end, walls, blizzards) -> tuple:
    max_x = max(x for x, _ in walls)
    max_y = max(y for _, y in walls)

    t = 0
    found = False
    queue = {start}

    while not found:
        t += 1
        blizzards = {
            (((p[0] + d[0]) % max_x, (p[1] + d[1]) % max_y), d) for p, d in blizzards
        }
        blizzard_positions = {p for p, _ in blizzards}
        possible_moves = {
            (x + dx, y + dy)
            for dx, dy in [(-1, 0), (1, 0), (0, 0), (0, 1), (0, -1)]
            for x, y in queue
        }
        possible_moves -= blizzard_positions
        possible_moves -= walls
        possible_moves = set(
            filter(
                lambda p: -1 <= p[0] <= max_x and -1 <= p[1] <= max_y, possible_moves
            )
        )

        queue = possible_moves
        assert len(queue) > 0
        if end in queue:
            found = True

    return t, walls, blizzards


def solve_part_1(lines: list, start: tuple = (-1, 0)) -> int:
    walls, blizzards = parse_input(lines)

    max_x = max(x for x, _ in walls)
    max_y = max(y for _, y in walls)
    end = (max_x, max_y - 1)

    time, _, _ = search_path(start, end, walls, blizzards)
    return time


def solve_part_2(lines: list, start: tuple = (-1, 0)) -> int:
    walls, blizzards = parse_input(lines)

    max_x = max(x for x, _ in walls)
    max_y = max(y for _, y in walls)
    end = (max_x, max_y - 1)
    t1, walls, blizzards = search_path(start, end, walls, blizzards)
    t2, walls, blizzards = search_path(end, start, walls, blizzards)
    t3, walls, blizzards = search_path(start, end, walls, blizzards)
    return sum((t1, t2, t3))


@app.command()
def part1(input_file: str):
    lines = read_input_file(input_file)
    result = solve_part_1(lines)
    print(result)


@app.command()
def part2(input_file: str):
    lines = read_input_file(input_file)
    result = solve_part_2(lines)
    print(result)


if __name__ == "__main__":
    app()
