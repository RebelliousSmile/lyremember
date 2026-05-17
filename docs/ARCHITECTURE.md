# LyRemember Architecture

## Overview
LyRemember is a lyrics memorization application supporting multiple languages. This document outlines the technical architecture and design decisions.

> **Statut (mai 2026)** : ce document décrit l'architecture du **proof of
> concept Python** (CLI Click + storage JSON), désormais archivé dans
> [`legacy/python-cli/`](../legacy/python-cli/). La stack canonique du
> projet est Rust + Tauri + Vue 3 :
> - Application : `lyremember-app/` (Vue 3 + TypeScript + Tauri v2 + Pinia)
> - Backend : `rust-backend/` (Rust + SQLite + PyO3)
> - Storage prod : **SQLite** via `rust-backend/src/db/sqlite.rs`
>   (4 tables : `users`, `songs`, `user_songs`, `practice_sessions`).
> - Storage POC legacy : **JSON** via `legacy/python-cli/lyremember/storage.py`.
>
> **Build flags Tauri** : la feature `python` (PyO3 bridge phonétique
> JP/KR/FR/EN via pykakasi / hangul-romanize / epitran) est activée
> **par défaut sur desktop**. Les builds Android passent
> `--no-default-features` (voir `.github/workflows/release.yml`) car
> PyO3 ne se cross-compile pas vers Android — la phonétique renvoie
> alors un stub d'erreur sur APK jusqu'à ce qu'une alternative Rust
> pure soit fournie.
>
> Pour les décisions techniques actuelles : voir
> [`FINAL_DECISIONS.md`](FINAL_DECISIONS.md).

## Technology Stack (POC Python — legacy)

### Core Application
- **Language**: Python 3.8+
- **Interface**: Command-Line Interface (CLI)
- **Data Storage**: JSON files (`legacy/python-cli/data/*.json`).
  La version prod (Tauri/Rust) utilise SQLite — voir note ci-dessus.

### Key Libraries
- `click` - CLI framework for user-friendly command-line interface
- `colorama` - Terminal color support for better UX
- `tabulate` - Pretty table formatting for song lists
- `fuzzywuzzy` - Fuzzy string matching for answer checking
- `python-Levenshtein` - Edit distance for flexible answer validation
- `pyyaml` - Configuration management

## Architecture Diagram

```
┌─────────────────────────────────────────────────┐
│              User Interface (CLI)                │
│  ┌──────────┬──────────┬──────────┬──────────┐ │
│  │  Add     │ Practice │ Progress │  List    │ │
│  │  Songs   │  Modes   │ Tracking │  Songs   │ │
│  └──────────┴──────────┴──────────┴──────────┘ │
└─────────────────────┬───────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│            Application Logic Layer               │
│  ┌──────────────────────────────────────────┐  │
│  │  Song Manager  │  Practice Engine        │  │
│  │  - Add/Edit    │  - Fill-in-blank        │  │
│  │  - List/Search │  - Flashcards           │  │
│  │  - Delete      │  - Line-by-line         │  │
│  └──────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────┐  │
│  │  Progress Tracker                        │  │
│  │  - Statistics  │  - History              │  │
│  │  - Scores      │  - Achievements         │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────┬───────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────┐
│              Data Access Layer                   │
│  ┌──────────────────────────────────────────┐  │
│  │  JSON Storage Handler                    │  │
│  │  - songs.json     - progress.json        │  │
│  │  - config.json    - vocabulary.json      │  │
│  └──────────────────────────────────────────┘  │
└──────────────────────────────────────────────────┘
```

## Data Models

### Song
```python
{
    "id": "unique-song-id",
    "title": "Song Title",
    "artist": "Artist Name",
    "language": "en",  # ISO 639-1 code
    "lyrics": [
        "First line of the song",
        "Second line of the song",
        ...
    ],
    "translations": {
        "es": ["Primera línea", "Segunda línea", ...],
        "fr": ["Première ligne", "Deuxième ligne", ...]
    },
    "created_at": "2026-02-17T10:00:00Z",
    "updated_at": "2026-02-17T10:00:00Z",
    "metadata": {
        "genre": "pop",
        "year": 2024,
        "difficulty": "medium"
    }
}
```

### Progress Entry
```python
{
    "song_id": "unique-song-id",
    "practice_sessions": [
        {
            "session_id": "session-id",
            "date": "2026-02-17T10:00:00Z",
            "mode": "fill-in-blank",
            "duration_seconds": 120,
            "score": 85,  # percentage
            "lines_practiced": 10,
            "lines_correct": 8,
            "difficult_lines": [2, 5, 7]  # line indices
        }
    ],
    "mastery_level": 0.75,  # 0-1 scale
    "total_practice_time": 3600,  # seconds
    "last_practiced": "2026-02-17T10:00:00Z"
}
```

### User Configuration
```python
{
    "user_name": "User",
    "preferred_language": "en",
    "difficulty_settings": {
        "fill_in_blank_percentage": 0.3,  # 30% words hidden
        "practice_time_minutes": 15
    },
    "ui_preferences": {
        "color_enabled": true,
        "show_hints": true
    }
}
```

## Core Components

### 1. CLI Interface (`cli.py`)
- Main entry point using `click`
- Command structure:
  ```
  lyremember
    ├── add           # Add new song
    ├── list          # List all songs
    ├── practice      # Start practice session
    │   ├── --mode    # fill-blank, flashcard, line-by-line
    │   └── --song-id # Specific song or random
    ├── progress      # View statistics
    ├── translate     # Add/view translations
    └── config        # Configure settings
  ```

### 2. Song Manager (`song_manager.py`)
- CRUD operations for songs
- Search and filter functionality
- Validation and sanitization
- Import/export capabilities

### 3. Practice Engine (`practice_engine.py`)
- Multiple practice modes:
  - **Fill-in-the-Blank**: Hide random words, user fills them
  - **Flashcard**: Show first part, recall second part
  - **Line-by-Line**: Progressive revelation
- Answer validation with fuzzy matching
- Hint system
- Difficulty adjustment

### 4. Progress Tracker (`progress_tracker.py`)
- Record practice sessions
- Calculate statistics
- Generate reports
- Track mastery levels using spaced repetition algorithm

### 5. Data Storage (`storage.py`)
- JSON file operations
- Data validation
- Backup and recovery
- Migration utilities

## Directory Structure (Python POC — legacy)

> Ce document décrit l'architecture du proof of concept Python, désormais
> archivé dans `legacy/python-cli/`. La stack canonique actuelle est
> Rust + Tauri + Vue 3 (voir `rust-backend/` et `lyremember-app/`).

```
legacy/python-cli/
├── requirements.txt
├── setup.py
├── lyremember/
│   ├── __init__.py
│   ├── cli.py              # CLI entry point
│   ├── song_manager.py     # Song CRUD operations
│   ├── practice_engine.py  # Practice modes
│   ├── progress_tracker.py # Statistics and tracking
│   ├── storage.py          # Data persistence
│   ├── utils.py            # Helper functions
│   └── models.py           # Data models
├── data/
│   ├── songs.json          # Songs database
│   ├── progress.json       # User progress
│   ├── config.json         # User configuration
│   └── samples/            # Sample songs
│       ├── sample_en.json
│       ├── sample_es.json
│       └── sample_fr.json
└── tests/
    ├── __init__.py
    ├── test_song_manager.py
    ├── test_practice_engine.py
    └── test_storage.py
    └── usage_guide.md
```

## Key Features for MVP

1. **Song Management**
   - Add songs with title, artist, language, and lyrics
   - List and search songs
   - View song details

2. **Fill-in-the-Blank Practice**
   - Randomly hide words based on difficulty setting
   - Accept user input with fuzzy matching
   - Provide immediate feedback

3. **Basic Progress Tracking**
   - Record practice sessions
   - Show basic statistics (accuracy, songs practiced)

4. **Multi-Language Support**
   - Support major languages (EN, ES, FR, DE, IT, PT, etc.)
   - Optional translation storage

5. **Data Persistence**
   - Save all data in JSON format
   - Auto-save after each session

## Future Enhancements

1. **Web Interface** - Flask/FastAPI web app
2. **Mobile Support** - React Native or Flutter
3. **Audio Integration** - Play song audio during practice
4. **Social Features** - Share songs, compete with friends
5. **Advanced Analytics** - ML-based difficulty prediction
6. **Spaced Repetition** - Smart scheduling based on forgetting curve
7. **Cloud Sync** - Multi-device synchronization
8. **Community Song Database** - Shared lyrics repository

## Design Principles

1. **Simplicity First**: MVP focuses on core features
2. **User-Friendly**: Intuitive CLI with helpful messages
3. **Extensible**: Modular design for easy feature addition
4. **Data Privacy**: All data stored locally by default
5. **Language Agnostic**: Unicode support for any language
6. **Offline-First**: No internet required for core functionality

## Development Phases

### Phase 1: Foundation (Current)
- Set up project structure
- Implement data models
- Create storage system
- Build basic CLI

### Phase 2: Core Features
- Implement song manager
- Create fill-in-blank practice mode
- Add progress tracking
- Sample data and documentation

### Phase 3: Enhanced Practice
- Add flashcard mode
- Implement line-by-line practice
- Translation support
- Advanced statistics

### Phase 4: Polish
- Comprehensive testing
- Performance optimization
- User documentation
- Sample songs in multiple languages
