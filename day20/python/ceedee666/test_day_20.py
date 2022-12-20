import unittest
from unittest import TestCase

import day_20

test_data = [
    1,
    2,
    -3,
    3,
    -2,
    0,
    4,
]


class Testing(TestCase):
    def test_part_1(self):
        result = day_20.solve_part_1(test_data)
        self.assertEqual(
            3,
            result,
            "The result should be correct.",
        )

    def test_part_2(self):
        result = day_20.solve_part_2(test_data)
        self.assertEqual(
            1623178306,
            result,
            "The result should be correct.",
        )


if __name__ == "__main__":
    unittest.main()
