"""Tests for the practice engine."""

import pytest
from lyremember.models import Song
from lyremember.practice_engine import PracticeEngine


@pytest.fixture
def song():
    return Song(
        title="Test",
        artist="Artist",
        language="en",
        lyrics=[
            "Twinkle twinkle little star",
            "How I wonder what you are",
            "",  # empty line — should be skipped by practice modes
            "Up above the world so high",
        ],
    )


class TestFillInBlank:
    def test_skips_empty_lines(self, song):
        engine = PracticeEngine(song, mode="fill-blank", difficulty=0.3)
        items = engine.fill_in_blank_practice()
        assert len(items) == 3
        for _modified, _hidden, original in items:
            assert original.strip() != ""

    def test_difficulty_zero_still_hides_at_least_one_word(self, song):
        # `hide_words` enforces `max(1, ...)` so even difficulty=0 hides one
        # word per non-empty line — this guards that contract.
        engine = PracticeEngine(song, mode="fill-blank", difficulty=0.0)
        items = engine.fill_in_blank_practice()
        for _modified, hidden, _original in items:
            assert len(hidden) >= 1

    def test_check_answer_returns_all_correct_on_exact_match(self, song):
        engine = PracticeEngine(song)
        all_ok, count = engine.check_fill_blank_answer(
            ["twinkle", "star"], ["twinkle", "star"]
        )
        assert all_ok is True
        assert count == 2

    def test_check_answer_mismatched_lengths(self, song):
        engine = PracticeEngine(song)
        all_ok, count = engine.check_fill_blank_answer(["a"], ["a", "b"])
        assert all_ok is False
        assert count == 0


class TestFlashcardMode:
    def test_flashcard_returns_prompts_for_non_empty_lines(self, song):
        engine = PracticeEngine(song, mode="flashcard")
        items = engine.flashcard_practice()
        assert len(items) == 3
        for prompt, answer in items:
            assert isinstance(prompt, str)
            assert isinstance(answer, str)


class TestLineByLine:
    def test_returns_non_empty_lines_only(self, song):
        engine = PracticeEngine(song, mode="line-by-line")
        lines = engine.line_by_line_practice()
        assert len(lines) == 3
        assert "" not in lines

    def test_shuffle_preserves_set_of_lines(self, song):
        engine = PracticeEngine(song, mode="line-by-line")
        sorted_original = sorted(engine.line_by_line_practice())
        sorted_shuffled = sorted(engine.line_by_line_practice(shuffle=True))
        assert sorted_original == sorted_shuffled


class TestSessionAccounting:
    def test_record_correct_increments_counters(self, song):
        engine = PracticeEngine(song)
        engine.record_line_result(0, correct=True)
        engine.record_line_result(1, correct=False)
        engine.record_line_result(2, correct=True)
        assert engine.lines_practiced == 3
        assert engine.lines_correct == 2
        assert 1 in engine.difficult_lines

    def test_create_session_uses_recorded_counters(self, song):
        engine = PracticeEngine(song, mode="fill-blank")
        engine.record_line_result(0, correct=True)
        engine.record_line_result(1, correct=False)
        session = engine.create_session()
        assert session.song_id == song.id
        assert session.mode == "fill-blank"
        assert session.lines_practiced == 2
        assert session.lines_correct == 1
        assert session.score == 50.0
