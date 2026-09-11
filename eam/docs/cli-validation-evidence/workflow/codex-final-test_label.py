import unittest
from label import label

class LabelTests(unittest.TestCase):
    def test_korean(self):
        self.assertEqual(label("  한글   프로젝트 "), "한글 프로젝트")
    def test_multiline(self):
        self.assertEqual(label("alpha\n\tbeta"), "alpha beta")
    def test_emoji(self):
        self.assertEqual(label("  코드🙂  "), "코드🙂")
    def test_empty(self):
        with self.assertRaises(ValueError):
            label(" \t\n")
    def test_none(self):
        with self.assertRaises(TypeError):
            label(None)
    def test_integer(self):
        with self.assertRaises(TypeError):
            label(123)

if __name__ == '__main__':
    unittest.main()
