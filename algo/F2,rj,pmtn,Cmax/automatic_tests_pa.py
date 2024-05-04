import unittest
from pa import run_algorithm

class TestPa(unittest.TestCase):
    def test1(self):
      data = [(1, 1, 2), (2, 3, 4), (3, 1, 2)]
      result = run_algorithm(data)
      expected_result = {'result_1': {1: [[1, 2]], 2: [[2, 3], [4, 6]], 3: [[3, 4]]},
                         'result_2': {1: [[2, 4]], 2: [[6, 10]], 3: [[4, 6]]}}
      self.assertEqual(result, expected_result)

    def test2(self):
        data = [(1, 1, 2), (2, 3, 4), (3, 1, 2),(5, 6, 7), (8, 1, 4), (10, 2, 3)]
        result = run_algorithm(data)
        expected_result = {'result_1': {1: [[1, 2]], 2: [[2, 3], [4, 5], [5, 6]], 3: [[3, 4]], 4: [[6, 8], [9, 10], [12, 15]], 5: [[8, 9]], 6: [[10, 12]]},
                           'result_2': {1: [[2, 4]], 2: [[6, 10]], 3: [[4, 6]], 4: [[17, 24]], 5: [[10, 14]], 6: [[14, 17]]}}
        self.assertEqual(result, expected_result)



if __name__ == '__main__':
    unittest.main()
