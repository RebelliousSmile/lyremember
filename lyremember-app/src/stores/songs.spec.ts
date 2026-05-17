import { describe, it, expect, beforeEach } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useSongsStore } from './songs';
import type { Song } from '../types';

function makeSong(over: Partial<Song> = {}): Song {
  return {
    id: 's1',
    title: 'Imagine',
    artist: 'John Lennon',
    language: 'en',
    lyrics: ['Imagine there’s no heaven'],
    phonetic_lyrics: null,
    translations: null,
    genius_id: null,
    genius_url: null,
    created_at: '2026-01-01T00:00:00Z',
    updated_at: '2026-01-01T00:00:00Z',
    ...over,
  };
}

describe('songs store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it('filters by case-insensitive search query on title and artist', () => {
    const store = useSongsStore();
    store.songs = [
      makeSong({ id: 'a', title: 'Imagine', artist: 'John Lennon' }),
      makeSong({ id: 'b', title: 'Yesterday', artist: 'The Beatles' }),
      makeSong({ id: 'c', title: 'Hello', artist: 'Adele' }),
    ];
    store.searchQuery = 'beat';
    expect(store.filteredSongs.map((s) => s.id)).toEqual(['b']);
  });

  it('filters by language when selectedLanguage is not "all"', () => {
    const store = useSongsStore();
    store.songs = [
      makeSong({ id: 'a', language: 'en' }),
      makeSong({ id: 'b', language: 'fr' }),
      makeSong({ id: 'c', language: 'fr' }),
    ];
    store.selectedLanguage = 'fr';
    expect(store.filteredSongs.map((s) => s.id).sort()).toEqual(['b', 'c']);
  });

  it('groups songs by language', () => {
    const store = useSongsStore();
    store.songs = [
      makeSong({ id: 'a', language: 'en' }),
      makeSong({ id: 'b', language: 'fr' }),
      makeSong({ id: 'c', language: 'en' }),
    ];
    expect(Object.keys(store.songsByLanguage).sort()).toEqual(['en', 'fr']);
    expect(store.songsByLanguage.en.length).toBe(2);
    expect(store.songsByLanguage.fr.length).toBe(1);
  });

  it('reports total songs count', () => {
    const store = useSongsStore();
    expect(store.totalSongs).toBe(0);
    store.songs = [makeSong(), makeSong({ id: 's2' })];
    expect(store.totalSongs).toBe(2);
  });
});
