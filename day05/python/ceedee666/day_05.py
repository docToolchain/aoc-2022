import string
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return list(map(lambda l: l.strip(), lines))


def parse_input(lines):
    split_index = lines.index("")
    state_lines = lines[:split_index]
    commands_lines = lines[split_index + 1 :]

    stacks = parse_state(state_lines)
    commands = parse_commands(commands_lines)

    return stacks, commands


def parse_state(lines):
    stack_count = int(lines[-1].split()[-1])
    stacks = [list() for _ in range(stack_count)]

    for line in reversed(lines[:-1]):
        for i in range(stack_count):
            index = (4 * i) + 1
            if index < len(line):
                if line[index] in string.ascii_uppercase:
                    stacks[i].append(line[index])

    return stacks


def parse_commands(lines):
    commands = [l.split() for l in lines]
    commands = [(int(c[1]), int(c[3]), int(c[5])) for c in commands]
    return commands


def execute_movements(stacks, commands):
    for c in commands:
        for _ in range(c[0]):
            e = stacks[c[1] - 1].pop()
            stacks[c[2] - 1].append(e)
    return stacks


def execute_movements_part2(stacks, commands):
    for c in commands:
        elements = []

        for _ in range(c[0]):
            elements.append(stacks[c[1] - 1].pop())

        stacks[c[2] - 1] += reversed(elements)
    return stacks


@app.command()
def part1(input_file: str):
    lines = read_input_file(input_file)
    stacks, commands = parse_input(lines)
    resulting_stacks = execute_movements(stacks, commands)

    print(
        f"The crates on top of each stack are: {''.join([s[-1] for s in resulting_stacks])}"
    )


@app.command()
def part2(input_file: str):
    lines = read_input_file(input_file)
    stacks, commands = parse_input(lines)
    resulting_stacks = execute_movements_part2(stacks, commands)

    print(
        f"The crates on top of each stack are: {''.join([s[-1] for s in resulting_stacks])}"
    )


if __name__ == "__main__":
    app()
