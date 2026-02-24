"""Genius API integration for LyRemember."""

import os
from typing import Optional, Dict, List
from lyremember.models import Song


class GeniusAPI:
    """Handles integration with Genius lyrics service."""
    
    def __init__(self, access_token: Optional[str] = None):
        """
        Initialize Genius API client.
        
        Args:
            access_token: Genius API access token
        """
        self.access_token = access_token or os.environ.get('GENIUS_ACCESS_TOKEN')
        self.genius = None
        
        if self.access_token:
            try:
                import lyricsgenius
                self.genius = lyricsgenius.Genius(self.access_token)
                self.genius.verbose = False
                self.genius.remove_section_headers = True
            except ImportError:
                print("Warning: lyricsgenius not installed. Install with: pip install lyricsgenius")
    
    def search_song(self, query: str, max_results: int = 5) -> List[Dict]:
        """
        Search for songs on Genius.
        
        Args:
            query: Search query (song title, artist, etc.)
            max_results: Maximum number of results to return
        
        Returns:
            List of song dictionaries with metadata
        """
        if not self.genius:
            return []
        
        try:
            response = self.genius.search_songs(query)
            
            results = []
            for hit in response.get('hits', [])[:max_results]:
                song_data = hit.get('result', {})
                results.append({
                    'id': song_data.get('id'),
                    'title': song_data.get('title'),
                    'artist': song_data.get('primary_artist', {}).get('name'),
                    'url': song_data.get('url'),
                    'thumbnail': song_data.get('song_art_image_thumbnail_url')
                })
            
            return results
        except Exception as e:
            print(f"Error searching Genius: {e}")
            return []
    
    def get_lyrics(self, song_title: str, artist: str) -> Optional[str]:
        """
        Get lyrics for a song from Genius.
        
        Args:
            song_title: Title of the song
            artist: Artist name
        
        Returns:
            Lyrics as a string, or None if not found
        """
        if not self.genius:
            return None
        
        try:
            song = self.genius.search_song(song_title, artist)
            if song:
                return song.lyrics
            return None
        except Exception as e:
            print(f"Error fetching lyrics from Genius: {e}")
            return None
    
    def create_song_from_genius(self, song_id: int, language: str = 'en') -> Optional[Song]:
        """
        Create a Song object from Genius data.
        
        Args:
            song_id: Genius song ID
            language: Language code for the song
        
        Returns:
            Song object or None if not found
        """
        if not self.genius:
            return None
        
        try:
            song_data = self.genius.search_song(song_id=song_id)
            if not song_data:
                return None
            
            # Split lyrics into lines
            lyrics_text = song_data.lyrics
            lyrics_lines = [line.strip() for line in lyrics_text.split('\n') if line.strip()]
            
            # Create Song object
            song = Song(
                title=song_data.title,
                artist=song_data.artist,
                language=language,
                lyrics=lyrics_lines,
                genius_id=str(song_id),
                genius_url=song_data.url
            )
            
            return song
        except Exception as e:
            print(f"Error creating song from Genius: {e}")
            return None
    
    def import_song(self, song_title: str, artist: str, language: str = 'en') -> Optional[Song]:
        """
        Import a song from Genius by title and artist.
        
        Args:
            song_title: Title of the song
            artist: Artist name
            language: Language code for the song
        
        Returns:
            Song object or None if not found
        """
        lyrics = self.get_lyrics(song_title, artist)
        
        if not lyrics:
            return None
        
        # Split lyrics into lines
        lyrics_lines = [line.strip() for line in lyrics.split('\n') if line.strip()]
        
        # Create Song object
        song = Song(
            title=song_title,
            artist=artist,
            language=language,
            lyrics=lyrics_lines
        )
        
        return song
