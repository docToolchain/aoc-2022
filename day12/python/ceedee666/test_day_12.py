import unittest
from unittest import TestCase

from day_12 import *

test_data = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"]


class Testing(TestCase):
    def test_part_1(self):
        r = solve_part_1(test_data)
        self.assertEqual(
            31,
            r,
            "The result shoud be correct.",
        )

    def test_part_2(self):
        r = solve_part_2(test_data)
        self.assertEqual(
            29,
            r,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
