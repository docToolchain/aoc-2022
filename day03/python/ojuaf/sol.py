import string


def load_input():
    data = list()
    with open('input') as fd:
        data = list()
        for line in fd:
            line = line.strip()
            data.append(line)
    return data


def get_priority(char):
    return string.ascii_letters.index(char) + 1


def main():
    data = load_input()

    # Task 1
    result = 0
    for rucksack in data:
        length = int(len(rucksack)/2)
        common = set(rucksack[0:length]).intersection(set(rucksack[length:]))
        result += sum(list(map(lambda x: get_priority(x), common)))

    print("Result 1: ", result)

    # Task 2
    chars = set()
    result = 0
    for i, rucksack in enumerate(data):
        if i % 3 == 0:
            chars = set(rucksack)
        else:
            chars.intersection_update(rucksack)

        if i % 3 == 2:
            result += sum(list(map(lambda x: get_priority(x), chars)))

    print("Result 2: ", result)


if __name__ == '__main__':
    main()
