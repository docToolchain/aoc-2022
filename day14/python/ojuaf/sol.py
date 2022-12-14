#!/usr/bin/env python3

import numpy as np


def load_input():
    with open('input') as fd:
        data = list()
        for line in fd:
            temp = line.strip().split('->')
            temp = [tuple([int(j) for j in i.split(',')]) for i in temp]
            data.append(temp)
    return data


def get_dir(a, b):
    return tuple([np.sign(j-i) for i, j in zip(a, b)])


def calc_pos(a, b):
    return tuple([i+j for i, j in zip(a, b)])


def create_field(data):
    field = dict()
    for rocks in data:
        for i in range(len(rocks)-1):
            d = get_dir(rocks[i], rocks[i+1])
            pos = rocks[i]
            while True:
                field[pos] = "rock"
                if pos == rocks[i+1]:
                    break
                pos = calc_pos(pos, d)
    return field


def process_sand_part1(field):
    start = (500, 0)
    pos = start
    end = max([key[1] for key in field.keys()])

    while True:
        next_pos = calc_pos(pos, (0, 1))
        if next_pos in field:
            next_pos = calc_pos(pos, (-1, 1))
        if next_pos in field:
            next_pos = calc_pos(pos, (1, 1))
        if next_pos in field:
            field[pos] = 'sand'
            next_pos = start
        if next_pos[1] == end:
            break
        pos = next_pos


def process_sand_part2(field):
    start = (500, 0)
    pos = start
    end = max([key[1] for key in field.keys()]) + 2

    while True:
        if start in field:
            break
        next_pos = calc_pos(pos, (0, 1))
        if next_pos in field:
            next_pos = calc_pos(pos, (-1, 1))
        if next_pos in field:
            next_pos = calc_pos(pos, (1, 1))
        if next_pos in field:
            field[pos] = 'sand'
            next_pos = start
        if next_pos[1] == end:
            field[pos] = 'sand'
            next_pos = start
        pos = next_pos


def part1():
    data = load_input()
    field = create_field(data)
    process_sand_part1(field)

    result = sum([1 for value in field.values() if value == 'sand'])
    print("Result 1: ", result)


def part2():
    data = load_input()
    field = create_field(data)
    process_sand_part2(field)

    result = sum([1 for value in field.values() if value == 'sand'])
    print("Result 2: ", result)


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
