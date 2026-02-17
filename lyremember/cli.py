"""Command-line interface for LyRemember."""

import click
from colorama import init, Fore, Style
from tabulate import tabulate
from lyremember.storage import Storage
from lyremember.song_manager import SongManager
from lyremember.practice_engine import PracticeEngine
from lyremember.progress_tracker import ProgressTracker
from lyremember.utils import check_answer, truncate_text

# Initialize colorama
init(autoreset=True)

# Global storage instance
storage = Storage()
song_manager = SongManager(storage)
progress_tracker = ProgressTracker(storage)


@click.group()
@click.version_option(version='0.1.0')
def cli():
    """LyRemember - Memorize lyrics in multiple languages!"""
    pass


@cli.command()
@click.option('--title', prompt='Song title', help='Title of the song')
@click.option('--artist', prompt='Artist name', help='Name of the artist')
@click.option('--language', prompt='Language code (e.g., en, es, fr)', help='Language code')
def add(title, artist, language):
    """Add a new song to your collection."""
    click.echo(f"\n{Fore.CYAN}Enter lyrics (one line at a time, empty line to finish):{Style.RESET_ALL}")
    
    lyrics = []
    while True:
        line = input()
        if line == '':
            if lyrics:  # At least one line entered
                break
            else:
                click.echo(f"{Fore.YELLOW}Please enter at least one line of lyrics.{Style.RESET_ALL}")
        else:
            lyrics.append(line)
    
    song = song_manager.add_song(title, artist, language, lyrics)
    click.echo(f"\n{Fore.GREEN}✓ Song added successfully!{Style.RESET_ALL}")
    click.echo(f"  ID: {song.id}")
    click.echo(f"  Title: {song.title}")
    click.echo(f"  Artist: {song.artist}")
    click.echo(f"  Language: {song.language}")
    click.echo(f"  Lines: {len(song.lyrics)}")


@cli.command()
@click.option('--language', help='Filter by language')
@click.option('--search', help='Search by title or artist')
def list(language, search):
    """List all songs in your collection."""
    songs = song_manager.get_all_songs()
    
    if language:
        songs = [s for s in songs if s.language == language]
    
    if search:
        songs = song_manager.search_songs(search)
    
    if not songs:
        click.echo(f"{Fore.YELLOW}No songs found.{Style.RESET_ALL}")
        click.echo(f"\nUse '{Fore.CYAN}lyremember add{Style.RESET_ALL}' to add your first song!")
        return
    
    # Prepare table data
    table_data = []
    for song in songs:
        table_data.append([
            truncate_text(song.id, 12),
            truncate_text(song.title, 30),
            truncate_text(song.artist, 25),
            song.language,
            len(song.lyrics)
        ])
    
    headers = ['ID', 'Title', 'Artist', 'Language', 'Lines']
    click.echo(f"\n{Fore.CYAN}Your Songs Collection:{Style.RESET_ALL}")
    click.echo(tabulate(table_data, headers=headers, tablefmt='grid'))
    click.echo(f"\nTotal: {len(songs)} song(s)")


@cli.command()
@click.argument('song_id')
def view(song_id):
    """View a song's lyrics."""
    song = song_manager.get_song(song_id)
    
    if not song:
        click.echo(f"{Fore.RED}✗ Song not found.{Style.RESET_ALL}")
        return
    
    click.echo(f"\n{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}")
    click.echo(f"{Fore.GREEN}{song.title}{Style.RESET_ALL} - {song.artist}")
    click.echo(f"Language: {song.language} | Lines: {len(song.lyrics)}")
    click.echo(f"{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}\n")
    
    for i, line in enumerate(song.lyrics, 1):
        click.echo(f"{Fore.YELLOW}{i:2d}.{Style.RESET_ALL} {line}")
    
    click.echo(f"\n{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}")


@cli.command()
@click.argument('song_id', required=False)
@click.option('--mode', default='fill-blank', type=click.Choice(['fill-blank', 'flashcard', 'line-by-line']), 
              help='Practice mode')
@click.option('--difficulty', default=0.3, type=float, help='Difficulty level (0-1)')
def practice(song_id, mode, difficulty):
    """Start a practice session."""
    
    # Get song
    if song_id:
        song = song_manager.get_song(song_id)
        if not song:
            click.echo(f"{Fore.RED}✗ Song not found.{Style.RESET_ALL}")
            return
    else:
        # Get recommended song
        recommended_id = progress_tracker.get_recommended_practice_song()
        if recommended_id:
            song = song_manager.get_song(recommended_id)
            click.echo(f"{Fore.CYAN}Recommended song for practice:{Style.RESET_ALL}")
        else:
            # Get any song
            songs = song_manager.get_all_songs()
            if not songs:
                click.echo(f"{Fore.YELLOW}No songs available. Add some songs first!{Style.RESET_ALL}")
                return
            song = songs[0]
    
    click.echo(f"\n{Fore.GREEN}Starting practice:{Style.RESET_ALL} {song.title} - {song.artist}")
    click.echo(f"Mode: {mode} | Difficulty: {difficulty}\n")
    
    # Initialize practice engine
    engine = PracticeEngine(song, mode, difficulty)
    
    if mode == 'fill-blank':
        practice_fill_blank(engine)
    elif mode == 'flashcard':
        practice_flashcard(engine)
    elif mode == 'line-by-line':
        practice_line_by_line(engine)
    
    # Save session
    session = engine.create_session()
    progress_tracker.record_session(session)
    
    # Show results
    click.echo(f"\n{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}")
    click.echo(f"{Fore.GREEN}Practice Complete!{Style.RESET_ALL}")
    click.echo(f"Score: {session.score:.1f}%")
    click.echo(f"Lines practiced: {session.lines_practiced}")
    click.echo(f"Lines correct: {session.lines_correct}")
    click.echo(f"Duration: {session.duration_seconds}s")
    click.echo(f"{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}")


def practice_fill_blank(engine: PracticeEngine):
    """Run fill-in-the-blank practice."""
    items = engine.fill_in_blank_practice()
    
    for idx, (modified_line, hidden_words, original_line) in enumerate(items):
        click.echo(f"\n{Fore.YELLOW}Line {idx + 1}:{Style.RESET_ALL}")
        click.echo(f"{Fore.CYAN}{modified_line}{Style.RESET_ALL}")
        click.echo(f"\nFill in the {len(hidden_words)} missing word(s):")
        
        user_words = []
        for i in range(len(hidden_words)):
            word = input(f"  Word {i + 1}: ").strip()
            user_words.append(word)
        
        # Check answer
        all_correct, num_correct = engine.check_fill_blank_answer(user_words, hidden_words)
        
        if all_correct:
            click.echo(f"{Fore.GREEN}✓ Correct!{Style.RESET_ALL}")
            engine.record_line_result(idx, True)
        else:
            click.echo(f"{Fore.RED}✗ Incorrect{Style.RESET_ALL}")
            click.echo(f"Correct: {' '.join(hidden_words)}")
            click.echo(f"Full line: {original_line}")
            engine.record_line_result(idx, False)


def practice_flashcard(engine: PracticeEngine):
    """Run flashcard practice."""
    flashcards = engine.flashcard_practice()
    
    for idx, (prompt, answer) in enumerate(flashcards):
        click.echo(f"\n{Fore.YELLOW}Card {idx + 1}/{len(flashcards)}:{Style.RESET_ALL}")
        click.echo(f"{Fore.CYAN}{prompt}{Style.RESET_ALL}")
        
        user_answer = input("\nComplete the line: ").strip()
        
        # Check answer
        if check_answer(user_answer, answer):
            click.echo(f"{Fore.GREEN}✓ Correct!{Style.RESET_ALL}")
            engine.record_line_result(idx, True)
        else:
            click.echo(f"{Fore.RED}✗ Incorrect{Style.RESET_ALL}")
            click.echo(f"Correct answer: {answer}")
            engine.record_line_result(idx, False)


def practice_line_by_line(engine: PracticeEngine):
    """Run line-by-line practice."""
    lines = engine.line_by_line_practice()
    
    for idx, line in enumerate(lines):
        click.echo(f"\n{Fore.YELLOW}Line {idx + 1}/{len(lines)}:{Style.RESET_ALL}")
        click.echo("Type the line:")
        
        user_line = input("> ").strip()
        
        # Check answer
        if check_answer(user_line, line):
            click.echo(f"{Fore.GREEN}✓ Correct!{Style.RESET_ALL}")
            engine.record_line_result(idx, True)
        else:
            click.echo(f"{Fore.RED}✗ Incorrect{Style.RESET_ALL}")
            click.echo(f"Correct: {line}")
            engine.record_line_result(idx, False)


@cli.command()
@click.argument('song_id', required=False)
def progress(song_id):
    """View practice statistics."""
    
    if song_id:
        # Show progress for specific song
        prog = progress_tracker.get_song_progress(song_id)
        song = song_manager.get_song(song_id)
        
        if not prog or not song:
            click.echo(f"{Fore.YELLOW}No progress data for this song yet.{Style.RESET_ALL}")
            return
        
        click.echo(f"\n{Fore.CYAN}Progress for: {song.title}{Style.RESET_ALL}")
        click.echo(f"Mastery: {prog.mastery_level * 100:.1f}%")
        click.echo(f"Total practice time: {prog.total_practice_time}s")
        click.echo(f"Sessions: {len(prog.practice_sessions)}")
        click.echo(f"Last practiced: {prog.last_practiced or 'Never'}")
    else:
        # Show overall statistics
        stats = progress_tracker.get_statistics()
        
        click.echo(f"\n{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}")
        click.echo(f"{Fore.GREEN}Your Learning Statistics{Style.RESET_ALL}")
        click.echo(f"{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}\n")
        
        click.echo(f"Songs practiced: {stats['total_songs_practiced']}")
        click.echo(f"Total sessions: {stats['total_sessions']}")
        click.echo(f"Practice time: {stats['total_practice_time_formatted']}")
        click.echo(f"Average mastery: {stats['average_mastery'] * 100:.1f}%")
        click.echo(f"Average accuracy: {stats['average_accuracy']:.1f}%")
        
        click.echo(f"\n{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}")


@cli.command()
@click.argument('song_id')
def delete(song_id):
    """Delete a song from your collection."""
    song = song_manager.get_song(song_id)
    
    if not song:
        click.echo(f"{Fore.RED}✗ Song not found.{Style.RESET_ALL}")
        return
    
    if click.confirm(f"Delete '{song.title}' by {song.artist}?"):
        song_manager.delete_song(song_id)
        click.echo(f"{Fore.GREEN}✓ Song deleted.{Style.RESET_ALL}")
    else:
        click.echo("Cancelled.")


if __name__ == '__main__':
    cli()
