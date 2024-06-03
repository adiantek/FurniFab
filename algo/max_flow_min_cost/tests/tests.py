import unittest
from ..negative_cycle_removal import start

class Tests(unittest.TestCase):

    def test1(self):
        data = "max_flow_min_cost/tests/test_01.txt"
        file = open(data)
        f = file.readlines()
        file.close()
        result = start(f)[1:]
        expected_result = (10, 160)

        self.assertEqual(result, expected_result)

    def test2(self):
        data = "max_flow_min_cost/tests/test_02.txt"
        file = open(data)
        f = file.readlines()
        file.close()
        result = start(f)[1:]
        expected_result = (37, 546)

        self.assertEqual(result, expected_result)

    def test3(self):
        data = "max_flow_min_cost/tests/test_03.txt"
        file = open(data)
        f = file.readlines()
        file.close()
        result = start(f)[1:]
        expected_result = (30, 344)

        self.assertEqual(result, expected_result)

    def test4(self):
        data = "max_flow_min_cost/tests/test_04.txt"
        file = open(data)
        f = file.readlines()
        file.close()
        result = start(f)[1:]
        expected_result = (15, 70)

        self.assertEqual(result, expected_result)

    def test5(self):
        data = "max_flow_min_cost/tests/test_05.txt"
        file = open(data)
        f = file.readlines()
        file.close()
        result = start(f)[1:]
        expected_result = (12, 26)

        self.assertEqual(result, expected_result)

    def test6(self):
        data = "max_flow_min_cost/tests/test_06.txt"
        file = open(data)
        f = file.readlines()
        file.close()
        result = start(f)[1:]
        expected_result = (35, 305)

        self.assertEqual(result, expected_result)

    def test7(self):
        data = "max_flow_min_cost/tests/test_07.txt"
        file = open(data)
        f = file.readlines()
        file.close()
        result = start(f)[1:]
        expected_result = (25, 150)

        self.assertEqual(result, expected_result)

    def test8(self):
        data = "max_flow_min_cost/tests/test_08.txt"
        file = open(data)
        f = file.readlines()
        file.close()
        result = start(f)[1:]
        expected_result = (10, 55)

        self.assertEqual(result, expected_result)

if __name__ == '__main__':
    unittest.main()