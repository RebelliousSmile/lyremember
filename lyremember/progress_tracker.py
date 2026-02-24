"""Progress tracking for LyRemember."""

from typing import List, Dict, Optional
from datetime import datetime
from lyremember.models import SongProgress, PracticeSession
from lyremember.storage import Storage
from lyremember.utils import format_time


class ProgressTracker:
    """Tracks and manages user progress."""

    def __init__(self, storage: Storage):
        """Initialize progress tracker with storage."""
        self.storage = storage

    def record_session(self, session: PracticeSession) -> None:
        """
        Record a practice session.

        Args:
            session: PracticeSession to record
        """
        # Get existing progress or create new
        progress = self.storage.get_song_progress(session.song_id)

        if progress is None:
            progress = SongProgress(song_id=session.song_id)

        # Add session
        progress.practice_sessions.append(session)

        # Update totals
        progress.total_practice_time += session.duration_seconds
        progress.last_practiced = session.date

        # Calculate mastery level (simple average of recent scores)
        recent_sessions = progress.practice_sessions[-10:]  # Last 10 sessions
        if recent_sessions:
            avg_score = sum(s.score for s in recent_sessions) / len(recent_sessions)
            progress.mastery_level = avg_score / 100.0

        # Save
        self.storage.save_progress(progress)

    def get_song_progress(self, song_id: str) -> Optional[SongProgress]:
        """Get progress for a specific song."""
        return self.storage.get_song_progress(song_id)

    def get_all_progress(self) -> Dict[str, SongProgress]:
        """Get all progress data."""
        return self.storage.get_all_progress()

    def get_statistics(self) -> dict:
        """
        Get overall statistics.

        Returns:
            Dictionary with statistics
        """
        all_progress = self.get_all_progress()

        total_songs = len(all_progress)
        total_sessions = sum(len(p.practice_sessions) for p in all_progress.values())
        total_time = sum(p.total_practice_time for p in all_progress.values())

        # Calculate average mastery
        if total_songs > 0:
            avg_mastery = sum(p.mastery_level for p in all_progress.values()) / total_songs
        else:
            avg_mastery = 0

        # Get all sessions for overall accuracy
        all_sessions = []
        for progress in all_progress.values():
            all_sessions.extend(progress.practice_sessions)

        if all_sessions:
            total_lines = sum(s.lines_practiced for s in all_sessions)
            correct_lines = sum(s.lines_correct for s in all_sessions)
            avg_accuracy = (correct_lines / total_lines * 100) if total_lines > 0 else 0
        else:
            avg_accuracy = 0

        return {
            'total_songs_practiced': total_songs,
            'total_sessions': total_sessions,
            'total_practice_time': total_time,
            'total_practice_time_formatted': format_time(total_time),
            'average_mastery': avg_mastery,
            'average_accuracy': avg_accuracy
        }

    def get_songs_by_mastery(self) -> List[tuple]:
        """
        Get songs sorted by mastery level.

        Returns:
            List of tuples (song_id, mastery_level)
        """
        all_progress = self.get_all_progress()
        songs_mastery = [(sid, p.mastery_level) for sid, p in all_progress.items()]
        songs_mastery.sort(key=lambda x: x[1], reverse=True)
        return songs_mastery

    def get_recommended_practice_song(self) -> Optional[str]:
        """
        Get recommended song ID for practice based on:
        - Lowest mastery level
        - Least recently practiced

        Returns:
            Song ID or None
        """
        all_progress = self.get_all_progress()
        if not all_progress:
            return None

        # Sort by mastery (ascending) and last practiced (oldest first)
        songs = []
        for song_id, progress in all_progress.items():
            last_practiced = progress.last_practiced or '1970-01-01'
            songs.append((song_id, progress.mastery_level, last_practiced))

        # Sort by mastery (lower first), then by date (older first)
        songs.sort(key=lambda x: (x[1], x[2]))

        return songs[0][0] if songs else None
