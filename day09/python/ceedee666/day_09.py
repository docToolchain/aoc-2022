from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    moves = [l.strip().split() for l in lines]

    return moves


def execute_moves(moves, knots=2, start=(0, 0)):
    rope = [[start] for _ in range(knots)]

    for move in moves:
        h_dir = (0, 0)

        match move:
            case ["U", _]:
                h_dir = (1, 0)
            case ["D", _]:
                h_dir = (-1, 0)
            case ["L", _]:
                h_dir = (0, -1)
            case ["R", _]:
                h_dir = (0, 1)

        for _ in range(int(move[1])):
            x_pos, y_pos = rope[0][-1]
            new_head_pos = (x_pos + h_dir[0], y_pos + h_dir[1])
            rope[0].append(new_head_pos)

            for i in range(knots - 1):
                pos = follow(rope[i][-1], rope[i + 1][-1])
                rope[i + 1].append(pos)

    return rope


def follow(h_position, t_position):
    x_pos_h, y_pos_h = h_position
    x_pos_t, y_pos_t = t_position

    if not knots_touching(h_position, t_position):
        if x_pos_h < x_pos_t:
            x_pos_t -= 1
        if x_pos_h > x_pos_t:
            x_pos_t += 1

        if y_pos_h < y_pos_t:
            y_pos_t -= 1
        if y_pos_h > y_pos_t:
            y_pos_t += 1

    return (x_pos_t, y_pos_t)


def knots_touching(h_position, t_position):
    x_pos_h, y_pos_h = h_position
    x_pos_t, y_pos_t = t_position

    x_dist = x_pos_h - x_pos_t
    y_dist = y_pos_h - y_pos_t

    if abs(x_dist) <= 1 and abs(y_dist) <= 1:
        return True
    else:
        return False


def solve_part_1(moves):
    return len(set(execute_moves(moves)[-1]))


def solve_part_2(moves):
    return len(set(execute_moves(moves, 10)[-1]))


@app.command()
def part1(input_file: str):
    moves = read_input_file(input_file)
    print(f"The tail of the rope visits {solve_part_1(moves)} positions at least once.")


@app.command()
def part2(input_file: str):
    moves = read_input_file(input_file)
    print(f"The tail of the rope visits {solve_part_2(moves)} positions at least once.")


if __name__ == "__main__":
    app()
