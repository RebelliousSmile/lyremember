"""User management for LyRemember."""

from typing import Optional
from lyremember.models import User
from lyremember.storage import Storage
from datetime import datetime, timezone


class UserManager:
    """Manages user accounts in the application."""
    
    def __init__(self, storage: Storage):
        """Initialize user manager with storage."""
        self.storage = storage
        self.current_user: Optional[User] = None
    
    def register(self, username: str, email: str, password: str) -> Optional[User]:
        """
        Register a new user.
        
        Args:
            username: Username
            email: Email address
            password: Plain text password (will be hashed)
        
        Returns:
            User object if successful, None if username exists
        """
        # Check if username already exists
        if self.storage.get_user_by_username(username):
            return None
        
        # Create new user
        user = User(
            username=username,
            email=email,
            password_hash=User.hash_password(password)
        )
        
        self.storage.save_user(user)
        return user
    
    def login(self, username: str, password: str) -> Optional[User]:
        """
        Login a user.
        
        Args:
            username: Username
            password: Plain text password
        
        Returns:
            User object if successful, None if invalid credentials
        """
        user = self.storage.get_user_by_username(username)
        
        if user and user.verify_password(password):
            # Update last login
            user.last_login = datetime.now(timezone.utc).isoformat()
            self.storage.save_user(user)
            self.current_user = user
            return user
        
        return None
    
    def logout(self):
        """Logout the current user."""
        self.current_user = None
    
    def get_current_user(self) -> Optional[User]:
        """Get the currently logged in user."""
        return self.current_user
    
    def add_song_to_repertoire(self, song_id: str) -> bool:
        """
        Add a song to the current user's repertoire.
        
        Args:
            song_id: ID of the song to add
        
        Returns:
            True if successful, False if no user logged in
        """
        if not self.current_user:
            return False
        
        if song_id not in self.current_user.song_repertoire:
            self.current_user.song_repertoire.append(song_id)
            self.storage.save_user(self.current_user)
        
        return True
    
    def remove_song_from_repertoire(self, song_id: str) -> bool:
        """
        Remove a song from the current user's repertoire.
        
        Args:
            song_id: ID of the song to remove
        
        Returns:
            True if successful, False if no user logged in
        """
        if not self.current_user:
            return False
        
        if song_id in self.current_user.song_repertoire:
            self.current_user.song_repertoire.remove(song_id)
            self.storage.save_user(self.current_user)
        
        return True
    
    def get_user_songs(self) -> list:
        """
        Get song IDs in the current user's repertoire.
        
        Returns:
            List of song IDs
        """
        if not self.current_user:
            return []
        
        return self.current_user.song_repertoire
    
    def set_genius_token(self, token: str) -> bool:
        """
        Set Genius API token for the current user.
        
        Args:
            token: Genius API access token
        
        Returns:
            True if successful, False if no user logged in
        """
        if not self.current_user:
            return False
        
        self.current_user.genius_token = token
        self.storage.save_user(self.current_user)
        return True
