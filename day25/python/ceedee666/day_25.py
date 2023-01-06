import math
from pathlib import Path

import typer

app = typer.Typer()


TO_DEC = {"2": 2, "1": 1, "0": 0, "-": -1, "=": -2}
TO_SNAFU = {0: "0", 1: "1", 2: "2", 3: "=", 4: "-"}


def read_input_file(input_file_path) -> list[str]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    return [l.strip("\n") for l in lines]


def convert_to_decimal(snafu_num: str) -> int:
    dec_num = 0
    p = len(snafu_num) - 1
    for i, c in enumerate(snafu_num):
        dec_num += TO_DEC[c] * 5 ** (p - i)
    return dec_num


def convert_to_snafu(dec_num: int) -> str:
    snafu_num = ""
    num = dec_num
    while num > 0:
        num, rem = divmod(num, 5)
        snafu_num += TO_SNAFU[rem]
        if rem > 2:
            num += 1
    return snafu_num[::-1] if snafu_num else str(0)


def solve_part_1(lines: list) -> str:
    return convert_to_snafu(sum(convert_to_decimal(s) for s in lines))


@app.command()
def part1(input_file: str):
    lines = read_input_file(input_file)
    print(solve_part_1(lines))


@app.command()
def part2(input_file: str):
    print(0)


if __name__ == "__main__":
    app()
