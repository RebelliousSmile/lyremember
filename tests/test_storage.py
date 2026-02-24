"""Tests for storage module."""

import unittest
import tempfile
import shutil
from lyremember.storage import Storage
from lyremember.models import Song, SongProgress, PracticeSession


class TestStorage(unittest.TestCase):
    """Test Storage class."""

    def setUp(self):
        """Set up test fixtures."""
        self.test_dir = tempfile.mkdtemp()
        self.storage = Storage(self.test_dir)

    def tearDown(self):
        """Clean up test fixtures."""
        shutil.rmtree(self.test_dir)

    def test_save_and_get_song(self):
        """Test saving and retrieving a song."""
        song = Song(
            title="Test Song",
            artist="Test Artist",
            language="en",
            lyrics=["Line 1", "Line 2"]
        )

        self.storage.save_song(song)
        retrieved = self.storage.get_song(song.id)

        self.assertIsNotNone(retrieved)
        self.assertEqual(retrieved.title, "Test Song")
        self.assertEqual(retrieved.lyrics, ["Line 1", "Line 2"])

    def test_get_all_songs(self):
        """Test getting all songs."""
        song1 = Song(title="Song 1", artist="Artist", language="en", lyrics=["A"])
        song2 = Song(title="Song 2", artist="Artist", language="es", lyrics=["B"])

        self.storage.save_song(song1)
        self.storage.save_song(song2)

        songs = self.storage.get_all_songs()
        self.assertEqual(len(songs), 2)

    def test_delete_song(self):
        """Test deleting a song."""
        song = Song(title="Song", artist="Artist", language="en", lyrics=["A"])
        self.storage.save_song(song)

        result = self.storage.delete_song(song.id)
        self.assertTrue(result)

        retrieved = self.storage.get_song(song.id)
        self.assertIsNone(retrieved)

    def test_update_song(self):
        """Test updating a song."""
        song = Song(title="Original", artist="Artist", language="en", lyrics=["A"])
        self.storage.save_song(song)

        song.title = "Updated"
        self.storage.save_song(song)

        retrieved = self.storage.get_song(song.id)
        self.assertEqual(retrieved.title, "Updated")

    def test_save_and_get_progress(self):
        """Test saving and getting progress."""
        progress = SongProgress(song_id="test-id", mastery_level=0.5)
        self.storage.save_progress(progress)

        retrieved = self.storage.get_song_progress("test-id")
        self.assertIsNotNone(retrieved)
        self.assertEqual(retrieved.mastery_level, 0.5)


if __name__ == '__main__':
    unittest.main()
