import string
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return list(map(lambda l: l.strip(), lines))


def find_common_item(items):
    p = int(len(items) / 2)
    return list(set(items[:p]) & set(items[p:]))[0]


def find_group_badge(items):
    return list(set(items[0]) & set(items[1]) & set(items[2]))[0]


def item_priority(item):
    return string.ascii_letters.index(item) + 1


@app.command()
def part1(input_file: str):
    items = read_input_file(input_file)
    priorities = list(map(lambda i: item_priority(find_common_item(i)), items))

    print(f"The sum of the priorities is: {sum(priorities)}")


@app.command()
def part2(input_file: str):
    items = read_input_file(input_file)
    groups = [items[i : i + 3] for i in range(0, len(items), 3)]
    priorities = list(map(lambda i: item_priority(find_group_badge(i)), groups))

    print(f"The sum of the priorities is: {sum(priorities)}")


if __name__ == "__main__":
    app()
