import unittest
from unittest import TestCase

import day_0


class Testing(TestCase):
    def test_dummy(self):
        result = False
        self.assertEqual(
            True,
            result,
            "The result shoud be correct.",
        )


if __name__ == "__main__":
    unittest.main()
