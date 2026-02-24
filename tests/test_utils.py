"""Tests for utility functions."""

import unittest
from lyremember.utils import normalize_text, check_answer, truncate_text, format_time, hide_words


class TestUtils(unittest.TestCase):
    """Test utility functions."""

    def test_normalize_text(self):
        """Test text normalization."""
        self.assertEqual(normalize_text("  Hello World  "), "hello world")
        self.assertEqual(normalize_text("Hello, World!"), "hello world")
        self.assertEqual(normalize_text("UPPER CASE"), "upper case")

    def test_check_answer_exact(self):
        """Test exact answer matching."""
        self.assertTrue(check_answer("hello world", "hello world"))
        self.assertTrue(check_answer("Hello World", "hello world"))
        self.assertFalse(check_answer("wrong answer", "hello world"))

    def test_check_answer_fuzzy(self):
        """Test fuzzy answer matching."""
        # Minor typo should pass
        self.assertTrue(check_answer("helo world", "hello world"))

    def test_truncate_text(self):
        """Test text truncation."""
        self.assertEqual(truncate_text("short", 10), "short")
        self.assertEqual(len(truncate_text("a" * 100, 20)), 20)
        self.assertTrue(truncate_text("a" * 100, 20).endswith("..."))

    def test_format_time(self):
        """Test time formatting."""
        self.assertEqual(format_time(30), "30s")
        self.assertEqual(format_time(90), "1m 30s")
        self.assertEqual(format_time(3661), "1h 1m")

    def test_hide_words(self):
        """Test word hiding."""
        line = "Twinkle twinkle little star"
        modified, hidden = hide_words(line, 0.5)
        self.assertTrue(len(hidden) > 0)
        self.assertIn('_', modified)


if __name__ == '__main__':
    unittest.main()
