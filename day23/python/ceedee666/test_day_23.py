import unittest
from unittest import TestCase

import day_23

test_single_step_start = [
    ".....",
    "..##.",
    "..#..",
    ".....",
    "..##.",
    ".....",
]

test_single_step_1 = [
    "..##.",
    ".....",
    "..#..",
    "...#.",
    "..#..",
    ".....",
]
test_single_step_2 = [
    ".....",
    "..##.",
    ".#...",
    "....#",
    ".....",
    "..#..",
]
test_single_step_end = [
    "..#..",
    "....#",
    "#....",
    "....#",
    ".....",
    "..#..",
]


test_data = [
    "..............",
    "..............",
    ".......#......",
    ".....###.#....",
    "...#...#.#....",
    "....#...##....",
    "...#.###......",
    "...##.#.##....",
    "....#..#......",
    "..............",
    "..............",
    "..............",
]


class Testing(TestCase):
    def test_single_step(self):
        start = day_23.parse_input(test_single_step_start)
        one = day_23.parse_input(test_single_step_1)
        two = day_23.parse_input(test_single_step_2)
        end = day_23.parse_input(test_single_step_end)

        self.assertEqual(
            one, day_23.execute_step(start, 0), "The result should be correct."
        )

        self.assertEqual(
            two, day_23.execute_step(one, 1), "The result should be correct."
        )
        self.assertEqual(
            end, day_23.execute_step(two, 2), "The result should be correct."
        )

    def test_part_1(self):
        tiles, empty = day_23.solve_part_1(test_data)
        self.assertEqual(
            110,
            empty,
            "The result should be correct.",
        )

    def test_part_2(self):
        step = day_23.solve_part_2(test_data)
        self.assertEqual(
            20,
            step,
            "The result should be correct.",
        )


if __name__ == "__main__":
    unittest.main()
