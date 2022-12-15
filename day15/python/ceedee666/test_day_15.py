import unittest
from unittest import TestCase

from day_15 import *

test_data = [
    "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
    "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
    "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
    "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
    "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
    "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
    "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
    "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
    "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
    "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
    "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
    "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
    "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
    "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
]


class Testing(TestCase):
    def test_part_1(self):
        r = solve_part_1(test_data, 10)
        self.assertEqual(
            26,
            r,
            "The result shoud be correct.",
        )

    def test_part_2(self):
        r = solve_part_2(test_data, 20, 20)
        self.assertEqual(
            56000011,
            r,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
