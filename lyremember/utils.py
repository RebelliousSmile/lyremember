"""Utility functions for LyRemember."""

import re
from typing import List, Tuple
import Levenshtein


def normalize_text(text: str) -> str:
    """Normalize text for comparison."""
    # Convert to lowercase and strip
    text = text.lower().strip()
    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text)
    # Remove punctuation for comparison
    text = re.sub(r'[^\w\s]', '', text)
    return text


def check_answer(user_answer: str, correct_answer: str, fuzzy_threshold: float = 0.85) -> bool:
    """
    Check if user answer matches correct answer.
    Uses fuzzy matching to allow minor typos.

    Args:
        user_answer: The answer provided by user
        correct_answer: The correct answer
        fuzzy_threshold: Similarity threshold (0-1) for accepting answer

    Returns:
        True if answer is correct (or close enough)
    """
    user_norm = normalize_text(user_answer)
    correct_norm = normalize_text(correct_answer)

    # Exact match
    if user_norm == correct_norm:
        return True

    # Fuzzy match using Levenshtein ratio
    similarity = Levenshtein.ratio(user_norm, correct_norm)
    return similarity >= fuzzy_threshold


def calculate_similarity(text1: str, text2: str) -> float:
    """Calculate similarity between two texts (0-1)."""
    norm1 = normalize_text(text1)
    norm2 = normalize_text(text2)
    return Levenshtein.ratio(norm1, norm2)


def split_words(line: str) -> List[str]:
    """Split a line into words, preserving punctuation context."""
    # Split on whitespace but keep track of original positions
    return line.split()


def hide_words(line: str, hide_percentage: float = 0.3) -> Tuple[str, List[str]]:
    """
    Hide random words in a line.

    Args:
        line: The line to process
        hide_percentage: Percentage of words to hide (0-1)

    Returns:
        Tuple of (modified_line, hidden_words)
    """
    import random

    words = split_words(line)
    if not words:
        return line, []

    # Determine how many words to hide
    num_to_hide = max(1, int(len(words) * hide_percentage))

    # Select random word indices to hide
    indices_to_hide = random.sample(range(len(words)), min(num_to_hide, len(words)))
    indices_to_hide.sort()

    hidden_words = []
    modified_words = []

    for i, word in enumerate(words):
        if i in indices_to_hide:
            hidden_words.append(word)
            # Replace with underscores based on word length
            modified_words.append('_' * max(3, len(re.sub(r'[^\w]', '', word))))
        else:
            modified_words.append(word)

    return ' '.join(modified_words), hidden_words


def format_time(seconds: int) -> str:
    """Format seconds into human-readable time string."""
    if seconds < 60:
        return f"{seconds}s"
    elif seconds < 3600:
        minutes = seconds // 60
        secs = seconds % 60
        return f"{minutes}m {secs}s"
    else:
        hours = seconds // 3600
        minutes = (seconds % 3600) // 60
        return f"{hours}h {minutes}m"


def truncate_text(text: str, max_length: int = 50) -> str:
    """Truncate text to max length with ellipsis."""
    if len(text) <= max_length:
        return text
    return text[:max_length - 3] + "..."
