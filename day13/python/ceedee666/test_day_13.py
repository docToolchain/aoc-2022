import unittest
from unittest import TestCase

from day_13 import *

test_data = [
    "[1,1,3,1,1]",
    "[1,1,5,1,1]",
    "",
    "[[1],[2,3,4]]",
    "[[1],4]",
    "",
    "[9]",
    "[[8,7,6]]",
    "",
    "[[4,4],4,4]",
    "[[4,4],4,4,4]",
    "",
    "[7,7,7,7]",
    "[7,7,7]",
    "",
    "[]",
    "[3]",
    "",
    "[[[]]]",
    "[[]]",
    "",
    "[1,[2,[3,[4,[5,6,7]]]],8,9]",
    "[1,[2,[3,[4,[5,6,0]]]],8,9]",
]


class Testing(TestCase):
    def test_part_1(self):
        r = solve_part_1(test_data)
        self.assertEqual(
            13,
            r,
            "The result shoud be correct.",
        )

    def test_part_2(self):
        r = solve_part_2(test_data)
        self.assertEqual(
            140,
            r,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
