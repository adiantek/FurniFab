import unittest
from BB import run_algorithm as run_algorithm1
from vns import run_algorithm as run_algorithm2
from Horn import run_algorithm as run_algorithm3
from controller import run_algorithm as run_algorithm4

class Tests(unittest.TestCase):

    def test1(self):
        data1 = [(2, 3, 4), (3, 4, 2), (5, 6, 7)]
        expected_c_max = 20
        result = run_algorithm1(data1)
        self.assertEqual(result['c_max'], expected_c_max)

    def test2(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4)]
        expected_c_max = 23
        result = run_algorithm1(data2)
        self.assertEqual(result['c_max'], expected_c_max)

    def test3(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4), (13, 4, 5), (14, 1, 2), (18, 5, 4), (23, 4, 5), (27, 8, 9), (28, 1, 2)]
        expected_c_max = 49
        result = run_algorithm2(data2)
        self.assertAlmostEqual(result['c_max'], expected_c_max, delta = 20)

    def test4(self):
        data1 = [(2, 3, 4), (3, 4, 2), (5, 6, 7)]
        expected_c_max = 20
        result = run_algorithm3(data1)
        self.assertEqual(result['c_max'], expected_c_max)

    def test5(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4)]
        expected_c_max = 23
        result = run_algorithm3(data2)
        self.assertEqual(result['c_max'], expected_c_max)

    def test6(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4), (13, 4, 5), (14, 1, 2), (18, 5, 4), (23, 4, 5), (27, 8, 9), (28, 1, 2)]
        expected_c_max = 49
        result = run_algorithm3(data2)
        self.assertEqual(result['c_max'], expected_c_max)

    def test7(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4), (13, 4, 5), (14, 1, 2), (18, 5, 4), (23, 4, 5), (27, 8, 9), (28, 1, 2)]
        expected_c_max = 49
        result = run_algorithm4(data2)
        self.assertEqual(result['c_max'], expected_c_max)

    def test8(self):
        data1 = [(2, 3, 4), (3, 4, 2), (5, 6, 7)]
        expected_c_max = 20
        result = run_algorithm4(data1)
        self.assertEqual(result['c_max'], expected_c_max)
if __name__ == '__main__':
    unittest.main()
