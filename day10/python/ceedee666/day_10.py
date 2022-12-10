from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return list(map(lambda l: l.strip().split(), lines))


def execute_program(program):
    x_values = [1]
    for op in program:
        current_x = x_values[-1]
        match op:
            case ["noop"]:
                x_values += [current_x]
            case ["addx", y]:
                y = int(y)
                x_values += [current_x, current_x + y]
    return x_values


def draw_screen(x_values):
    screen = ""
    for i in range(240):
        if i % 40 == 0:
            screen += "\n"

        sprite_pos = x_values[i]
        if sprite_pos - 1 <= i % 40 <= sprite_pos + 1:
            screen += "#"
        else:
            screen += " "

    return screen


def solve_part_1(program):
    x_values = execute_program(program)
    cycles = [20, 60, 100, 140, 180, 220]
    return sum([x_values[c - 1] * c for c in cycles])


def solve_part_2(program):
    x_values = execute_program(program)
    screen = draw_screen(x_values)
    return screen


@app.command()
def part1(input_file: str):
    program = read_input_file(input_file)
    result = solve_part_1(program)
    print(f"The sum of the signal strengths is {result}")


@app.command()
def part2(input_file: str):
    program = read_input_file(input_file)
    screen = solve_part_2(program)
    print("The resulting screen is:")
    print(screen)


if __name__ == "__main__":
    app()
