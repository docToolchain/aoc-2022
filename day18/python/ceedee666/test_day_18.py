import unittest
from unittest import TestCase

import day_18

test_data = [
    "2,2,2",
    "1,2,2",
    "3,2,2",
    "2,1,2",
    "2,3,2",
    "2,2,1",
    "2,2,3",
    "2,2,4",
    "2,2,6",
    "1,2,5",
    "3,2,5",
    "2,1,5",
    "2,3,5",
]


class Testing(TestCase):
    def test_part_1(self):
        result = day_18.solve_part_1(test_data)
        self.assertEqual(
            64,
            result,
            "The result shoud be correct.",
        )

    def test_part_2(self):
        result = day_18.solve_part_2(test_data)
        self.assertEqual(
            58,
            result,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
