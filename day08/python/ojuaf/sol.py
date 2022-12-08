#!/usr/bin/env python3


import numpy as np


def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            line = line.strip()
            data.append([int(value) for value in line])
    data = np.array(data)
    return data


def main():
    data = load_input()

    # Task 1
    visibles = np.full(data.shape, True, dtype=np.bool_)
    visibles[1:-1, 1:-1] = False

    inner = data[1:-1, 1:-1]
    it = np.nditer(inner, flags=['multi_index'])
    for value in it:
        i, j = it.multi_index
        i += 1
        j += 1
        for k in range(4):
            highest = 0
            if k == 0:
                highest = max(data[0:i, j])
            elif k == 1:
                highest = max(data[i+1:, j])
            elif k == 2:
                highest = max(data[i, 0:j])
            else:
                highest = max(data[i, j+1:])
            if highest < value:
                visibles[i, j] = True
                break

    result = np.sum(visibles)
    print("Result 1: ", result)

    # Task2
    directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    options = visibles.copy()

    options[0, :] = False
    options[:, 0] = False
    options[-1, :] = False
    options[:, -1] = False
    indexes = np.argwhere(options)

    most_scenic = 0

    for i, j in indexes:
        temp = 1
        for d in directions:
            y = i
            x = j
            while True:
                y += d[0]
                x += d[1]
                if x == 0 or x == data.shape[1] - 1 or y == 0 or \
                   y == data.shape[0] - 1 or data[y, x] >= data[i, j]:
                    temp *= abs(j-x) + abs(i-y)
                    break

        most_scenic = most_scenic if most_scenic > temp else temp

    result = most_scenic
    print("Result 2: ", result)


if __name__ == '__main__':
    main()
