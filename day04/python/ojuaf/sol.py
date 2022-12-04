#!/usr/bin/env python3

def load_input():
    data = list()
    with open('input') as fd:
        data = list()
        for line in fd:
            temp = list()
            for values in line.strip().split(','):
                temp.append(list(map(lambda x: int(x), values.split('-'))))
            data.append(temp)
    return data


def main():
    data = load_input()

    # Task 1
    fully_contains = 0
    for a, b in data:
        if a[0] <= b[0] and a[1] >= b[1]:
            fully_contains += 1
        elif a[0] >= b[0] and a[1] <= b[1]:
            fully_contains += 1
        else:
            pass
    result = fully_contains
    print("Result 1: ", result)

    # Task 2
    non_overlaps = 0
    for a, b in data:
        if a[1] < b[0] or a[0] > b[1]:
            non_overlaps += 1

    result = len(data) - non_overlaps
    print("Result 2: ", result)


if __name__ == '__main__':
    main()
