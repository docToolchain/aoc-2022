from functools import reduce
from operator import add, mul
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path: str):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return [l.strip() for l in lines]


def parse_input(lines: list[str]):
    monkeys = []
    monkey = dict()

    for l in lines:
        if "Monkey" in l:
            if len(monkey) > 0:
                monkeys.append(monkey)
            monkey = {"ic": 0}

        if "Starting" in l:
            items = l.split(":")[1].split(",")
            items = list(map(int, items))
            monkey["items"] = items

        if "Operation" in l:
            op = l.split("=")[1].strip().split(" ")
            if op[1] == "+":
                monkey["op"] = add
                monkey["val"] = int(op[2])
            else:
                monkey["op"] = mul
                if "old" in op[2]:
                    monkey["val"] = "o"
                else:
                    monkey["val"] = int(op[2])

        if "Test" in l:
            monkey["test"] = int(l.split()[-1])

        if "If t" in l:
            monkey["mt"] = int(l.split()[-1])

        if "If f" in l:
            monkey["mf"] = int(l.split()[-1])

    monkeys.append(monkey)
    return monkeys


def execute_round(monkeys, divider, mod):
    for m in monkeys:
        for item in m["items"]:
            m["items"] = m["items"][1:]
            m["ic"] += 1

            if m["val"] == "o":
                val = item
            else:
                val = m["val"]

            new_item = m["op"](item, val) // divider

            if new_item % m["test"] == 0:
                next_monkey = m["mt"]
            else:
                next_monkey = m["mf"]

            new_item %= mod
            monkeys[next_monkey]["items"].append(new_item)

    return monkeys


def execute_rounds(monkeys, rounds=20, divider=3):
    divisor_product = reduce(mul, [m["test"] for m in monkeys], 1)

    for _ in range(rounds):
        monkeys = execute_round(monkeys, divider, divisor_product)
    return monkeys


def solve_part_1(input_lines):
    monkeys = parse_input(input_lines)
    monkeys = execute_rounds(monkeys)
    inspection_counts = sorted([m["ic"] for m in monkeys])
    return inspection_counts[-1] * inspection_counts[-2]


def solve_part_2(input_lines):
    monkeys = parse_input(input_lines)
    monkeys = execute_rounds(monkeys, divider=1, rounds=10000)
    inspection_counts = sorted([m["ic"] for m in monkeys])
    return inspection_counts[-1] * inspection_counts[-2]


@app.command()
def part1(input_file):
    monkey_business_value = solve_part_1(read_input_file(input_file))
    print(
        f"The value of the monkey business after 20 rounds is {monkey_business_value}"
    )


@app.command()
def part2(input_file: str):
    monkey_business_value = solve_part_2(read_input_file(input_file))
    print(
        f"The value of the monkey business after 10000 rounds is {monkey_business_value}"
    )


if __name__ == "__main__":
    app()
