import unittest
from unittest import TestCase

from day_07 import *

test_string = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""


class Testing(TestCase):
    def test_dummy(self):
        r = solve_part_1(test_string.split("\n"))

        self.assertEqual(
            95437,
            r,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
