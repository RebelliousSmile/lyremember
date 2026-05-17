"""Data models for LyRemember."""

from dataclasses import dataclass, field
from typing import List, Dict, Optional
from datetime import datetime, timezone
import uuid
import hashlib


@dataclass
class User:
    """Represents a user account."""
    
    username: str
    email: str
    password_hash: str
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    genius_token: Optional[str] = None
    song_repertoire: List[str] = field(default_factory=list)  # List of song IDs
    created_at: str = field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
    last_login: Optional[str] = None
    
    @staticmethod
    def hash_password(password: str) -> str:
        """Hash a password using SHA-256."""
        return hashlib.sha256(password.encode()).hexdigest()
    
    def verify_password(self, password: str) -> bool:
        """Verify a password against the hash."""
        return self.password_hash == self.hash_password(password)
    
    def to_dict(self) -> dict:
        """Convert user to dictionary."""
        return {
            'id': self.id,
            'username': self.username,
            'email': self.email,
            'password_hash': self.password_hash,
            'genius_token': self.genius_token,
            'song_repertoire': self.song_repertoire,
            'created_at': self.created_at,
            'last_login': self.last_login
        }
    
    @classmethod
    def from_dict(cls, data: dict) -> 'User':
        """Create user from dictionary."""
        return cls(
            id=data.get('id', str(uuid.uuid4())),
            username=data['username'],
            email=data['email'],
            password_hash=data['password_hash'],
            genius_token=data.get('genius_token'),
            song_repertoire=data.get('song_repertoire', []),
            created_at=data.get('created_at', datetime.now(timezone.utc).isoformat()),
            last_login=data.get('last_login')
        )


@dataclass
class Song:
    """Represents a song with lyrics."""
    
    title: str
    artist: str
    language: str
    lyrics: List[str]
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    translations: Dict[str, List[str]] = field(default_factory=dict)
    phonetic_lyrics: Optional[List[str]] = None
    genius_id: Optional[str] = None
    genius_url: Optional[str] = None
    created_at: str = field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
    updated_at: str = field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
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
            'phonetic_lyrics': self.phonetic_lyrics,
            'genius_id': self.genius_id,
            'genius_url': self.genius_url,
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
            phonetic_lyrics=data.get('phonetic_lyrics'),
            genius_id=data.get('genius_id'),
            genius_url=data.get('genius_url'),
            created_at=data.get('created_at', datetime.now(timezone.utc).isoformat()),
            updated_at=data.get('updated_at', datetime.now(timezone.utc).isoformat()),
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
    date: str = field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
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
            date=data.get('date', datetime.now(timezone.utc).isoformat()),
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
