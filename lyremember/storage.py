"""Storage layer for LyRemember."""

import json
import os
from pathlib import Path
from typing import List, Optional, Dict
from lyremember.models import Song, SongProgress


class Storage:
    """Handles data persistence using JSON files."""
    
    def __init__(self, data_dir: str = None):
        """Initialize storage with data directory."""
        if data_dir is None:
            # Use data directory in project root
            self.data_dir = Path(__file__).parent.parent / 'data'
        else:
            self.data_dir = Path(data_dir)
        
        self.data_dir.mkdir(parents=True, exist_ok=True)
        
        self.songs_file = self.data_dir / 'songs.json'
        self.progress_file = self.data_dir / 'progress.json'
        self.config_file = self.data_dir / 'config.json'
        
        # Initialize files if they don't exist
        self._init_file(self.songs_file, [])
        self._init_file(self.progress_file, {})
        self._init_file(self.config_file, self._default_config())
    
    def _init_file(self, filepath: Path, default_data):
        """Initialize a JSON file with default data if it doesn't exist."""
        if not filepath.exists():
            with open(filepath, 'w', encoding='utf-8') as f:
                json.dump(default_data, f, indent=2, ensure_ascii=False)
    
    def _default_config(self) -> dict:
        """Get default configuration."""
        return {
            'user_name': 'User',
            'preferred_language': 'en',
            'difficulty_settings': {
                'fill_in_blank_percentage': 0.3,
                'practice_time_minutes': 15
            },
            'ui_preferences': {
                'color_enabled': True,
                'show_hints': True
            }
        }
    
    # Song operations
    def get_all_songs(self) -> List[Song]:
        """Get all songs."""
        with open(self.songs_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        return [Song.from_dict(song_data) for song_data in data]
    
    def get_song(self, song_id: str) -> Optional[Song]:
        """Get a specific song by ID."""
        songs = self.get_all_songs()
        for song in songs:
            if song.id == song_id:
                return song
        return None
    
    def save_song(self, song: Song) -> None:
        """Save a song (add or update)."""
        songs = self.get_all_songs()
        
        # Update existing or add new
        found = False
        for i, s in enumerate(songs):
            if s.id == song.id:
                songs[i] = song
                found = True
                break
        
        if not found:
            songs.append(song)
        
        # Write to file
        with open(self.songs_file, 'w', encoding='utf-8') as f:
            json.dump([s.to_dict() for s in songs], f, indent=2, ensure_ascii=False)
    
    def delete_song(self, song_id: str) -> bool:
        """Delete a song by ID."""
        songs = self.get_all_songs()
        original_len = len(songs)
        songs = [s for s in songs if s.id != song_id]
        
        if len(songs) < original_len:
            with open(self.songs_file, 'w', encoding='utf-8') as f:
                json.dump([s.to_dict() for s in songs], f, indent=2, ensure_ascii=False)
            return True
        return False
    
    # Progress operations
    def get_all_progress(self) -> Dict[str, SongProgress]:
        """Get all progress data."""
        with open(self.progress_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        return {
            song_id: SongProgress.from_dict(prog_data)
            for song_id, prog_data in data.items()
        }
    
    def get_song_progress(self, song_id: str) -> Optional[SongProgress]:
        """Get progress for a specific song."""
        all_progress = self.get_all_progress()
        return all_progress.get(song_id)
    
    def save_progress(self, progress: SongProgress) -> None:
        """Save progress for a song."""
        all_progress = self.get_all_progress()
        all_progress[progress.song_id] = progress
        
        with open(self.progress_file, 'w', encoding='utf-8') as f:
            json.dump(
                {k: v.to_dict() for k, v in all_progress.items()},
                f,
                indent=2,
                ensure_ascii=False
            )
    
    # Config operations
    def get_config(self) -> dict:
        """Get configuration."""
        with open(self.config_file, 'r', encoding='utf-8') as f:
            return json.load(f)
    
    def save_config(self, config: dict) -> None:
        """Save configuration."""
        with open(self.config_file, 'w', encoding='utf-8') as f:
            json.dump(config, f, indent=2, ensure_ascii=False)
