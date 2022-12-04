from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return [l.split() for l in lines]


def my_score(game):
    return {"X": 1, "Y": 2, "Z": 3}[game[1]]


def game_score(game):
    score = 0
    if game in [["A", "X"], ["B", "Y"], ["C", "Z"]]:
        score = 3
    if game in [["C", "X"], ["A", "Y"], ["B", "Z"]]:
        score = 6
    return score


def new_strategy(game):
    shape = ""

    LOOSE = {"A": "Z", "B": "X", "C": "Y"}
    DRAW = {"A": "X", "B": "Y", "C": "Z"}
    WIN = {"C": "X", "A": "Y", "B": "Z"}

    if game[1] == "X":
        shape = LOOSE[game[0]]
    elif game[1] == "Y":
        shape = DRAW[game[0]]
    else:
        shape = WIN[game[0]]

    return [game[0], shape]


@app.command()
def part1(input_file: str):
    games = read_input_file(input_file)
    scores = [game_score(game) + my_score(game) for game in games]
    print(f"The total sorce for part 1 is: {sum(scores)}")


@app.command()
def part2(input_file: str):
    games = read_input_file(input_file)
    new_games = [new_strategy(g) for g in games]
    scores = [game_score(game) + my_score(game) for game in new_games]
    print(f"The total sorce for part 2 is: {sum(scores)}")


if __name__ == "__main__":
    app()
