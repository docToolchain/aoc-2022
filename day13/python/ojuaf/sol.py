#!/usr/bin/env python3


def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            line = line.strip()
            if line:
                data.append(eval(line))
    return data


def check_order(packet1, packet2):
    ordered = True
    stop = False
    if packet1 and not packet2:
        stop = True
        ordered = False
    elif not packet1 and packet2:
        stop = True
    elif not packet1 and not packet2:
        pass
    else:
        for item1, item2 in zip(packet1, packet2):
            if isinstance(item1, list) and isinstance(item2, list):
                stop, ordered = check_order(item1, item2)
            elif isinstance(item1, int) and isinstance(item2, list):
                stop, ordered = check_order([item1], item2)
            elif isinstance(item1, list) and isinstance(item2, int):
                stop, ordered = check_order(item1, [item2])
            else:
                if item1 > item2:
                    ordered = False
                    stop = True
                elif item1 < item2:
                    stop = True
                else:
                    pass

            if stop:
                break

        else:
            if len(packet1) > len(packet2):
                ordered = False
                stop = True
            elif len(packet1) < len(packet2):
                stop = True
            else:
                pass

    return stop, ordered


def part1():
    data = load_input()
    result = 0
    for i in range(len(data)//2):
        _, ordered = check_order(data[2*i], data[2*i+1])
        if ordered:
            result += i + 1

    print("Result 1: ", result)


def part2():
    data = load_input()
    order = [[[2]], [[6]]]

    for i in range(len(data)):
        for j in range(len(order)):
            _, ordered = check_order(order[j], data[i])
            if not ordered:
                break
        else:
            j += 1

        order.insert(j, data[i])

    result = (order.index([[2]]) + 1) * (order.index([[6]]) + 1)
    print("Result 2: ", result)


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
