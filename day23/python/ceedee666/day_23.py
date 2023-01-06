from collections import Counter
from pathlib import Path

import typer

app = typer.Typer()

DIRECTIONS = "NSWE"


def read_input_file(input_file_path) -> list[str]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    return [l.strip("\n") for l in lines]


def parse_input(lines: list) -> set:
    world = set()
    for i, l in enumerate(lines):
        for j, c in enumerate(l):
            if c == "#":
                pos = (i, j)
                world.add(pos)

    return world


def positions_to_check(pos: tuple, num_round: int) -> list[tuple]:
    x, y = pos
    d = DIRECTIONS[num_round % len(DIRECTIONS)]
    positions = []
    match d:
        case "N":
            positions = [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]
        case "S":
            positions = [(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
        case "W":
            positions = [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]
        case "E":
            positions = [(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
    return positions


def propose_position(pos: tuple, num_round: int, world: set) -> tuple[tuple, bool]:
    neighbours = []
    for i in range(4):
        neighbours += positions_to_check(pos, num_round + i)

    if any([p in world for p in neighbours]):
        for i in range(4):
            pos_to_check = positions_to_check(pos, num_round + i)
            move = all([p not in world for p in pos_to_check])
            if move:
                return pos_to_check[1], move
    return pos, False


def count_tiles(world: set) -> int:
    min_x = min([e[0] for e in world])
    max_x = max([e[0] for e in world])
    min_y = min([e[1] for e in world])
    max_y = max([e[1] for e in world])

    return (max_x - (min_x - 1)) * (max_y - (min_y - 1))


def execute_step(world: set, step) -> set:
    size = len(world)

    proposals = {p: propose_position(p, step, world) for p in world}
    moving_elves = list(filter(lambda p: proposals[p][1] == True, proposals))
    target_position_count = Counter([proposals[p][0] for p in moving_elves])

    for e in moving_elves:
        if target_position_count[proposals[e][0]] != 1:
            proposals[e] = (e, False)
    world = set([proposals[p][0] for p in proposals])
    assert size == len(world)

    return world


def solve_part_1(lines: list, steps: int = 10) -> tuple:
    world = parse_input(lines)

    for step in range(steps):
        world = execute_step(world, step)

    tiles = count_tiles(world)
    return tiles, tiles - len(world)


def solve_part_2(lines: list) -> int:
    world = parse_input(lines)

    step = 0
    finished = False
    while not finished:
        world_2 = execute_step(world, step)
        if world != world_2:
            world = world_2
        else:
            finished = True
        step += 1
    return step


@app.command()
def part1(input_file: str):
    lines = read_input_file(input_file)
    _, empty = solve_part_1(lines)
    print(empty)


@app.command()
def part2(input_file: str):
    lines = read_input_file(input_file)
    step = solve_part_2(lines)
    print(step)


if __name__ == "__main__":
    app()
