#!/usr/bin/env python3


import re


def get_dist(a, b):
    return sum(abs(i-j) for i, j in zip(a, b))


def load_input():
    sensors = list()
    beacons = list()

    pattern = re.compile('Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)')

    with open('input') as fd:
        for line in fd:
            line = line.strip()
            match = pattern.search(line)
            sensors.append((int(match.group(1)), int(match.group(2))))
            beacons.append((int(match.group(3)), int(match.group(4))))
    return sensors, beacons


def part1():
    sensors, beacons = load_input()
    line = 2000000

    positions = list()
    for sensor, beacon in zip(sensors, beacons):
        d = get_dist(sensor, beacon)
        d_y = abs(sensor[1] - line)
        d_x = d - d_y
        if d_x >= 0:
            x_min = sensor[0] - d_x
            x_max = sensor[0] + d_x
            positions.append([x_min, x_max])

    positions.sort(key=lambda x: x[0])
    fp = [positions[0]]
    for pos in positions[1:]:
        if fp[-1][1] >= pos[0]-1:
            if fp[-1][1] < pos[1]:
                fp[-1][1] = pos[1]
        else:
            fp.append(pos)

    result = sum([j - i for i, j in fp])
    print("Result 1: ", result)


def part2():
    sensors, beacons = load_input()
    x_limit = 4000000
    y_limit = 4000000

    for y in range(y_limit+1):
        positions = list()
        for sensor, beacon in zip(sensors, beacons):
            d = get_dist(sensor, beacon)
            d_y = abs(sensor[1] - y)
            d_x = d - d_y
            if d_x >= 0:
                x_min = sensor[0] - d_x
                x_max = sensor[0] + d_x
                positions.append([x_min, x_max])

        positions.sort(key=lambda x: x[0])
        fp = [[0, 0]]
        for pos in positions:
            x_max = pos[1] if pos[1] < x_limit else x_limit
            x_min = pos[0] if pos[0] > 0 else 0

            if fp[-1][1] >= pos[0]-1:
                if fp[-1][1] < pos[1]:
                    fp[-1][1] = x_max
            else:
                fp.append([x_min, x_max])

            if fp[-1][1] == x_limit:
                break

        if len(fp) == 2:
            x_sol = fp[0][1]+1
            y_sol = y
            break

    result = x_sol*4000000 + y_sol
    print("Result 2: ", result)


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
