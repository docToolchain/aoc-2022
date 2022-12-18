import unittest
from unittest import TestCase

from day_17 import *

test_data = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"


class Testing(TestCase):
    def test_part_1(self):
        r = solve_part_1(test_data)
        self.assertEqual(
            3068,
            r,
            "The result should be correct.",
        )

    def test_part_2(self):
        r = solve_part_2(test_data)
        self.assertEqual(
            1514285714288,
            r,
            "The result should be correct.",
        )


if __name__ == "__main__":
    unittest.main()
