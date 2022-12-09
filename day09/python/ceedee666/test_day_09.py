import unittest
from unittest import TestCase

from day_09 import *


class Testing(TestCase):
    def test_part_1(self):
        test_data = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"]
        moves = [l.strip().split() for l in test_data]

        r = solve_part_1(moves)

        self.assertEqual(
            13,
            r,
            "The result shoud be correct.",
        )

    def test_part_2(self):
        test_data = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"]
        test_data_2 = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"]
        moves = [l.strip().split() for l in test_data]
        moves_2 = [l.strip().split() for l in test_data_2]

        r = solve_part_2(moves)

        self.assertEqual(
            1,
            r,
            "The result shoud be correct.",
        )

        r = solve_part_2(moves_2)

        self.assertEqual(
            36,
            r,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
