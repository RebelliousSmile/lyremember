"""Data models for LyRemember."""

from dataclasses import dataclass, field
from typing import List, Dict, Optional
from datetime import datetime
import uuid


@dataclass
class Song:
    """Represents a song with lyrics."""
    
    title: str
    artist: str
    language: str
    lyrics: List[str]
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    translations: Dict[str, List[str]] = field(default_factory=dict)
    created_at: str = field(default_factory=lambda: datetime.utcnow().isoformat())
    updated_at: str = field(default_factory=lambda: datetime.utcnow().isoformat())
    metadata: Dict = field(default_factory=dict)
    
    def to_dict(self) -> dict:
        """Convert song to dictionary."""
        return {
            'id': self.id,
            'title': self.title,
            'artist': self.artist,
            'language': self.language,
            'lyrics': self.lyrics,
            'translations': self.translations,
            'created_at': self.created_at,
            'updated_at': self.updated_at,
            'metadata': self.metadata
        }
    
    @classmethod
    def from_dict(cls, data: dict) -> 'Song':
        """Create song from dictionary."""
        return cls(
            id=data.get('id', str(uuid.uuid4())),
            title=data['title'],
            artist=data['artist'],
            language=data['language'],
            lyrics=data['lyrics'],
            translations=data.get('translations', {}),
            created_at=data.get('created_at', datetime.utcnow().isoformat()),
            updated_at=data.get('updated_at', datetime.utcnow().isoformat()),
            metadata=data.get('metadata', {})
        )


@dataclass
class PracticeSession:
    """Represents a practice session."""
    
    song_id: str
    mode: str
    score: float
    lines_practiced: int
    lines_correct: int
    difficult_lines: List[int] = field(default_factory=list)
    session_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    date: str = field(default_factory=lambda: datetime.utcnow().isoformat())
    duration_seconds: int = 0
    
    def to_dict(self) -> dict:
        """Convert session to dictionary."""
        return {
            'session_id': self.session_id,
            'song_id': self.song_id,
            'date': self.date,
            'mode': self.mode,
            'duration_seconds': self.duration_seconds,
            'score': self.score,
            'lines_practiced': self.lines_practiced,
            'lines_correct': self.lines_correct,
            'difficult_lines': self.difficult_lines
        }
    
    @classmethod
    def from_dict(cls, data: dict) -> 'PracticeSession':
        """Create session from dictionary."""
        return cls(
            session_id=data.get('session_id', str(uuid.uuid4())),
            song_id=data['song_id'],
            date=data.get('date', datetime.utcnow().isoformat()),
            mode=data['mode'],
            duration_seconds=data.get('duration_seconds', 0),
            score=data['score'],
            lines_practiced=data['lines_practiced'],
            lines_correct=data['lines_correct'],
            difficult_lines=data.get('difficult_lines', [])
        )


@dataclass
class SongProgress:
    """Tracks progress for a specific song."""
    
    song_id: str
    practice_sessions: List[PracticeSession] = field(default_factory=list)
    mastery_level: float = 0.0
    total_practice_time: int = 0
    last_practiced: Optional[str] = None
    
    def to_dict(self) -> dict:
        """Convert progress to dictionary."""
        return {
            'song_id': self.song_id,
            'practice_sessions': [s.to_dict() for s in self.practice_sessions],
            'mastery_level': self.mastery_level,
            'total_practice_time': self.total_practice_time,
            'last_practiced': self.last_practiced
        }
    
    @classmethod
    def from_dict(cls, data: dict) -> 'SongProgress':
        """Create progress from dictionary."""
        return cls(
            song_id=data['song_id'],
            practice_sessions=[PracticeSession.from_dict(s) for s in data.get('practice_sessions', [])],
            mastery_level=data.get('mastery_level', 0.0),
            total_practice_time=data.get('total_practice_time', 0),
            last_practiced=data.get('last_practiced')
        )
