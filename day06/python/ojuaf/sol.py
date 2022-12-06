#!/usr/bin/env python3


def load_input():
    with open('input') as fd:
        data = fd.read().strip()
    return data


def start_of_message(data, n):
    """"""
    for i in range(len(data)):
        if len(set(data[i:i+n])) == n:
            break

    return i + n


def main():
    data = load_input()

    # Task 1
    result = start_of_message(data, 4)
    print("Result 1: ", result)

    # Task 2
    result = start_of_message(data, 14)
    print("Result 2: ", result)


if __name__ == '__main__':
    main()
