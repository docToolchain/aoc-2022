import functools
import re
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path: str) -> list[str]:
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return [l.strip() for l in lines]


def parse_input(lines: list[str]) -> dict:
    valves_pattern = re.compile(r"[A-Z]{2}")
    flow_rate_pattern = re.compile(r"\d+")

    valves = [re.findall(valves_pattern, l) for l in lines]
    flow_rates = [re.findall(flow_rate_pattern, l)[0] for l in lines]

    scan = {
        v[0]: {"flow": int(f), "tunnels": v[1:], "paths": {}}
        for v, f in zip(valves, flow_rates)
    }

    return scan


def distance(start: str, end: str, scan: dict) -> int:
    current_step = set([start])
    dist = 0

    while True:
        next_step = set()
        for s in current_step:
            if s == end:
                return dist
            else:
                next_step |= set(scan[s]["tunnels"])
        current_step = next_step
        dist += 1


def build_distance_cache(valves: dict, scan: dict) -> dict:
    for s in valves:
        for e in valves:
            if s != e:
                valves[s]["paths"][e] = distance(s, e, scan)
    return valves


def solve_part_1(input_lines: list[str], time: int = 30) -> int:
    scan = parse_input(input_lines)

    usable_valves = {k: v for k, v in scan.items() if v["flow"] > 0 or k == "AA"}
    usable_valves = build_distance_cache(usable_valves, scan)

    def search_max_flow(current: str, time: int, opened: set):
        if current in opened or time <= 0:
            return 0
        else:
            next_opened = opened | set([current])

            max_flow = max(
                [
                    search_max_flow(
                        p, time - 1 - usable_valves[current]["paths"][p], next_opened
                    )
                    for p in usable_valves[current]["paths"]
                ]
            )

            return usable_valves[current]["flow"] * time + max_flow

    return search_max_flow("AA", time, set())


@app.command()
def part1(input_file: str):
    result = solve_part_1(read_input_file(input_file))
    print(f"The most pressure that can be release in 30 minutes is {result}")


if __name__ == "__main__":
    app()
