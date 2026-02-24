"""Tests for song manager module."""

import unittest
import tempfile
import shutil
from lyremember.storage import Storage
from lyremember.song_manager import SongManager


class TestSongManager(unittest.TestCase):
    """Test SongManager class."""

    def setUp(self):
        """Set up test fixtures."""
        self.test_dir = tempfile.mkdtemp()
        self.storage = Storage(self.test_dir)
        self.manager = SongManager(self.storage)

    def tearDown(self):
        """Clean up test fixtures."""
        shutil.rmtree(self.test_dir)

    def test_add_song(self):
        """Test adding a song."""
        song = self.manager.add_song(
            title="Test Song",
            artist="Test Artist",
            language="en",
            lyrics=["Line 1", "Line 2"]
        )

        self.assertIsNotNone(song.id)
        self.assertEqual(song.title, "Test Song")
        self.assertEqual(song.artist, "Test Artist")

    def test_get_song(self):
        """Test getting a song."""
        song = self.manager.add_song("Song", "Artist", "en", ["A"])
        retrieved = self.manager.get_song(song.id)

        self.assertIsNotNone(retrieved)
        self.assertEqual(retrieved.id, song.id)

    def test_search_songs(self):
        """Test searching songs."""
        self.manager.add_song("Love Song", "Artist 1", "en", ["A"])
        self.manager.add_song("Happy Song", "Artist 2", "en", ["B"])
        self.manager.add_song("Sad Melody", "Love Band", "en", ["C"])

        results = self.manager.search_songs("love")
        self.assertEqual(len(results), 2)  # "Love Song" and by "Love Band"

    def test_filter_by_language(self):
        """Test filtering by language."""
        self.manager.add_song("English Song", "Artist", "en", ["A"])
        self.manager.add_song("Spanish Song", "Artist", "es", ["B"])
        self.manager.add_song("French Song", "Artist", "fr", ["C"])

        spanish_songs = self.manager.filter_by_language("es")
        self.assertEqual(len(spanish_songs), 1)
        self.assertEqual(spanish_songs[0].language, "es")

    def test_add_translation(self):
        """Test adding a translation."""
        song = self.manager.add_song("Song", "Artist", "en", ["Hello world"])

        result = self.manager.add_translation(
            song.id,
            "es",
            ["Hola mundo"]
        )

        self.assertTrue(result)

        updated = self.manager.get_song(song.id)
        self.assertIn("es", updated.translations)
        self.assertEqual(updated.translations["es"], ["Hola mundo"])


if __name__ == '__main__':
    unittest.main()
