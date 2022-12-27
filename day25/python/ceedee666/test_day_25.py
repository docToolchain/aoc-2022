import unittest
from unittest import TestCase

import day_25

test_data_to_dec = {
    "1=-0-2": 1747,
    "12111": 906,
    "2=0=": 198,
    "21": 11,
    "2=01": 201,
    "111": 31,
    "20012": 1257,
    "112": 32,
    "1=-1=": 353,
    "1-12": 107,
    "12": 7,
    "1=": 3,
    "122": 37,
}
test_data_to_snafu = {
    1: "1",
    2: "2",
    3: "1=",
    4: "1-",
    5: "10",
    6: "11",
    7: "12",
    8: "2=",
    9: "2-",
    10: "20",
    15: "1=0",
    20: "1-0",
    2022: "1=11-2",
    12345: "1-0---0",
    314159265: "1121-1110-1=0",
}


class Testing(TestCase):
    def test_convert_to_dec(self):
        for s in test_data_to_dec:
            dec_num = day_25.convert_to_decimal(s)
            self.assertEqual(
                test_data_to_dec[s],
                dec_num,
                "The result should be correct.",
            )

    def test_convert_to_snafu(self):
        for n in test_data_to_snafu:
            snafu_num = day_25.convert_to_snafu(n)
            self.assertEqual(
                test_data_to_snafu[n],
                snafu_num,
                "The result should be correct.",
            )


if __name__ == "__main__":
    unittest.main()
