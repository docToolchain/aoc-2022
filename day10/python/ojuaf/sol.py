#!/usr/bin/env python3


def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            line = line.strip().split()
            if line[0] == 'addx':
                data.append([line[0], int(line[1])])
            else:
                data.append(line)
    return data


def update_pic(pic, value, cycle):
    if cycle in list(range(value-1, value+2)):
        pic += '#'
    else:
        pic += ' '
    return pic


def part():
    data = load_input()
    value = 1
    cycle = 0
    limit = 20

    result = 0
    pic = ''

    next_add = 0

    for cmd in data:
        value += next_add
        next_add = 0

        pic = update_pic(pic, value, cycle % 40)
        cycle += 1

        if cmd[0] == 'addx':
            pic = update_pic(pic, value, cycle % 40)
            cycle += 1
            next_add = cmd[1]

        if cycle >= limit:
            result += value*limit
            limit += 40

    print("Result 1: ", result)

    # Task2
    print("Result2:")
    for i in range(cycle//40):
        print(pic[40*i:40*i+40])


def main():
    part()


if __name__ == '__main__':
    main()
