def load_input():
    data = list()
    with open('input') as fd:
        data = list()
        temp = list()
        for line in fd:
            line = line.strip()
            if line:
                temp.append(int(line))
            else:
                data.append(temp)
                temp = list()
    return data


def main():
    data = load_input()

    # Task 1
    calories = list()
    for values in data:
        calories.append(sum(values))
    result = max(calories)
    print("Result 1: ", result)

    # Task 2
    calories.sort()
    result = sum(calories[-3:])
    print("Result 2: ", result)

    return 0


if __name__ == '__main__':
    main()
