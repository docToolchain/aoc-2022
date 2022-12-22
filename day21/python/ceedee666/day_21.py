import re
from operator import add, mul, sub, truediv
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path) -> list[str]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    return lines


def parse_input(lines: list) -> dict:
    monkeys = {}
    for l in lines:
        name, operation = l.split(":")
        matches = re.findall(r"\d+", operation)
        if matches:
            monkeys[name] = [int(matches[0])]
        else:
            n1, op, n2 = operation.split()
            match op:
                case "+":
                    op = add
                case "-":
                    op = sub
                case "/":
                    op = truediv
                case "*":
                    op = mul
            monkeys[name] = [op, n1, n2]

    return monkeys


def solve(name: str, monkeys: dict) -> int:
    if len(monkeys[name]) == 1:
        return monkeys[name][0]
    else:
        op, n1, n2 = monkeys[name]
        return op(solve(n1, monkeys), solve(n2, monkeys))


def solve_part_1(lines: list) -> int:
    monkeys = parse_input(lines)
    return solve("root", monkeys)


def solve_part_2(lines: list) -> int:
    monkeys = parse_input(lines)
    found = False

    i = 0
    incr = 1
    deltas = []

    _, n1, n2 = monkeys["root"]
    result_n2 = solve(n2, monkeys)

    while not found:
        monkeys["humn"] = [i]
        result_n1 = solve(n1, monkeys)

        if result_n1 == result_n2:
            found = True
        else:
            d = result_n1 - result_n2
            deltas.append(d)

            if len(deltas) > 1:
                if all(
                    abs(deltas[j]) > abs(deltas[j + 1])
                    and (deltas[j] < 0) == (deltas[j + 1] < 0)
                    for j in range(len(deltas) - 1)
                ):
                    incr *= 2
                else:
                    incr = -1 if incr > 0 else 1
                    deltas = []

            i += incr

    return i


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
