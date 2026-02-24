"""Tests for storage module."""

import unittest
import tempfile
import shutil
from pathlib import Path
from lyremember.storage import Storage
from lyremember.models import Song, SongProgress, PracticeSession


class TestStorage(unittest.TestCase):
    """Test Storage class."""
    
    def setUp(self):
        """Set up test fixtures."""
        # Create temporary directory for test data
        self.test_dir = tempfile.mkdtemp()
        self.storage = Storage(self.test_dir)
    
    def tearDown(self):
        """Clean up test fixtures."""
        # Remove temporary directory
        shutil.rmtree(self.test_dir)
    
    def test_init_creates_files(self):
        """Test that initialization creates necessary files."""
        self.assertTrue((Path(self.test_dir) / 'songs.json').exists())
        self.assertTrue((Path(self.test_dir) / 'progress.json').exists())
        self.assertTrue((Path(self.test_dir) / 'config.json').exists())
    
    def test_save_and_get_song(self):
        """Test saving and retrieving a song."""
        song = Song(
            title="Test Song",
            artist="Test Artist",
            language="en",
            lyrics=["Line 1", "Line 2", "Line 3"]
        )
        
        self.storage.save_song(song)
        retrieved = self.storage.get_song(song.id)
        
        self.assertIsNotNone(retrieved)
        self.assertEqual(retrieved.title, "Test Song")
        self.assertEqual(retrieved.artist, "Test Artist")
        self.assertEqual(len(retrieved.lyrics), 3)
    
    def test_get_all_songs(self):
        """Test getting all songs."""
        song1 = Song(title="Song 1", artist="Artist 1", language="en", lyrics=["A"])
        song2 = Song(title="Song 2", artist="Artist 2", language="es", lyrics=["B"])
        
        self.storage.save_song(song1)
        self.storage.save_song(song2)
        
        all_songs = self.storage.get_all_songs()
        self.assertEqual(len(all_songs), 2)
    
    def test_delete_song(self):
        """Test deleting a song."""
        song = Song(title="To Delete", artist="Artist", language="en", lyrics=["A"])
        self.storage.save_song(song)
        
        result = self.storage.delete_song(song.id)
        self.assertTrue(result)
        
        retrieved = self.storage.get_song(song.id)
        self.assertIsNone(retrieved)
    
    def test_save_and_get_progress(self):
        """Test saving and retrieving progress."""
        session = PracticeSession(
            song_id="test-song-id",
            mode="fill-blank",
            score=85.0,
            lines_practiced=10,
            lines_correct=8
        )
        
        progress = SongProgress(song_id="test-song-id")
        progress.practice_sessions.append(session)
        progress.mastery_level = 0.85
        
        self.storage.save_progress(progress)
        retrieved = self.storage.get_song_progress("test-song-id")
        
        self.assertIsNotNone(retrieved)
        self.assertEqual(len(retrieved.practice_sessions), 1)
        self.assertEqual(retrieved.mastery_level, 0.85)


if __name__ == '__main__':
    unittest.main()
