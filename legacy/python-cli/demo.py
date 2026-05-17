#!/usr/bin/env python3
"""
Demo script for LyRemember application.
Showcases the main features of the application.
"""

from lyremember.song_manager import SongManager
from lyremember.storage import Storage
from lyremember.practice_engine import PracticeEngine
from lyremember.progress_tracker import ProgressTracker
from colorama import init, Fore, Style

# Initialize colorama
init(autoreset=True)

def print_header(text):
    """Print a colored header."""
    print(f"\n{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}")
    print(f"{Fore.GREEN}{text}{Style.RESET_ALL}")
    print(f"{Fore.CYAN}{'=' * 60}{Style.RESET_ALL}\n")


def demo():
    """Run the demo."""
    print_header("LyRemember Demo - Lyrics Memorization Application")
    
    # Initialize components
    storage = Storage()
    manager = SongManager(storage)
    tracker = ProgressTracker(storage)
    
    # Demo 1: Add songs
    print_header("1. Adding Songs in Multiple Languages")
    
    # English song
    song_en = manager.add_song(
        title="Twinkle Twinkle Little Star",
        artist="Traditional",
        language="en",
        lyrics=[
            "Twinkle, twinkle, little star",
            "How I wonder what you are",
            "Up above the world so high",
            "Like a diamond in the sky"
        ]
    )
    print(f"{Fore.GREEN}✓{Style.RESET_ALL} Added English song: {song_en.title}")
    
    # Spanish song
    song_es = manager.add_song(
        title="Las Mañanitas",
        artist="Traditional Mexican",
        language="es",
        lyrics=[
            "Estas son las mañanitas",
            "Que cantaba el Rey David",
            "Hoy por ser día de tu santo",
            "Te las cantamos a ti"
        ]
    )
    print(f"{Fore.GREEN}✓{Style.RESET_ALL} Added Spanish song: {song_es.title}")
    
    # Demo 2: List songs
    print_header("2. Listing All Songs")
    songs = manager.get_all_songs()
    for song in songs:
        print(f"  • {song.title} by {song.artist} [{song.language}] - {len(song.lyrics)} lines")
    
    # Demo 3: Practice modes
    print_header("3. Practice Modes")
    
    print(f"{Fore.YELLOW}Fill-in-the-Blank Mode:{Style.RESET_ALL}")
    engine = PracticeEngine(song_en, 'fill-blank', difficulty=0.3)
    items = engine.fill_in_blank_practice()
    
    for i, (modified, hidden, original) in enumerate(items[:2], 1):
        print(f"\n  Line {i}: {Fore.CYAN}{modified}{Style.RESET_ALL}")
        print(f"  Missing: {', '.join(hidden)}")
        print(f"  Original: {original}")
        # Simulate correct answer
        engine.record_line_result(i-1, True)
    
    # Complete session and save
    for i in range(2, len(items)):
        engine.record_line_result(i, True)
    
    session = engine.create_session()
    tracker.record_session(session)
    
    print(f"\n  {Fore.GREEN}Session completed!{Style.RESET_ALL}")
    print(f"  Score: {session.score:.1f}%")
    
    # Demo 4: Flashcard mode
    print(f"\n{Fore.YELLOW}Flashcard Mode:{Style.RESET_ALL}")
    engine2 = PracticeEngine(song_es, 'flashcard', difficulty=0.3)
    flashcards = engine2.flashcard_practice()
    
    for prompt, answer in flashcards[:2]:
        print(f"\n  Prompt: {Fore.CYAN}{prompt}{Style.RESET_ALL}")
        print(f"  Answer: {answer}")
    
    # Demo 5: Progress tracking
    print_header("4. Progress Tracking")
    
    stats = tracker.get_statistics()
    print(f"Total songs practiced: {stats['total_songs_practiced']}")
    print(f"Total sessions: {stats['total_sessions']}")
    print(f"Average mastery: {stats['average_mastery'] * 100:.1f}%")
    print(f"Average accuracy: {stats['average_accuracy']:.1f}%")
    
    # Demo 6: Search and filter
    print_header("5. Search and Filter")
    
    print(f"{Fore.YELLOW}Spanish songs:{Style.RESET_ALL}")
    spanish_songs = manager.filter_by_language("es")
    for song in spanish_songs:
        print(f"  • {song.title}")
    
    print(f"\n{Fore.YELLOW}Search for 'twinkle':{Style.RESET_ALL}")
    search_results = manager.search_songs("twinkle")
    for song in search_results:
        print(f"  • {song.title} by {song.artist}")
    
    # Demo 7: Song details
    print_header("6. Song Details")
    print(f"{Fore.GREEN}{song_en.title}{Style.RESET_ALL} - {song_en.artist}")
    print(f"Language: {song_en.language} | Lines: {len(song_en.lyrics)}\n")
    for i, line in enumerate(song_en.lyrics, 1):
        print(f"{i}. {line}")
    
    print_header("Demo Complete!")
    print("Try it yourself with: lyremember --help")
    print(f"\nKey commands:")
    print(f"  {Fore.CYAN}lyremember add{Style.RESET_ALL}      - Add a new song")
    print(f"  {Fore.CYAN}lyremember list{Style.RESET_ALL}     - List all songs")
    print(f"  {Fore.CYAN}lyremember practice{Style.RESET_ALL} - Start practicing")
    print(f"  {Fore.CYAN}lyremember progress{Style.RESET_ALL} - View statistics")
    print()


if __name__ == '__main__':
    demo()
