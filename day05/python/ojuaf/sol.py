#!/usr/bin/env python3

import re
from collections import deque


def load_input():
    number_list = 9

    data = list()
    commands = list()

    pattern = re.compile('move (\d+) from (\d+) to (\d+)')

    with open('input') as fd:
        data = [deque() for _ in range(number_list)]
        section = 0
        for line in fd:
            if section == 0:
                if line[1] == '1':
                    section = 1
                    continue
                for i in range(number_list):
                    pos = 4*i+1
                    if pos < len(line):
                        char = line[pos]
                        if char != ' ':
                            data[i].appendleft(char)
                    else:
                        break
            elif section == 1:
                section = 2
            else:
                line = line.strip()
                match = pattern.search(line)
                commands.append([int(match.group(1)), int(match.group(2))-1, int(match.group(3))-1])

    return data, commands


def main():
    data, commands = load_input()

    # Task 1
    for command in commands:
        for _ in range(command[0]):
            temp = data[command[1]].pop()
            data[command[2]].append(temp)

    result = "".join([char[-1] for char in data])
    print("Result 1: ", result)

    # Task 2
    data, commands = load_input()
    for command in commands:
        temp = deque()
        for _ in range(command[0]):
            temp.appendleft(data[command[1]].pop())
        data[command[2]].extend(temp)

    result = "".join([char[-1] for char in data])
    print("Result 2: ", result)


if __name__ == '__main__':
    main()
