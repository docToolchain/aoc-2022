import string
import sys
from collections import deque
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path: str):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return [l.strip() for l in lines]


def parse_input(lines):
    height_grid = []
    start, end = (0, 0), (0, 0)

    for x in range(len(lines)):
        height_grid_row = []

        for y in range(len(lines[x])):
            match lines[x][y]:
                case "S":
                    height_grid_row.append(0)
                    start = (x, y)
                case "E":
                    height = len(string.ascii_lowercase) - 1
                    height_grid_row.append(height)
                    end = (x, y)
                case _:
                    height = string.ascii_lowercase.index(lines[x][y])
                    height_grid_row.append(height)

        height_grid.append(height_grid_row)

    return start, end, height_grid


def init_dist_grid(start, height_grid):
    s_x, s_y = start
    dist_grid = []

    for _ in range(len(height_grid)):
        dist_grid_row = []

        for _ in range(len(height_grid[0])):
            dist_grid_row.append(sys.maxsize)

        dist_grid.append(dist_grid_row)

    dist_grid[s_x][s_y] = 0
    return dist_grid


def neighbours(x, y, height_grid):
    candidates = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]

    neighbours = list(
        filter(
            lambda c: 0 <= c[0] < len(height_grid) and 0 <= c[1] < len(height_grid[0]),
            candidates,
        )
    )

    neighbours = list(
        filter(lambda c: height_grid[c[0]][c[1]] <= height_grid[x][y] + 1, neighbours)
    )

    return neighbours


def bfs(start, end, height_grid, dist_grid):
    path_found = False
    visited = [start]
    next_coords = deque([start])

    while len(next_coords) > 0 and not path_found:
        c_x, c_y = next_coords.popleft()
        for n in neighbours(c_x, c_y, height_grid):
            if n == end:
                path_found = True

            if n not in visited:
                visited.append(n)
                next_coords.append(n)

            n_x, n_y = n

            if dist_grid[n_x][n_y] > dist_grid[c_x][c_y] + 1:
                dist_grid[n_x][n_y] = dist_grid[c_x][c_y] + 1

    return dist_grid


def solve_part_1(input_lines):
    start, end, height_grid = parse_input(input_lines)
    dist_grid = init_dist_grid(start, height_grid)
    dist_grid = bfs(start, end, height_grid, dist_grid)
    return dist_grid[end[0]][end[1]]


def solve_part_2(input_lines):
    _, end, height_grid = parse_input(input_lines)

    possible_starts = list(
        filter(
            lambda c: height_grid[c[0]][c[1]] == 0,
            [
                (i, j)
                for i in range(len(height_grid))
                for j in range(len(height_grid[0]))
            ],
        )
    )

    distances = []

    for start in possible_starts:
        dist_grid = init_dist_grid(start, height_grid)
        distances.append(bfs(start, end, height_grid, dist_grid)[end[0]][end[1]])

    return min(distances)


@app.command()
def part1(input_file):
    steps = solve_part_1(read_input_file(input_file))
    print(f"The fewest steps to the position with the best signal is {steps}")


@app.command()
def part2(input_file: str):
    steps = solve_part_2(read_input_file(input_file))
    print(
        f"The fewest steps form any elevation a to the position with the best signal is {steps}"
    )


if __name__ == "__main__":
    app()
