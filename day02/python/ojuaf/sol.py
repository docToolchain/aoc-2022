
SHAPE2POINTS = {
    'X': 1,
    'Y': 2,
    'Z': 3,
    'A': 1,
    'B': 2,
    'C': 3
}


def load_input():
    data = list()
    with open('input') as fd:
        data = list()
        for line in fd:
            line = line.strip()
            data.append(list(map(lambda x: SHAPE2POINTS[x], line.split())))
    return data


def get_result_task1(rd):
    result = 0
    if rd[0] == rd[1]:
        result = 3
    elif (rd[0] == 1 and rd[1] == 3) or (rd[0] == 2 and rd[1] == 1) or \
         (rd[0] == 3 and rd[1] == 2):
        result = 0
    else:
        result = 6
    result += rd[1]
    return result


def get_result_task2(rd):
    if rd[1] == 2:
        return rd[0] + 3
    elif rd[1] == 1:
        shape = rd[0] - 1 if rd[0] - 1 != 0 else 3
        return shape
    else:
        shape = rd[0] + 1 if rd[0] + 1 != 4 else 1
        return shape + 6


def main():
    data = load_input()

    # Task 1
    score = 0
    for rd in data:
        score += get_result_task1(rd)

    result = score
    print("Result 1: ", result)

    # Task 2
    score = 0
    for rd in data:
        score += get_result_task2(rd)

    result = score
    print("Result 2: ", result)


if __name__ == '__main__':
    main()
