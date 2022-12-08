import unittest
from unittest import TestCase

from day_08 import *

test_data = ["30373", "25512", "65332", "33549", "35390"]


grid = [[int(c) for c in l.strip()] for l in test_data]


class Testing(TestCase):
    def test_part_1(self):
        r = solve_part_1(grid)

        self.assertEqual(
            21,
            r,
            "The result shoud be correct.",
        )

    def test_part_2(self):
        r = solve_part_2(grid)

        self.assertEqual(
            8,
            r,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
