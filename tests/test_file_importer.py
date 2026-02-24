"""Tests for file importer module."""

import json
import os
import tempfile
import unittest

from lyremember.file_importer import FileImportError, import_file


class TestImportFileTxt(unittest.TestCase):
    """Test importing lyrics from plain-text files."""

    def _write(self, content: str, suffix: str = '.txt') -> str:
        fd, path = tempfile.mkstemp(suffix=suffix)
        os.close(fd)
        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)
        self.addCleanup(os.unlink, path)
        return path

    # ------------------------------------------------------------------
    # Happy paths
    # ------------------------------------------------------------------

    def test_txt_basic(self):
        """Every non-empty line becomes a lyric line."""
        path = self._write("Line one\nLine two\nLine three\n")
        title, artist, language, lyrics = import_file(
            path, title="T", artist="A", language="en"
        )
        self.assertEqual(lyrics, ["Line one", "Line two", "Line three"])
        self.assertEqual(title, "T")
        self.assertEqual(artist, "A")
        self.assertEqual(language, "en")

    def test_txt_skips_blank_lines(self):
        """Blank lines inside a text file are skipped."""
        path = self._write("Line 1\n\nLine 2\n\n")
        _, _, _, lyrics = import_file(path, title="T", artist="A", language="en")
        self.assertEqual(lyrics, ["Line 1", "Line 2"])

    def test_txt_skips_comment_lines(self):
        """Lines starting with '#' are ignored."""
        path = self._write("# This is a comment\nLine 1\n# Another comment\nLine 2\n")
        _, _, _, lyrics = import_file(path, title="T", artist="A", language="en")
        self.assertEqual(lyrics, ["Line 1", "Line 2"])

    def test_txt_preserves_whitespace_in_lyrics(self):
        """Leading/trailing spaces within a lyric line are preserved (not stripped)."""
        path = self._write("  indented line  \nnormal line\n")
        _, _, _, lyrics = import_file(path, title="T", artist="A", language="en")
        # Only trailing newline is stripped, inner spaces stay
        self.assertEqual(lyrics[0], "  indented line  ")

    # ------------------------------------------------------------------
    # Error paths
    # ------------------------------------------------------------------

    def test_txt_empty_file_raises(self):
        """An empty text file raises FileImportError."""
        path = self._write("")
        with self.assertRaises(FileImportError):
            import_file(path, title="T", artist="A", language="en")

    def test_txt_only_comments_raises(self):
        """A file with only comments raises FileImportError."""
        path = self._write("# only a comment\n")
        with self.assertRaises(FileImportError):
            import_file(path, title="T", artist="A", language="en")

    def test_file_not_found_raises(self):
        """A non-existent file raises FileNotFoundError."""
        with self.assertRaises(FileNotFoundError):
            import_file("/non/existent/file.txt", title="T", artist="A", language="en")

    def test_unsupported_extension_raises(self):
        """An unsupported file extension raises FileImportError."""
        path = self._write("some content", suffix='.pdf')
        with self.assertRaises(FileImportError):
            import_file(path, title="T", artist="A", language="en")


class TestImportFileJson(unittest.TestCase):
    """Test importing lyrics from JSON files."""

    def _write_json(self, data: dict) -> str:
        fd, path = tempfile.mkstemp(suffix='.json')
        os.close(fd)
        with open(path, 'w', encoding='utf-8') as f:
            json.dump(data, f)
        self.addCleanup(os.unlink, path)
        return path

    # ------------------------------------------------------------------
    # Happy paths
    # ------------------------------------------------------------------

    def test_json_full_metadata(self):
        """JSON file with all fields is read correctly."""
        path = self._write_json({
            "title": "My Song",
            "artist": "My Artist",
            "language": "fr",
            "lyrics": ["Bonjour", "Au revoir"]
        })
        title, artist, language, lyrics = import_file(path)
        self.assertEqual(title, "My Song")
        self.assertEqual(artist, "My Artist")
        self.assertEqual(language, "fr")
        self.assertEqual(lyrics, ["Bonjour", "Au revoir"])

    def test_json_cli_options_override_file(self):
        """CLI options take precedence over values in the JSON file."""
        path = self._write_json({
            "title": "File Title",
            "artist": "File Artist",
            "language": "es",
            "lyrics": ["Hola"]
        })
        title, artist, language, _ = import_file(
            path, title="CLI Title", artist="CLI Artist", language="en"
        )
        self.assertEqual(title, "CLI Title")
        self.assertEqual(artist, "CLI Artist")
        self.assertEqual(language, "en")

    def test_json_partial_metadata(self):
        """A JSON file with only lyrics returns None for missing fields."""
        path = self._write_json({"lyrics": ["Only a line"]})
        title, artist, language, lyrics = import_file(path)
        self.assertIsNone(title)
        self.assertIsNone(artist)
        self.assertIsNone(language)
        self.assertEqual(lyrics, ["Only a line"])

    # ------------------------------------------------------------------
    # Error paths
    # ------------------------------------------------------------------

    def test_json_invalid_syntax_raises(self):
        """Malformed JSON raises FileImportError."""
        fd, path = tempfile.mkstemp(suffix='.json')
        os.close(fd)
        self.addCleanup(os.unlink, path)
        with open(path, 'w') as f:
            f.write("not valid json {{{")
        with self.assertRaises(FileImportError):
            import_file(path, title="T", artist="A", language="en")

    def test_json_list_root_raises(self):
        """A JSON array at the root raises FileImportError."""
        fd, path = tempfile.mkstemp(suffix='.json')
        os.close(fd)
        self.addCleanup(os.unlink, path)
        with open(path, 'w') as f:
            json.dump(["line 1", "line 2"], f)
        with self.assertRaises(FileImportError):
            import_file(path, title="T", artist="A", language="en")

    def test_json_empty_lyrics_raises(self):
        """A JSON file with an empty lyrics list raises FileImportError."""
        path = self._write_json({"title": "T", "artist": "A", "language": "en", "lyrics": []})
        with self.assertRaises(FileImportError):
            import_file(path)

    def test_json_lyrics_not_list_raises(self):
        """A JSON file where 'lyrics' is not a list raises FileImportError."""
        path = self._write_json({"lyrics": "just a string"})
        with self.assertRaises(FileImportError):
            import_file(path)


if __name__ == '__main__':
    unittest.main()
