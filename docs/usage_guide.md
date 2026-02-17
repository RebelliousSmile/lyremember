# LyRemember Usage Guide

## Table of Contents
1. [Getting Started](#getting-started)
2. [Managing Your Song Collection](#managing-your-song-collection)
3. [Practice Modes Explained](#practice-modes-explained)
4. [Understanding Your Progress](#understanding-your-progress)
5. [Tips for Effective Learning](#tips-for-effective-learning)
6. [Troubleshooting](#troubleshooting)

## Getting Started

### First Time Setup

After installation, you're ready to start! There's no additional configuration needed. The app will create necessary data files automatically on first use.

### Your First Song

1. Run the add command:
   ```bash
   lyremember add
   ```

2. Enter the song details when prompted:
   - **Title**: The name of the song
   - **Artist**: Who performs it
   - **Language**: Use ISO 639-1 codes (en, es, fr, de, it, pt, etc.)

3. Enter lyrics line by line:
   - Type each line and press Enter
   - Press Enter on an empty line when finished
   - You need at least one line

4. The app confirms your song was added and shows the unique ID

**Pro Tip**: Keep a song file open in a text editor and copy-paste lyrics for faster input!

## Managing Your Song Collection

### Viewing All Songs

```bash
# See all songs
lyremember list

# Filter by language
lyremember list --language es

# Search by title or artist
lyremember list --search "love"
```

### Viewing Song Details

```bash
lyremember view <song-id>
```

This shows:
- Full song title and artist
- Language
- All lyrics with line numbers
- Number of lines

### Deleting Songs

```bash
lyremember delete <song-id>
```

You'll be asked to confirm before deletion.

## Practice Modes Explained

### Fill-in-the-Blank Mode (Default)

**Best for**: Testing specific word recall

**How it works**:
1. Random words are hidden in each line
2. You type the missing words
3. Get immediate feedback

**Tips**:
- Start with lower difficulty (0.2-0.3) to build confidence
- The app accepts minor typos and spelling variations
- Increase difficulty gradually as you improve

**Example**:
```
Line: "Twinkle, twinkle, _____ star"
You type: "little"
Result: ✓ Correct!
```

### Flashcard Mode

**Best for**: Learning line completions

**How it works**:
1. You see the beginning of a line
2. Try to complete it from memory
3. Type your answer
4. See if you got it right

**Tips**:
- Great for practicing transitions between lines
- Focus on the flow and rhythm
- Perfect for songs with repetitive structures

**Example**:
```
Prompt: "Twinkle, twinkle..."
You type: "little star"
Result: ✓ Correct!
```

### Line-by-Line Mode

**Best for**: Full memorization testing

**How it works**:
1. You're prompted for each line
2. Type the complete line from memory
3. Get feedback on accuracy

**Tips**:
- Most challenging mode - use after other modes
- Tests both memory and exact wording
- Great for final preparation before performing

**Example**:
```
Line 1/6:
> Twinkle, twinkle, little star
✓ Correct!
```

## Understanding Your Progress

### Overall Statistics

```bash
lyremember progress
```

Shows:
- **Songs practiced**: How many different songs you've worked on
- **Total sessions**: Number of practice sessions completed
- **Practice time**: Total time spent practicing
- **Average mastery**: Overall skill level (0-100%)
- **Average accuracy**: How often you get answers right

### Song-Specific Progress

```bash
lyremember progress <song-id>
```

Shows for a specific song:
- **Mastery level**: How well you know this song (0-100%)
- **Total practice time**: Time spent on this song
- **Sessions**: Number of times practiced
- **Last practiced**: When you last practiced this song

### Mastery Levels Explained

- **0-25%**: Just started - keep practicing!
- **25-50%**: Getting familiar - you're making progress
- **50-75%**: Pretty good - almost there!
- **75-90%**: Well memorized - great job!
- **90-100%**: Fully mastered - excellent!

## Tips for Effective Learning

### 1. Practice Regularly
- Short daily sessions (10-15 minutes) are more effective than long sporadic ones
- Use `lyremember practice` without song ID to get automatic recommendations

### 2. Progressive Difficulty
Start easy and increase difficulty:
```bash
lyremember practice <song-id> --difficulty 0.2   # Week 1
lyremember practice <song-id> --difficulty 0.4   # Week 2
lyremember practice <song-id> --difficulty 0.6   # Week 3
```

### 3. Mix Practice Modes
- Start with **fill-blank** to learn specific words
- Move to **flashcard** to practice flow
- Finish with **line-by-line** for complete mastery

### 4. Focus on Difficult Lines
- The app tracks which lines you struggle with
- These appear in your session statistics
- Practice songs with lower mastery more often

### 5. Language Learning
- Add translations for better understanding
- Practice pronunciation while typing
- Learn vocabulary in context

### 6. Pre-Performance Preparation
1. Practice at increasing difficulty levels
2. Do final run-throughs in line-by-line mode
3. Check mastery level - aim for 80%+ before performing

## Advanced Usage

### Custom Difficulty Levels

Difficulty controls how many words are hidden in fill-blank mode:
- `0.1` - Very easy (10% words hidden)
- `0.3` - Easy (30% words hidden) - Default
- `0.5` - Medium (50% words hidden)
- `0.7` - Hard (70% words hidden)
- `0.9` - Very hard (90% words hidden)

### Practice Strategies

**Sprint Learning** (New song, tight deadline):
```bash
# Day 1-2: Easy difficulty, all modes
lyremember practice <song-id> --mode fill-blank --difficulty 0.2
lyremember practice <song-id> --mode flashcard

# Day 3-4: Medium difficulty
lyremember practice <song-id> --difficulty 0.5

# Day 5+: Full testing
lyremember practice <song-id> --mode line-by-line
```

**Maintenance Learning** (Keep songs fresh):
```bash
# Weekly review of all songs
lyremember practice  # Let app choose based on need
```

**Language Learning**:
```bash
# Add songs in target language
lyremember add --language es

# Practice regularly with translations
lyremember list --language es
lyremember practice <song-id>
```

## Troubleshooting

### "Song not found" Error
- Check the song ID is correct
- Use `lyremember list` to see all available songs
- Song IDs are case-sensitive

### Wrong Answers Marked Correct (or vice versa)
- The app uses fuzzy matching to allow minor typos
- Similarity threshold is 85%
- If consistently having issues, the lyrics might need correction

### Progress Not Saving
- Check that the `data/` directory exists and is writable
- Ensure you have disk space
- Progress is saved automatically after each session

### Songs Disappeared
- Check you're running from the correct directory
- Look for `data/songs.json` file
- Never delete the data directory

### Can't Install Dependencies
- Ensure Python 3.8+ is installed: `python --version`
- Try upgrading pip: `pip install --upgrade pip`
- Use a virtual environment to avoid conflicts

## Getting Help

- Check this guide and README.md
- Review USER_STORIES.md for feature descriptions
- See ARCHITECTURE.md for technical details
- Open an issue on GitHub for bugs or questions

## Keyboard Shortcuts

While the CLI doesn't support traditional keyboard shortcuts, here are some shell tips:

**Bash/Zsh**:
- `Ctrl+C` - Cancel current operation
- `Ctrl+D` - Exit (in some prompts)
- `↑/↓` - Navigate command history
- `Tab` - Autocomplete commands

**Quick Commands**:
```bash
# Create aliases for frequently used commands
alias lyr='lyremember'
alias lyr-practice='lyremember practice'
alias lyr-list='lyremember list'

# Now you can use:
lyr list
lyr-practice
```

Happy learning! 🎵📚
