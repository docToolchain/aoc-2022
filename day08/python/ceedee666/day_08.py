from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    grid = [[int(c) for c in l.strip()] for l in lines]

    return grid


def is_visible(r_idx, c_idx, grid):
    visible = False

    # outside trees are visible
    if r_idx == 0 or r_idx == len(grid) - 1 or c_idx == 0 or c_idx == len(grid[0]) - 1:
        visible = True
    else:
        # check row visibility
        if max(grid[r_idx][:c_idx]) < grid[r_idx][c_idx]:
            visible = True
        if c_idx + 1 < len(grid[r_idx]):
            if max(grid[r_idx][c_idx + 1 :]) < grid[r_idx][c_idx]:
                visible = True

        # check col visibility
        row_range_low = range(r_idx)
        row_range_high = range(r_idx + 1, len(grid))
        col_low = [grid[r][c_idx] for r in row_range_low]
        col_high = [grid[r][c_idx] for r in row_range_high]

        if max(col_low) < grid[r_idx][c_idx] or (
            len(col_high) > 0 and (max(col_high) < grid[r_idx][c_idx])
        ):
            visible = True

    return visible


def scenic_score(r_idx, c_idx, grid):
    distance_left = 0

    if c_idx < len(grid[r_idx]) and c_idx > 0:
        view_blocked = False
        for c in range(c_idx - 1, -1, -1):
            if not view_blocked:
                distance_left += 1
            if grid[r_idx][c] >= grid[r_idx][c_idx]:
                view_blocked = True

    distance_right = 0
    view_blocked = False
    for c in range(c_idx + 1, len(grid[r_idx])):
        if not view_blocked:
            distance_right += 1
        if grid[r_idx][c] >= grid[r_idx][c_idx]:
            view_blocked = True

    distance_up = 0
    if r_idx < len(grid) and r_idx > 0:
        view_blocked = False
        for r in range(r_idx - 1, -1, -1):
            if not view_blocked:
                distance_up += 1
            if grid[r][c_idx] >= grid[r_idx][c_idx]:
                view_blocked = True

    distance_down = 0
    view_blocked = False
    for r in range(r_idx + 1, len(grid)):
        if not view_blocked:
            distance_down += 1
        if grid[r][c_idx] >= grid[r_idx][c_idx]:
            view_blocked = True

    return distance_left * distance_right * distance_up * distance_down


def solve_part_1(grid):
    grid_visible = [
        [1 if is_visible(i, j, grid) else 0 for j in range(len(grid[i]))]
        for i in range(len(grid))
    ]
    return sum(map(sum, grid_visible))


def solve_part_2(grid):
    scenic_score_grid = [
        [scenic_score(i, j, grid) for j in range(len(grid[i]))]
        for i in range(len(grid))
    ]
    return max(map(max, scenic_score_grid))


@app.command()
def part1(input_file: str):
    grid = read_input_file(input_file)
    visible_trees = solve_part_1(grid)
    print(f"From the outside of the grid {visible_trees} are visible")


@app.command()
def part2(input_file: str):
    grid = read_input_file(input_file)
    max_scenic_score = solve_part_2(grid)
    print(f"The highest scenic score is {max_scenic_score}")


if __name__ == "__main__":
    app()
