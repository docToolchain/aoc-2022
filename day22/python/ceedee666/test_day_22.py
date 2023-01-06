import unittest
from unittest import TestCase

import day_22

test_data = [
    "        ...#",
    "        .#..",
    "        #...",
    "        ....",
    "...#.......#",
    "........#...",
    "..#....#....",
    "..........#.",
    "        ...#....",
    "        .....#..",
    "        .#......",
    "        ......#.",
    "",
    "10R5L5R10L4R5L5",
]


class Testing(TestCase):
    def test_part_1(self):
        result = day_22.solve_part_1(test_data)
        self.assertEqual(
            6032,
            result,
            "The result should be correct.",
        )


if __name__ == "__main__":
    unittest.main()
