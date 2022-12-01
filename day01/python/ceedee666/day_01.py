from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return list(map(lambda l: l.strip(), lines))


def group_calories_by_elve(data):
    calories = []
    group = []

    for e in data:
        if e == "":
            calories.append(group)
            group = []
        else:
            group.append(int(e))

    return calories


@app.command()
def part1(input_file: str):
    data = read_input_file(input_file)
    calories_by_elve = [sum(e) for e in group_calories_by_elve(data)]
    print(f"The maximum calories an elve is carrying is {max(calories_by_elve)}")


@app.command()
def part2(input_file: str):
    data = read_input_file(input_file)
    calories_by_elve = [sum(e) for e in group_calories_by_elve(data)]
    top_three_calories = sorted(calories_by_elve, reverse=True)[0:3]

    print(f"The sum of calories the top three elves is {sum(top_three_calories)}")


if __name__ == "__main__":
    app()
