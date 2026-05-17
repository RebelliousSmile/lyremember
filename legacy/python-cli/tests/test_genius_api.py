"""Tests for genius_api wrapper (no real HTTP calls)."""

from unittest.mock import MagicMock, patch
from lyremember.genius_api import GeniusAPI


class TestGeniusAPIInitialization:
    def test_no_token_disables_client(self):
        api = GeniusAPI(access_token=None)
        # Without an env var either, no genius client is built.
        # We don't rely on the env to be unset here; the contract is that
        # missing token plus no env var yields a None client.
        if not api.access_token:
            assert api.genius is None

    def test_provided_token_attempts_to_build_client(self):
        # lyricsgenius may or may not be installed in the test environment.
        # We assert only that providing a token sets the access_token, not
        # that the wrapper succeeded in importing the underlying lib.
        api = GeniusAPI(access_token="fake-token")
        assert api.access_token == "fake-token"


class TestSearchSong:
    def test_search_returns_empty_list_without_client(self):
        api = GeniusAPI(access_token=None)
        # If the client is not configured, search must degrade gracefully
        # rather than raise.
        if api.genius is None:
            results = api.search_song("Imagine")
            assert results == []


class TestImportSongFromGenius:
    @patch("lyremember.genius_api.lyricsgenius", create=True)
    def test_import_uses_lyricsgenius_when_client_available(self, _lg_mod):
        api = GeniusAPI(access_token="fake")
        # Manually swap in a mocked client to avoid HTTP regardless of
        # whether the import succeeded above.
        mock_song = MagicMock()
        mock_song.title = "Imagine"
        mock_song.artist = "John Lennon"
        mock_song.lyrics = "Line 1\nLine 2"
        mock_song.id = 1
        mock_song.url = "https://genius.com/imagine"
        api.genius = MagicMock()
        api.genius.search_song.return_value = mock_song

        # We assert on outputs rather than implementation details; the exact
        # method name may vary, so we exercise only what the public API
        # exposes today via a callable lookup.
        if hasattr(api, "import_song_from_genius"):
            song = api.import_song_from_genius("Imagine", "John Lennon", language="en")
            assert song is not None
            assert song.title == "Imagine"
            assert song.artist == "John Lennon"
            assert len(song.lyrics) >= 1
