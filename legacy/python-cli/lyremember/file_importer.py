"""File importer for LyRemember.

Supports importing lyrics from plain text (.txt) and JSON (.json) files.

Text file format (.txt):
    Each non-empty line is treated as one lyric line.
    Lines starting with '#' are treated as comments and ignored.

JSON file format (.json):
    {
        "title": "Song Title",
        "artist": "Artist Name",
        "language": "en",
        "lyrics": ["line 1", "line 2", ...]
    }
    All fields are optional in the JSON file; missing ones can be supplied
    via CLI options or interactive prompts.
"""

import json
import os
from pathlib import Path
from typing import Dict, List, Optional, Tuple


class FileImportError(Exception):
    """Raised when a file cannot be imported."""


def _read_txt(filepath: Path) -> List[str]:
    """Read lyrics from a plain-text file (one lyric line per file line)."""
    lyrics = []
    with open(filepath, 'r', encoding='utf-8') as f:
        for line in f:
            stripped = line.rstrip('\n')
            # Skip comment lines
            if stripped.startswith('#'):
                continue
            if stripped.strip():
                lyrics.append(stripped)
    return lyrics


def _read_json(filepath: Path) -> Dict:
    """Read song data from a JSON file."""
    with open(filepath, 'r', encoding='utf-8') as f:
        try:
            data = json.load(f)
        except json.JSONDecodeError as exc:
            raise FileImportError(f"Invalid JSON in '{filepath}': {exc}") from exc

    if not isinstance(data, dict):
        raise FileImportError(
            f"JSON file '{filepath}' must contain a single object, not a list."
        )

    lyrics = data.get('lyrics')
    if lyrics is not None and not isinstance(lyrics, list):
        raise FileImportError(
            f"'lyrics' field in '{filepath}' must be a list of strings."
        )

    return data


def import_file(
    filepath: str,
    title: Optional[str] = None,
    artist: Optional[str] = None,
    language: Optional[str] = None,
) -> Tuple[Optional[str], Optional[str], Optional[str], List[str]]:
    """
    Import a lyrics file and return the song data.

    Supported formats:
        - ``.txt``: plain text, one lyric line per line
        - ``.json``: JSON object with optional title/artist/language/lyrics fields

    Args:
        filepath: Path to the lyrics file.
        title: Song title (overrides any value found in the file).
        artist: Artist name (overrides any value found in the file).
        language: Language code such as ``'en'`` (overrides any value in the file).

    Returns:
        A tuple ``(title, artist, language, lyrics)``.

    Raises:
        FileImportError: If the file cannot be read or has an unsupported format.
        FileNotFoundError: If the file does not exist.
    """
    path = Path(filepath)

    if not path.exists():
        raise FileNotFoundError(f"File not found: '{filepath}'")

    suffix = path.suffix.lower()

    if suffix == '.txt':
        lyrics = _read_txt(path)
        file_title = None
        file_artist = None
        file_language = None
    elif suffix == '.json':
        data = _read_json(path)
        lyrics = data.get('lyrics', [])
        file_title = data.get('title')
        file_artist = data.get('artist')
        file_language = data.get('language')
    else:
        raise FileImportError(
            f"Unsupported file format '{suffix}'. Supported formats: .txt, .json"
        )

    if not lyrics:
        raise FileImportError(f"No lyrics found in '{filepath}'.")

    # CLI options take precedence over file contents
    resolved_title = title or file_title
    resolved_artist = artist or file_artist
    resolved_language = language or file_language

    return resolved_title, resolved_artist, resolved_language, lyrics
