import re
from collections import deque
from functools import reduce
from operator import mul
from pathlib import Path

import typer

app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()
    return lines


def parse_input(lines):
    numbers = re.compile(r"\d+")

    return [[int(m) for m in numbers.findall(l)] for l in lines]


def prune_search_space(state, max_cost_ore, max_cost_cla, max_cost_obs):
    # ore, clay, obsidian, geode, ore_robot, clay_robot, obsidian_robot, geode_robot

    ore_robots = max_cost_ore if state[4] >= max_cost_ore else state[4]
    clay_robots = max_cost_cla if state[5] >= max_cost_cla else state[5]
    obsidian_robots = max_cost_obs if state[6] >= max_cost_obs else state[6]

    max_ore = state[8] * max_cost_ore - ore_robots * (state[8] - 1)
    ore = max_ore if state[0] >= max_ore else state[0]

    max_clay = state[8] * max_cost_cla - clay_robots * (state[8] - 1)
    clay = max_clay if state[1] >= max_clay else state[1]

    max_obsidian = state[8] * max_cost_obs - obsidian_robots * (state[8] - 1)
    obsidian = max_obsidian if state[2] >= max_obsidian else state[2]

    return (
        ore,
        clay,
        obsidian,
        state[3],
        ore_robots,
        clay_robots,
        obsidian_robots,
        state[7],
        state[8],
    )


def generate_next_states(
    state,
    cost_or_ore,
    cost_cr_ore,
    cost_obr_ore,
    cost_obr_clay,
    cost_gr_ore,
    cost_gr_obs,
):
    next_states = []

    # ore, clay, obsidian, geode, ore_robot, clay_robot, obsidian_robot, geode_robot
    new_state = (
        state[0] + state[4],
        state[1] + state[5],
        state[2] + state[6],
        state[3] + state[7],
        state[4],
        state[5],
        state[6],
        state[7],
        state[8] - 1,
    )
    next_states.append(new_state)

    if state[0] >= cost_or_ore:
        new_state = (
            state[0] + state[4] - cost_or_ore,
            state[1] + state[5],
            state[2] + state[6],
            state[3] + state[7],
            state[4] + 1,
            state[5],
            state[6],
            state[7],
            state[8] - 1,
        )
        next_states.append(new_state)
    if state[0] >= cost_cr_ore:
        new_state = (
            state[0] + state[4] - cost_cr_ore,
            state[1] + state[5],
            state[2] + state[6],
            state[3] + state[7],
            state[4],
            state[5] + 1,
            state[6],
            state[7],
            state[8] - 1,
        )
        next_states.append(new_state)

    if state[0] >= cost_obr_ore and state[1] >= cost_obr_clay:
        new_state = (
            state[0] + state[4] - cost_obr_ore,
            state[1] + state[5] - cost_obr_clay,
            state[2] + state[6],
            state[3] + state[7],
            state[4],
            state[5],
            state[6] + 1,
            state[7],
            state[8] - 1,
        )
        next_states.append(new_state)
    if state[0] >= cost_gr_ore and state[2] >= cost_gr_obs:
        new_state = (
            state[0] + state[4] - cost_gr_ore,
            state[1] + state[5],
            state[2] + state[6] - cost_gr_obs,
            state[3] + state[7],
            state[4],
            state[5],
            state[6],
            state[7] + 1,
            state[8] - 1,
        )
        next_states.append(new_state)
    return next_states


def determine_quality(line, time=24):
    (
        plan_id,
        cost_or_ore,
        cost_cr_ore,
        cost_obr_ore,
        cost_obr_clay,
        cost_gr_ore,
        cost_gr_obs,
    ) = line

    max_cost_ore = max(cost_or_ore, cost_cr_ore, cost_obr_ore, cost_gr_ore)
    # ore, clay, obsidian, geode, ore_robot, clay_robot, obsidian_robot, geode_robot
    state = (0, 0, 0, 0, 1, 0, 0, 0, time)
    max_value = 0
    queue = deque([state])
    seen = set()

    while queue:
        current_state = queue.popleft()
        max_value = max(max_value, current_state[3])

        if current_state[8] > 0:
            current_state = prune_search_space(
                current_state, max_cost_ore, cost_obr_clay, cost_gr_obs
            )
            if current_state not in seen:
                seen.add(current_state)
                queue.extend(
                    generate_next_states(
                        current_state,
                        cost_or_ore,
                        cost_cr_ore,
                        cost_obr_ore,
                        cost_obr_clay,
                        cost_gr_ore,
                        cost_gr_obs,
                    )
                )

    return plan_id, max_value


def solve_part_1(lines):
    quality = [determine_quality(l) for l in parse_input(lines)]
    return sum(i * q for (i, q) in quality)


def solve_part_2(lines):
    quality = [determine_quality(l, 32) for l in parse_input(lines[:3])]
    return reduce(mul, [q for (_, q) in quality])


@app.command()
def part1(input_file: str):
    data = read_input_file(input_file)
    result = solve_part_1(data)
    print(result)


@app.command()
def part2(input_file: str):
    data = read_input_file(input_file)
    result = solve_part_2(data)
    print(result)


if __name__ == "__main__":
    app()
