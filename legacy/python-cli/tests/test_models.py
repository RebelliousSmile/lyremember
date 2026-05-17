"""Tests for the dataclass models."""

from lyremember.models import User, Song, PracticeSession, SongProgress


class TestUser:
    def test_hash_password_is_deterministic(self):
        assert User.hash_password("hunter2") == User.hash_password("hunter2")

    def test_hash_password_differs_for_different_inputs(self):
        assert User.hash_password("a") != User.hash_password("b")

    def test_verify_password_success(self):
        user = User("alice", "a@example.com", User.hash_password("pw"))
        assert user.verify_password("pw") is True

    def test_verify_password_failure(self):
        user = User("alice", "a@example.com", User.hash_password("pw"))
        assert user.verify_password("wrong") is False

    def test_round_trip_to_dict_from_dict(self):
        original = User(
            "alice",
            "a@example.com",
            User.hash_password("pw"),
            genius_token="token-abc",
            song_repertoire=["s1", "s2"],
        )
        restored = User.from_dict(original.to_dict())
        assert restored.username == original.username
        assert restored.email == original.email
        assert restored.password_hash == original.password_hash
        assert restored.genius_token == original.genius_token
        assert restored.song_repertoire == original.song_repertoire
        assert restored.id == original.id


class TestSong:
    def test_song_defaults(self):
        song = Song("T", "A", "en", ["line"])
        assert song.translations == {}
        assert song.phonetic_lyrics is None
        assert song.genius_id is None
        assert song.genius_url is None

    def test_song_round_trip(self):
        original = Song(
            "Title",
            "Artist",
            "fr",
            ["bonjour", "monde"],
            translations={"en": ["hello", "world"]},
            phonetic_lyrics=["bɔ̃ʒuʁ", "mɔ̃d"],
            genius_url="https://genius.com/x",
        )
        restored = Song.from_dict(original.to_dict())
        assert restored.title == original.title
        assert restored.artist == original.artist
        assert restored.language == original.language
        assert restored.lyrics == original.lyrics
        assert restored.translations == original.translations
        assert restored.phonetic_lyrics == original.phonetic_lyrics
        assert restored.genius_url == original.genius_url


class TestPracticeSession:
    def test_practice_session_required_fields(self):
        s = PracticeSession(song_id="s1", mode="fill-blank", score=0.8, lines_practiced=10, lines_correct=8)
        assert s.song_id == "s1"
        assert s.lines_correct == 8
        assert s.difficult_lines == []
        assert s.duration_seconds == 0

    def test_practice_session_round_trip(self):
        original = PracticeSession(
            song_id="s1",
            mode="oral",
            score=0.5,
            lines_practiced=4,
            lines_correct=2,
            difficult_lines=[1, 3],
            duration_seconds=120,
        )
        restored = PracticeSession.from_dict(original.to_dict())
        assert restored.song_id == "s1"
        assert restored.mode == "oral"
        assert restored.difficult_lines == [1, 3]


class TestSongProgress:
    def test_progress_defaults(self):
        progress = SongProgress(song_id="s1")
        assert progress.practice_sessions == []
        assert progress.mastery_level == 0.0
        assert progress.total_practice_time == 0
        assert progress.last_practiced is None

    def test_progress_round_trip_with_sessions(self):
        original = SongProgress(
            song_id="s1",
            practice_sessions=[
                PracticeSession(song_id="s1", mode="fill-blank", score=0.7, lines_practiced=10, lines_correct=7),
            ],
            mastery_level=0.4,
            total_practice_time=300,
            last_practiced="2025-01-01T00:00:00",
        )
        restored = SongProgress.from_dict(original.to_dict())
        assert restored.mastery_level == 0.4
        assert restored.total_practice_time == 300
        assert len(restored.practice_sessions) == 1
        assert restored.practice_sessions[0].score == 0.7
