import unittest
from solution import Solution


class TestValidParenthesis(unittest.TestCase):
    def setUp(self):
        self.sol = Solution()

    def test_examples_true(self):
        for s in ("()", "([])", "{[()]}"):
            with self.subTest(s=s):
                self.assertTrue(self.sol.isValid(s))

    def test_examples_false(self):
        for s in ("(]", "([))", "(){}}{", "(", "])"):
            with self.subTest(s=s):
                self.assertFalse(self.sol.isValid(s))


if __name__ == "__main__":
    unittest.main()
