import unittest
from unittest import TestCase

import day_24

test_data = [
    "#.######",
    "#>>.<^<#",
    "#.<..<<#",
    "#>v.><>#",
    "#<^v^^>#",
    "######.#",
]


class Testing(TestCase):
    def test_part_1(self):
        time = day_24.solve_part_1(test_data)
        self.assertEqual(
            18,
            time,
            "The result should be correct.",
        )

    def test_part_2(self):
        time = day_24.solve_part_2(test_data)
        self.assertEqual(
            54,
            time,
            "The result should be correct.",
        )


if __name__ == "__main__":
    unittest.main()
