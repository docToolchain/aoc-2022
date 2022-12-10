#!/usr/bin/env python3

import numpy as np


direction = {
    'R': (0, 1),
    'L': (0, -1),
    'D': (-1, 0),
    'U': (1, 0)
}


def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            a, b = line.strip().split()
            data.append((a, int(b)))
    return data


def next_pos(pos, d):
    next_pos = (pos[0] + d[0], pos[1] + d[1])
    return next_pos


def next_tail_pos(head, tail):
    next_tail_pos = tail
    diff = [i - j for i, j in zip(head, tail)]
    if max([abs(v) for v in diff]) > 1:
        d = [np.sign(i) for i in diff]
        next_tail_pos = next_pos(tail, d)

    return next_tail_pos


def get_tail_pos(pos, value, length):
    tail_pos = list()
    for i in range(value[1]):
        pos[0] = next_pos(pos[0], direction[value[0]])
        for j in range(length):
            pos[j+1] = next_tail_pos(pos[j], pos[j+1])
        tail_pos.append(pos[length])
    return tail_pos


def part_n(number, length):
    data = load_input()
    length = length
    pos = [(0, 0) for _ in range(length+1)]
    tail_pos = list()
    for value in data:
        tail_pos.extend(get_tail_pos(pos, value, length))

    result = len(set(tail_pos))
    print(f"Result: {number}: {result}")


def main():
    part_n(1, 1)
    part_n(2, 9)


if __name__ == '__main__':
    main()
