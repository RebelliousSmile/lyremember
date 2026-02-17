"""Tests for utils module."""

import unittest
from lyremember.utils import (
    normalize_text,
    check_answer,
    calculate_similarity,
    hide_words,
    format_time,
    truncate_text
)


class TestUtils(unittest.TestCase):
    """Test utility functions."""
    
    def test_normalize_text(self):
        """Test text normalization."""
        self.assertEqual(normalize_text("Hello World!"), "hello world")
        self.assertEqual(normalize_text("  Multiple   Spaces  "), "multiple spaces")
        self.assertEqual(normalize_text("Punctuation..."), "punctuation")
    
    def test_check_answer_exact(self):
        """Test exact answer checking."""
        self.assertTrue(check_answer("hello", "hello"))
        self.assertTrue(check_answer("Hello", "HELLO"))  # Case insensitive
        self.assertTrue(check_answer("hello!", "hello"))  # Ignores punctuation
    
    def test_check_answer_fuzzy(self):
        """Test fuzzy answer checking."""
        # Minor typo should pass
        self.assertTrue(check_answer("helo", "hello"))
        # Very different should fail
        self.assertFalse(check_answer("goodbye", "hello"))
    
    def test_calculate_similarity(self):
        """Test similarity calculation."""
        # Identical
        self.assertEqual(calculate_similarity("test", "test"), 1.0)
        # Completely different
        self.assertLess(calculate_similarity("cat", "dog"), 0.5)
        # Similar
        self.assertGreater(calculate_similarity("hello", "hallo"), 0.7)
    
    def test_hide_words(self):
        """Test word hiding."""
        line = "Hello world this is a test"
        modified, hidden = hide_words(line, 0.5)
        
        # Should hide approximately 50% of words (3 out of 6)
        self.assertEqual(len(hidden), 3)
        # Modified line should contain underscores
        self.assertIn("_", modified)
    
    def test_format_time(self):
        """Test time formatting."""
        self.assertEqual(format_time(30), "30s")
        self.assertEqual(format_time(90), "1m 30s")
        self.assertEqual(format_time(3661), "1h 1m")
    
    def test_truncate_text(self):
        """Test text truncation."""
        short = "Short text"
        long = "This is a very long text that should be truncated"
        
        self.assertEqual(truncate_text(short, 20), short)
        self.assertTrue(truncate_text(long, 20).endswith("..."))
        self.assertEqual(len(truncate_text(long, 20)), 20)


if __name__ == '__main__':
    unittest.main()
