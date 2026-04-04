import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useSongsStore } from '../songs'

// Mock tauri-api
vi.mock('../../lib/tauri-api', () => ({
  getSongs: vi.fn(),
  getSong: vi.fn(),
  getUserSongs: vi.fn(),
  createSong: vi.fn(),
  addToRepertoire: vi.fn(),
  updateSong: vi.fn(),
  deleteSong: vi.fn(),
}))

import * as api from '../../lib/tauri-api'

const mockSongs = [
  { id: '1', title: 'Sakura', artist: 'Artist A', language: 'jp', lyrics: ['line1'], phonetic_lyrics: null, translations: null, genius_id: null, genius_url: null, created_at: '', updated_at: '' },
  { id: '2', title: 'Bonjour', artist: 'Artist B', language: 'fr', lyrics: ['line1'], phonetic_lyrics: null, translations: null, genius_id: null, genius_url: null, created_at: '', updated_at: '' },
  { id: '3', title: 'Hello', artist: 'Artist A', language: 'en', lyrics: ['line1'], phonetic_lyrics: null, translations: null, genius_id: null, genius_url: null, created_at: '', updated_at: '' },
]

describe('useSongsStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('has correct initial state', () => {
    const store = useSongsStore()
    expect(store.songs).toEqual([])
    expect(store.currentSong).toBeNull()
    expect(store.loading).toBe(false)
    expect(store.totalSongs).toBe(0)
  })

  it('fetches all songs', async () => {
    vi.mocked(api.getSongs).mockResolvedValueOnce(mockSongs)
    const store = useSongsStore()

    await store.fetchAllSongs()
    expect(store.songs).toEqual(mockSongs)
    expect(store.totalSongs).toBe(3)
    expect(store.loading).toBe(false)
  })

  it('filters songs by search query', async () => {
    vi.mocked(api.getSongs).mockResolvedValueOnce(mockSongs)
    const store = useSongsStore()
    await store.fetchAllSongs()

    store.setSearchQuery('sakura')
    expect(store.filteredSongs).toHaveLength(1)
    expect(store.filteredSongs[0].title).toBe('Sakura')
  })

  it('filters songs by language', async () => {
    vi.mocked(api.getSongs).mockResolvedValueOnce(mockSongs)
    const store = useSongsStore()
    await store.fetchAllSongs()

    store.setSelectedLanguage('fr')
    expect(store.filteredSongs).toHaveLength(1)
    expect(store.filteredSongs[0].language).toBe('fr')
  })

  it('filters by both search and language', async () => {
    vi.mocked(api.getSongs).mockResolvedValueOnce(mockSongs)
    const store = useSongsStore()
    await store.fetchAllSongs()

    store.setSearchQuery('artist a')
    store.setSelectedLanguage('jp')
    expect(store.filteredSongs).toHaveLength(1)
    expect(store.filteredSongs[0].title).toBe('Sakura')
  })

  it('groups songs by language', async () => {
    vi.mocked(api.getSongs).mockResolvedValueOnce(mockSongs)
    const store = useSongsStore()
    await store.fetchAllSongs()

    expect(store.songsByLanguage['jp']).toHaveLength(1)
    expect(store.songsByLanguage['fr']).toHaveLength(1)
    expect(store.songsByLanguage['en']).toHaveLength(1)
  })

  it('creates a song and adds to list', async () => {
    const newSong = { ...mockSongs[0], id: '4', title: 'New Song' }
    vi.mocked(api.createSong).mockResolvedValueOnce({ song: newSong, warnings: [] })
    const store = useSongsStore()

    const result = await store.createSong('New Song', 'Artist', 'jp', ['lyrics'])
    expect(result.song).toEqual(newSong)
    expect(store.songs).toContainEqual(newSong)
  })

  it('deletes a song and removes from list', async () => {
    vi.mocked(api.getSongs).mockResolvedValueOnce(mockSongs)
    vi.mocked(api.deleteSong).mockResolvedValueOnce(undefined)
    const store = useSongsStore()
    await store.fetchAllSongs()

    await store.deleteSong('1')
    expect(store.songs).toHaveLength(2)
    expect(store.songs.find(s => s.id === '1')).toBeUndefined()
  })

  it('clears currentSong when deleted song is current', async () => {
    vi.mocked(api.getSongs).mockResolvedValueOnce(mockSongs)
    vi.mocked(api.deleteSong).mockResolvedValueOnce(undefined)
    const store = useSongsStore()
    await store.fetchAllSongs()
    store.currentSong = mockSongs[0]

    await store.deleteSong('1')
    expect(store.currentSong).toBeNull()
  })

  it('handles fetch error', async () => {
    vi.mocked(api.getSongs).mockRejectedValueOnce(new Error('Network error'))
    const store = useSongsStore()

    await expect(store.fetchAllSongs()).rejects.toThrow()
    expect(store.error).toBe('Network error')
    expect(store.loading).toBe(false)
  })
})
