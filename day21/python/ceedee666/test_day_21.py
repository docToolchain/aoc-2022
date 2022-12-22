import unittest
from unittest import TestCase

import day_21

test_data = [
    "root: pppw + sjmn",
    "dbpl: 5",
    "cczh: sllz + lgvd",
    "zczc: 2",
    "ptdq: humn - dvpt",
    "dvpt: 3",
    "lfqf: 4",
    "humn: 5",
    "ljgn: 2",
    "sjmn: drzm * dbpl",
    "sllz: 4",
    "pppw: cczh / lfqf",
    "lgvd: ljgn * ptdq",
    "drzm: hmdt - zczc",
    "hmdt: 32",
]


class Testing(TestCase):
    def test_part_2(self):
        result = day_21.solve_part_2(test_data)
        self.assertEqual(
            301,
            result,
            "The result should be correct.",
        )


if __name__ == "__main__":
    unittest.main()
