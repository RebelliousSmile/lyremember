"""Song management for LyRemember."""

from typing import List, Optional
from lyremember.models import Song
from lyremember.storage import Storage


class SongManager:
    """Manages songs in the application."""

    def __init__(self, storage: Storage):
        """Initialize song manager with storage."""
        self.storage = storage

    def add_song(self, title: str, artist: str, language: str, lyrics: List[str]) -> Song:
        """
        Add a new song.

        Args:
            title: Song title
            artist: Artist name
            language: Language code (e.g., 'en', 'es', 'fr')
            lyrics: List of lyrics lines

        Returns:
            The created Song object
        """
        song = Song(
            title=title,
            artist=artist,
            language=language,
            lyrics=lyrics
        )
        self.storage.save_song(song)
        return song

    def get_song(self, song_id: str) -> Optional[Song]:
        """Get a song by ID."""
        return self.storage.get_song(song_id)

    def get_all_songs(self) -> List[Song]:
        """Get all songs."""
        return self.storage.get_all_songs()

    def update_song(self, song: Song) -> None:
        """Update an existing song."""
        self.storage.save_song(song)

    def delete_song(self, song_id: str) -> bool:
        """Delete a song by ID."""
        return self.storage.delete_song(song_id)

    def search_songs(self, query: str) -> List[Song]:
        """
        Search songs by title or artist.

        Args:
            query: Search query

        Returns:
            List of matching songs
        """
        query_lower = query.lower()
        all_songs = self.get_all_songs()

        return [
            song for song in all_songs
            if query_lower in song.title.lower() or query_lower in song.artist.lower()
        ]

    def filter_by_language(self, language: str) -> List[Song]:
        """Filter songs by language."""
        all_songs = self.get_all_songs()
        return [song for song in all_songs if song.language == language]

    def add_translation(self, song_id: str, language: str, translation: List[str]) -> bool:
        """
        Add a translation to a song.

        Args:
            song_id: ID of the song
            language: Language code for translation
            translation: Translated lyrics lines

        Returns:
            True if successful, False if song not found
        """
        song = self.get_song(song_id)
        if not song:
            return False

        song.translations[language] = translation
        self.update_song(song)
        return True
