"""Practice engine for LyRemember."""

import random
import time
from typing import List, Tuple, Optional
from lyremember.models import Song, PracticeSession
from lyremember.utils import hide_words, check_answer, normalize_text


class PracticeEngine:
    """Handles practice sessions and different practice modes."""

    def __init__(self, song: Song, mode: str = 'fill-blank', difficulty: float = 0.3):
        """
        Initialize practice engine.

        Args:
            song: Song to practice
            mode: Practice mode ('fill-blank', 'flashcard', 'line-by-line')
            difficulty: Difficulty level (0-1, higher = harder)
        """
        self.song = song
        self.mode = mode
        self.difficulty = difficulty
        self.start_time = time.time()

        self.lines_practiced = 0
        self.lines_correct = 0
        self.difficult_lines = []

    def fill_in_blank_practice(self) -> List[Tuple[str, List[str], str]]:
        """
        Generate fill-in-the-blank practice questions.

        Returns:
            List of tuples (modified_line, hidden_words, original_line)
        """
        practice_items = []

        for line in self.song.lyrics:
            if line.strip():  # Skip empty lines
                modified_line, hidden_words = hide_words(line, self.difficulty)
                practice_items.append((modified_line, hidden_words, line))

        return practice_items

    def check_fill_blank_answer(self, user_words: List[str], correct_words: List[str]) -> Tuple[bool, int]:
        """
        Check fill-in-blank answers.

        Args:
            user_words: Words provided by user
            correct_words: Correct words

        Returns:
            Tuple of (all_correct, num_correct)
        """
        if len(user_words) != len(correct_words):
            return False, 0

        num_correct = 0
        for user_word, correct_word in zip(user_words, correct_words):
            if check_answer(user_word, correct_word):
                num_correct += 1

        return num_correct == len(correct_words), num_correct

    def flashcard_practice(self) -> List[Tuple[str, str]]:
        """
        Generate flashcard practice items.
        Shows first half of line, user recalls second half.

        Returns:
            List of tuples (prompt, answer)
        """
        flashcards = []

        for line in self.song.lyrics:
            if line.strip():
                words = line.split()
                if len(words) > 2:
                    # Split at midpoint
                    midpoint = len(words) // 2
                    prompt = ' '.join(words[:midpoint]) + '...'
                    answer = ' '.join(words[midpoint:])
                    flashcards.append((prompt, answer))

        return flashcards

    def line_by_line_practice(self, shuffle: bool = False) -> List[str]:
        """
        Get lines for line-by-line practice.

        Args:
            shuffle: Whether to shuffle the lines

        Returns:
            List of lines in practice order
        """
        lines = [line for line in self.song.lyrics if line.strip()]

        if shuffle:
            random.shuffle(lines)

        return lines

    def record_line_result(self, line_index: int, correct: bool) -> None:
        """Record the result for a line."""
        self.lines_practiced += 1
        if correct:
            self.lines_correct += 1
        else:
            if line_index not in self.difficult_lines:
                self.difficult_lines.append(line_index)

    def create_session(self) -> PracticeSession:
        """
        Create a practice session record.

        Returns:
            PracticeSession object
        """
        duration = int(time.time() - self.start_time)
        score = (self.lines_correct / self.lines_practiced * 100) if self.lines_practiced > 0 else 0

        return PracticeSession(
            song_id=self.song.id,
            mode=self.mode,
            score=score,
            lines_practiced=self.lines_practiced,
            lines_correct=self.lines_correct,
            difficult_lines=self.difficult_lines,
            duration_seconds=duration
        )
