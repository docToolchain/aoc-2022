import unittest
from unittest import TestCase

from day_14 import *

test_data = ["498,4 -> 498,6 -> 496,6", "503,4 -> 502,4 -> 502,9 -> 494,9"]


class Testing(TestCase):
    def test_part_1(self):
        r = solve_part_1(test_data)
        self.assertEqual(
            24,
            r,
            "The result shoud be correct.",
        )

    def test_part_2(self):
        r = solve_part_2(test_data)
        self.assertEqual(
            93,
            r,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
