# LyRemember 🎵

Application for memorizing and playing with lyrics in multiple languages.

## Overview

LyRemember is an interactive command-line application designed to help you memorize song lyrics in any language. Whether you're learning a new language, preparing for a performance, or just want to remember your favorite songs, LyRemember makes memorization fun and effective through various practice modes.

## Features

✨ **Multi-Language Support** - Add and practice songs in any language (English, Spanish, French, German, etc.)

🎯 **Multiple Practice Modes:**
- **Fill-in-the-Blank**: Random words are hidden; fill them in to test your memory
- **Flashcard**: See the beginning of a line and recall the rest
- **Line-by-Line**: Type each line from memory

📊 **Progress Tracking** - Track your learning with detailed statistics:
- Practice time and session count
- Accuracy and mastery levels
- Personalized recommendations

🌍 **Translation Support** - Add translations to help with language learning

💾 **Data Persistence** - All your songs and progress are saved locally

## Installation

### Prerequisites
- Python 3.8 or higher
- pip (Python package installer)

### Install from source

```bash
# Clone the repository
git clone https://github.com/RebelliousSmile/lyremember.git
cd lyremember

# Install dependencies
pip install -r requirements.txt

# Install the package
pip install -e .
```

## Quick Start

### 1. Add Your First Song

```bash
lyremember add
```

Follow the prompts to enter the song title, artist, language, and lyrics.

### 2. List Your Songs

```bash
lyremember list
```

### 3. Start Practicing

```bash
# Practice with fill-in-the-blank mode (default)
lyremember practice <song-id>

# Try different modes
lyremember practice <song-id> --mode flashcard
lyremember practice <song-id> --mode line-by-line

# Adjust difficulty (0.0 - 1.0, higher = more difficult)
lyremember practice <song-id> --difficulty 0.5
```

### 4. View Your Progress

```bash
# Overall statistics
lyremember progress

# Progress for a specific song
lyremember progress <song-id>
```

## Commands Reference

### Add a Song
```bash
lyremember add [OPTIONS]
```
Options:
- `--title TEXT`: Song title
- `--artist TEXT`: Artist name
- `--language TEXT`: Language code (e.g., en, es, fr)

### List Songs
```bash
lyremember list [OPTIONS]
```
Options:
- `--language TEXT`: Filter by language
- `--search TEXT`: Search by title or artist

### View Song Lyrics
```bash
lyremember view <song-id>
```

### Practice
```bash
lyremember practice [song-id] [OPTIONS]
```
Options:
- `--mode [fill-blank|flashcard|line-by-line]`: Practice mode (default: fill-blank)
- `--difficulty FLOAT`: Difficulty level 0-1 (default: 0.3)

If no song-id is provided, a recommended song will be selected based on your progress.

### View Progress
```bash
lyremember progress [song-id]
```

### Delete a Song
```bash
lyremember delete <song-id>
```

## Usage Examples

### Example 1: Learning Spanish Songs

```bash
# Add a Spanish song
lyremember add --title "La Cucaracha" --artist "Traditional" --language es

# List Spanish songs
lyremember list --language es

# Practice with easier difficulty
lyremember practice <song-id> --difficulty 0.2
```

### Example 2: Preparing for a Performance

```bash
# Add the song you need to perform
lyremember add

# Practice intensively with increasing difficulty
lyremember practice <song-id> --difficulty 0.3
lyremember practice <song-id> --difficulty 0.5
lyremember practice <song-id> --difficulty 0.7

# Check your mastery level
lyremember progress <song-id>
```

### Example 3: Daily Practice Routine

```bash
# Let the app recommend what to practice
lyremember practice

# View overall progress
lyremember progress
```

## Sample Songs

The repository includes sample songs in multiple languages in the `data/samples/` directory:
- `sample_en.json` - Twinkle Twinkle Little Star (English)
- `sample_es.json` - La Cucaracha (Spanish with English translation)
- `sample_fr.json` - Frère Jacques (French with English translation)

## Data Storage

All your data is stored locally in the `data/` directory:
- `songs.json` - Your song collection
- `progress.json` - Your practice history and statistics
- `config.json` - User preferences

## Project Structure

```
lyremember/
├── README.md                   # This file
├── USER_STORIES.md            # User stories and requirements
├── ARCHITECTURE.md            # Technical architecture
├── requirements.txt           # Python dependencies
├── setup.py                   # Package setup
├── lyremember/               # Main application package
│   ├── cli.py                # Command-line interface
│   ├── models.py             # Data models
│   ├── storage.py            # Data persistence
│   ├── song_manager.py       # Song CRUD operations
│   ├── practice_engine.py    # Practice modes
│   ├── progress_tracker.py   # Statistics tracking
│   └── utils.py              # Helper functions
└── data/                     # User data
    └── samples/              # Sample songs
```

## User Stories

See [USER_STORIES.md](USER_STORIES.md) for detailed user stories and feature roadmap.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for technical architecture and design decisions.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - See LICENSE file for details

## Roadmap

### Current Features (v0.1.0)
- [x] Add and manage songs
- [x] Multiple language support
- [x] Fill-in-the-blank practice
- [x] Flashcard practice
- [x] Line-by-line practice
- [x] Progress tracking
- [x] Basic statistics

### Planned Features
- [ ] Web interface
- [ ] Audio playback integration
- [ ] Spaced repetition algorithm
- [ ] Community song database
- [ ] Mobile app
- [ ] Advanced analytics

## Support

For issues, questions, or suggestions, please open an issue on GitHub.

## Acknowledgments

Built with love for music and language learners everywhere! 🎶🌍
