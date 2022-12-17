#!/usr/bin/env python3


SHAPES = [[(0, 2), (0, 3), (0, 4), (0, 5)],
          [(0, 3), (1, 2), (1, 3), (1, 4), (2, 3)],
          [(0, 2), (0, 3), (0, 4), (1, 4), (2, 4)],
          [(0, 2), (1, 2), (2, 2), (3, 2)],
          [(0, 2), (0, 3), (1, 2), (1, 3)]]


def load_input():
    with open('input') as fd:
        data = fd.read().strip()
    return data


def calc_start(shape, y_max):
    result = [(y+y_max+4, x) for y, x in shape]
    return result


def move_down(shape):
    return [(y-1, x) for y, x in shape]


def process_jet_gas(shape, gas):
    d = 1 if gas == '>' else -1
    return [(y, x+d) for y, x in shape]


def is_shape_in_field(field, shape):
    result = False
    for pos in shape:
        if pos in field:
            result = True
            break
    return result


def parts():
    data = load_input()
    field = set(((0, j) for j in range(7)))
    y_max = 0
    jet_count = 0

    prev_max = 0

    # Preparation for part2
    value = 0
    steps = 1000000000000
    # Period must be manually determined
    period = 87*100
    period_value = 13405
    init_value = 13404
    init_step = period
    rem_step = steps - ((steps - init_step)//period)*period - init_step
    rem_value = 0

    for i in range(20000):

        # Determine period
        # if i%(100*87) == 0:
        # Example Period
        # if i%(40*7) == 0:
            # print(i)
            # print(y_max - prev_max)
            # print("")
            # prev_max = y_max

        shape = calc_start(SHAPES[i % len(SHAPES)], y_max)
        while True:
            gas = data[jet_count % len(data)]
            jet_count += 1
            next_shape = process_jet_gas(shape, gas)
            xs = [x for _, x in next_shape]
            if not(min(xs) < 0 or max(xs) > 6):
                if not(is_shape_in_field(field, next_shape)):
                    shape = next_shape

            next_shape = move_down(shape)
            if is_shape_in_field(field, next_shape):
                break
            else:
                shape = next_shape
        field.update(set(shape))
        y_s = [y for y, _ in shape]
        y_ms = max(y_s)
        y_max = y_ms if y_ms > y_max else y_max

        # Required for part2
        if i == init_step + rem_step - 1:
            rem_value = y_max - init_value

        # Part 1
        if i == 2022-1:
            result = y_max

    print("Result 1: ", result)

    value = init_value + period_value*((steps - init_step)//period) + rem_value
    print("Result 2: ", value)


def main():
    parts()


if __name__ == '__main__':
    main()
