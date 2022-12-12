#!/usr/bin/env python3

from copy import deepcopy
import math as m


def load_input():
    d = {
        'items': deque(),
        'op': "",
        'cond': list(),
        'count': 0
         }
    data = list()

    with open('input') as fd:
        for i, line in enumerate(fd):
            line = line.strip().replace(',', '')
            temp = line.split()
            if i%7 == 0:
                data.append(deepcopy(d))
            elif (i+6)%7 == 0:
                data[-1]['items'] = deque([int(v) for v in temp[2:]])
            elif (i+5)%7 == 0:
                data[-1]['op'] = "".join(temp[-3:])
            elif (i+4)%7 == 0:
                data[-1]['cond'].append(int(temp[-1]))
            elif (i+3)%7 == 0:
                data[-1]['cond'].append(int(temp[-1]))
            elif (i+2)%7 == 0:
                data[-1]['cond'].append(int(temp[-1]))
            else:
                pass

    return data


def part1():
    data = load_input()
    rounds = 20
    worry_factor = m.prod([monkey['cond'][0] for monkey in data])
    for _ in range(rounds):
        for monkey in data:
            for _ in range(len(monkey['items'])):
                old = monkey['items'].popleft()
                monkey['count'] += 1
                worry = eval(monkey['op'])//3
                if worry % monkey['cond'][0] == 0:
                    data[monkey['cond'][1]]['items'].append(worry)
                else:
                    data[monkey['cond'][2]]['items'].append(worry)

    temp = sorted([monkey['count'] for monkey in data], reverse=True)
    result = temp[0]*temp[1]
    print("Result 1: ", result)


def part2():
    data = load_input()
    rounds = 10000
    worry_factor = m.prod([monkey['cond'][0] for monkey in data])

    for _ in range(rounds):
        for monkey in data:
            for _ in range(len(monkey['items'])):
                old = monkey['items'].popleft()
                monkey['count'] += 1
                worry = eval(monkey['op'])%worry_factor
                if worry % monkey['cond'][0] == 0:
                    data[monkey['cond'][1]]['items'].append(worry)
                else:
                    data[monkey['cond'][2]]['items'].append(worry)

    temp = sorted([monkey['count'] for monkey in data], reverse=True)
    result = temp[0]*temp[1]
    print("Result 2: ", result)



def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
