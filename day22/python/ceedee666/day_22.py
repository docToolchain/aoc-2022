import re
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path) -> list[str]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    return [l.strip("\n") for l in lines]


def parse_input(lines: list) -> tuple[dict, list]:
    board = {}
    for i, l in enumerate(lines[:-1]):
        for j, c in enumerate(l):
            coord = (i, j)
            if c in "#.":
                board[coord] = c

    moves_re = re.compile(r"\d+|L|R")
    matches = moves_re.findall(lines[-1])
    moves = list(map(lambda t: (int(t[0]), t[1]), zip(matches[::2], matches[1::2])))
    if len(matches[::2]) > len(matches[1::2]):
        moves.append((int(matches[-1]), ""))
    return board, moves


def first_in_row(row, board) -> int:
    return min(map(lambda c: c[1], filter(lambda c: c[0] == row, board)))


def last_in_row(row, board) -> int:
    return max(map(lambda c: c[1], filter(lambda c: c[0] == row, board)))


def first_in_col(col, board) -> int:
    return min(map(lambda c: c[0], filter(lambda c: c[1] == col, board)))


def last_in_col(col, board) -> int:
    return max(map(lambda c: c[0], filter(lambda c: c[1] == col, board)))


def move_one_step(pos, board):
    r, c, d = pos
    inc = 0, 0

    match d:
        case "R":
            inc = 0, 1
        case "L":
            inc = 0, -1
        case "U":
            inc = -1, 0
        case "D":
            inc = 1, 0

    n_r = r + inc[0]
    n_c = c + inc[1]

    if (n_r, n_c) not in board:
        match d:
            case "R":
                n_c = first_in_row(r, board)
            case "L":
                n_c = last_in_row(r, board)
            case "D":
                n_r = first_in_col(c, board)
            case "U":
                n_r = last_in_col(c, board)
    tile = board[(n_r, n_c)]
    return (n_r, n_c, d) if tile == "." else (r, c, d)


def move(steps, pos, board):
    new_pos = pos
    for _ in range(steps):
        new_pos = move_one_step(new_pos, board)
    return new_pos


def turn(direction, pos):
    if direction == "":
        return pos
    else:
        d = "RDLU"
        idx = d.index(pos[2])
        if direction == "R":
            idx += 1
            idx %= len(d)
        else:
            idx -= 1
            idx %= len(d)
        return pos[0], pos[1], d[idx]


def solve_part_1(lines: list) -> int:
    board, moves = parse_input(lines)
    col = first_in_row(0, board)
    pos = (0, col, "R")

    for steps, direction in moves:
        pos = move(steps, pos, board)
        pos = turn(direction, pos)

    dir_val = 0
    match pos[2]:
        case "R":
            dir_val = 0
        case "D":
            dir_val = 1
        case "L":
            dir_val = 2
        case "U":
            dir_val = 3

    return 1000 * (pos[0] + 1) + 4 * (pos[1] + 1) + dir_val


@app.command()
def part1(input_file: str):
    lines = read_input_file(input_file)
    print(solve_part_1(lines))


@app.command()
def part2(input_file: str):
    yield


if __name__ == "__main__":
    app()
