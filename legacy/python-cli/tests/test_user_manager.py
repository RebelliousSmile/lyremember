"""Tests for the UserManager.

Note: the storage layer in this legacy POC does not implement the user
persistence methods that UserManager calls (get_user_by_username,
save_user). These tests mock the Storage interface so we can validate
UserManager's own logic independently — this is the legacy contract.
"""

from unittest.mock import MagicMock
from lyremember.models import User
from lyremember.user_manager import UserManager


def _new_user_manager():
    storage = MagicMock()
    storage.get_user_by_username = MagicMock(return_value=None)
    storage.save_user = MagicMock()
    return UserManager(storage)


class TestRegister:
    def test_register_creates_user_when_username_is_free(self):
        manager = _new_user_manager()
        user = manager.register("alice", "a@example.com", "secret")
        assert user is not None
        assert user.username == "alice"
        assert user.email == "a@example.com"
        assert user.verify_password("secret")
        manager.storage.save_user.assert_called_once_with(user)

    def test_register_returns_none_when_username_taken(self):
        manager = _new_user_manager()
        existing = User("alice", "x@example.com", User.hash_password("p"))
        manager.storage.get_user_by_username.return_value = existing

        user = manager.register("alice", "new@example.com", "secret")
        assert user is None
        manager.storage.save_user.assert_not_called()


class TestLogin:
    def test_login_success_sets_current_user_and_updates_last_login(self):
        manager = _new_user_manager()
        existing = User("alice", "x@example.com", User.hash_password("secret"))
        manager.storage.get_user_by_username.return_value = existing
        assert existing.last_login is None

        user = manager.login("alice", "secret")
        assert user is existing
        assert manager.current_user is existing
        assert existing.last_login is not None
        manager.storage.save_user.assert_called_once_with(existing)

    def test_login_wrong_password_returns_none(self):
        manager = _new_user_manager()
        existing = User("alice", "x@example.com", User.hash_password("secret"))
        manager.storage.get_user_by_username.return_value = existing
        assert manager.login("alice", "wrong") is None
        assert manager.current_user is None

    def test_login_unknown_user_returns_none(self):
        manager = _new_user_manager()
        manager.storage.get_user_by_username.return_value = None
        assert manager.login("ghost", "any") is None


class TestRepertoire:
    def test_add_song_appends_unique_ids_only(self):
        manager = _new_user_manager()
        manager.current_user = User("alice", "x@example.com", "h")

        assert manager.add_song_to_repertoire("s1") is True
        assert manager.add_song_to_repertoire("s2") is True
        assert manager.add_song_to_repertoire("s1") is True  # idempotent

        assert manager.current_user.song_repertoire == ["s1", "s2"]

    def test_add_song_without_login_returns_false(self):
        manager = _new_user_manager()
        manager.current_user = None
        assert manager.add_song_to_repertoire("s1") is False

    def test_remove_song_drops_id_from_repertoire(self):
        manager = _new_user_manager()
        manager.current_user = User(
            "alice", "x@example.com", "h", song_repertoire=["s1", "s2"]
        )
        manager.remove_song_from_repertoire("s1")
        assert manager.current_user.song_repertoire == ["s2"]

    def test_get_user_songs_returns_empty_when_logged_out(self):
        manager = _new_user_manager()
        manager.current_user = None
        assert manager.get_user_songs() == []


class TestGeniusToken:
    def test_set_genius_token_updates_current_user(self):
        manager = _new_user_manager()
        manager.current_user = User("alice", "x@example.com", "h")
        manager.set_genius_token("token-123")
        assert manager.current_user.genius_token == "token-123"
        manager.storage.save_user.assert_called_with(manager.current_user)

    def test_set_genius_token_without_login_returns_false(self):
        manager = _new_user_manager()
        manager.current_user = None
        assert manager.set_genius_token("x") is False


class TestLogout:
    def test_logout_clears_current_user(self):
        manager = _new_user_manager()
        manager.current_user = User("alice", "x@example.com", "h")
        manager.logout()
        assert manager.current_user is None
        assert manager.get_current_user() is None
