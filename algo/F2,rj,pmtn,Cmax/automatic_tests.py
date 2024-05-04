import unittest
from Johnson import run_algorithm as run_algorithm1
from Johnson_ver2 import run_algorithm as run_algorithm2
from neh import run_algorithm as run_algorithm3
from neh2 import run_algorithm as run_algorithm4
from neh import C_max as assess_len
class Tests(unittest.TestCase):
    #maxDiff = None
    #
    def test1(self):
        data1 = [(2, 3, 4), (3, 4, 2), (5, 6, 7)]
        result = run_algorithm1(data1)
        expected_result = {'result_1':  {1: [[2, 3], [3, 5]], 2: [[11, 15]], 3: [[5, 11]]},
                           'result_2': {1: [[5, 9]], 2: [[18, 20]], 3: [[11, 18]]}}
        result.pop('c_max', None)
        self.assertEqual(result, expected_result)

    def test2(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4)]
        result = run_algorithm1(data2)
        expected_result = {'result_1':  {1: [[1, 2], [2, 3]], 2: [[3, 6]], 3: [[6, 7], [7, 9]], 4: [[9, 10], [11, 12], [15, 21]], 5: [[10, 11]], 6: [[12, 15]]},
                           'result_2': {1: [[3, 6]], 2: [[6, 10]], 3: [[10, 14]], 4: [[21, 23]], 5: [[14, 16]], 6: [[16, 20]]}}
        result.pop('c_max', None)
        self.assertEqual(result, expected_result)

    def test3(self):
        data1 = [(2, 3, 4), (3, 4, 2), (5, 6, 7)]
        result = run_algorithm2(data1)
        expected_result = {'result_1':  {1: [[2, 3], [3, 5]], 2: [[11, 15]], 3: [[5, 11]]},
                           'result_2': {1: [[5, 9]], 2: [[18, 20]], 3: [[11, 18]]}}
        result.pop('c_max', None)
        self.assertEqual(result, expected_result)

    def test4(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4)]
        result = run_algorithm2(data2)
        expected_result = {'result_1':  {1: [[1, 2], [2, 3]], 2: [[3, 6]], 3: [[6, 7], [7, 9]], 4: [[9, 10], [11, 12], [15, 21]], 5: [[10, 11]], 6: [[12, 15]]},
                           'result_2': {1: [[3, 6]], 2: [[6, 10]], 3: [[10, 14]], 4: [[21, 23]], 5: [[14, 16]], 6: [[16, 20]]}}
        result.pop('c_max', None)
        self.assertEqual(result, expected_result)

    def test5(self):
        data1 = [(2, 3, 4), (3, 4, 2), (5, 6, 7)]
        result = run_algorithm3(data1)
        expected_result = {'result_1':  {1: [[2, 3], [3, 5]], 2: [[11, 15]], 3: [[5, 11]]},
                           'result_2': {1: [[5, 9]], 2: [[18, 20]], 3: [[11, 18]]}}
        result.pop('c_max', None)
        self.assertEqual(result, expected_result)

    def test6(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4)]
        result = run_algorithm3(data2)
        expected_result = {'result_1':  {1: [[1, 2], [2, 3]], 2: [[3, 6]], 3: [[6, 7], [7, 9]], 4: [[9, 10], [11, 12], [15, 21]], 5: [[10, 11]], 6: [[12, 15]]},
                           'result_2': {1: [[3, 6]], 2: [[6, 10]], 3: [[10, 14]], 4: [[21, 23]], 5: [[14, 16]], 6: [[16, 20]]}}
        result.pop('c_max', None)
        self.assertEqual(result, expected_result)

    def test7(self):
        data1 = [(2, 3, 4), (3, 4, 2), (5, 6, 7)]
        result = run_algorithm4(data1)
        expected_result = {'result_1':  {1: [[2, 3], [3, 5]], 2: [[11, 15]], 3: [[5, 11]]},
                           'result_2': {1: [[5, 9]], 2: [[18, 20]], 3: [[11, 18]]}}
        result.pop('c_max', None)
        self.assertEqual(result, expected_result)

    def test8(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4)]
        result = run_algorithm4(data2)
        expected_result = {'result_1':  {1: [[1, 2], [2, 3]], 2: [[3, 6]], 3: [[6, 7], [7, 9]], 4: [[9, 10], [11, 12], [15, 21]], 5: [[10, 11]], 6: [[12, 15]]},
                           'result_2': {1: [[3, 6]], 2: [[6, 10]], 3: [[10, 14]], 4: [[21, 23]], 5: [[14, 16]], 6: [[16, 20]]}}
        result.pop('c_max', None)
        self.assertEqual(result, expected_result)

    def test9(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4), (13, 4, 5), (14, 1, 2), (18, 5, 4), (23, 4, 5), (27, 8, 9), (28, 1, 2)]
        expected_c_max = 49
        result = run_algorithm1(data2)
        self.assertEqual(result['c_max'], expected_c_max)

    def test10(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4), (13, 4, 5), (14, 1, 2), (18, 5, 4), (23, 4, 5), (27, 8, 9), (28, 1, 2)]
        expected_c_max = 49
        result = run_algorithm2(data2)
        self.assertAlmostEqual(result['c_max'], expected_c_max, delta = 10)

    def test11(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4), (13, 4, 5), (14, 1, 2), (18, 5, 4), (23, 4, 5), (27, 8, 9), (28, 1, 2)]
        expected_c_max = 49
        result = run_algorithm4(data2)
        self.assertEqual(result['c_max'], expected_c_max)

    def test12(self):
        data2 = [(1, 2, 3), (2, 3, 4), (6, 3, 4), (7, 8, 2), (10, 1, 2), (12, 3, 4), (13, 4, 5), (14, 1, 2), (18, 5, 4), (23, 4, 5), (27, 8, 9), (28, 1, 2)]
        expected_c_max = 49
        result = run_algorithm3(data2)
        self.assertAlmostEqual(result['c_max'], expected_c_max, delta = 10)

    def test13(self):
        data = [(0,1,3),(2,2,3),(3,3,4), (5,5,2),(7,3,1) ]
        expected_result = 15
        result = assess_len(data)
        self.assertEqual(expected_result, result)


if __name__ == '__main__':
    unittest.main()
