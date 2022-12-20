import unittest
from unittest import TestCase

import day_19

test_data = [
    "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
    "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
]


class Testing(TestCase):
    def test_part_1(self):
        result = day_19.solve_part_1(test_data)
        self.assertEqual(
            33,
            result,
            "The result should be correct.",
        )

    def test_bp_27(self):
        result = day_19.solve_part_1(
            [
                "Blueprint 27: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 2 ore and 7 obsidian."
            ]
        )
        self.assertEqual(
            351,
            result,
            "The result should be correct.",
        )


if __name__ == "__main__":
    unittest.main()
