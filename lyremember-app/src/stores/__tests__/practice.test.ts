import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { usePracticeStore } from '../practice'

vi.mock('../../lib/tauri-api', () => ({
  createPracticeSession: vi.fn().mockResolvedValue({
    id: 'session-1', user_id: 'u1', song_id: 's1', mode: 'karaoke',
    score: 75, lines_practiced: 4, lines_correct: 3, duration_seconds: 30, created_at: '',
  }),
}))

const mockSong = {
  id: 's1', title: 'Test Song', artist: 'Artist', language: 'en',
  lyrics: ['Line 1', 'Line 2', 'Line 3', 'Line 4'],
  phonetic_lyrics: null, translations: null,
  genius_id: null, genius_url: null,
  created_at: '', updated_at: '',
}

describe('usePracticeStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('has correct initial state', () => {
    const store = usePracticeStore()
    expect(store.song).toBeNull()
    expect(store.mode).toBeNull()
    expect(store.state).toBeNull()
    expect(store.finished).toBe(false)
    expect(store.score).toBe(0)
  })

  it('starts a session', () => {
    const store = usePracticeStore()
    store.startSession(mockSong, 'karaoke')
    expect(store.song).toEqual(mockSong)
    expect(store.mode).toBe('karaoke')
    expect(store.state!.totalLines).toBe(4)
    expect(store.state!.currentLine).toBe(0)
    expect(store.finished).toBe(false)
  })

  it('tracks correct answers', () => {
    const store = usePracticeStore()
    store.startSession(mockSong, 'fill-blank')

    store.answerLine(true)
    expect(store.state!.currentLine).toBe(1)
    expect(store.state!.correctLines).toBe(1)

    store.answerLine(false)
    expect(store.state!.currentLine).toBe(2)
    expect(store.state!.correctLines).toBe(1)
  })

  it('calculates score correctly', () => {
    const store = usePracticeStore()
    store.startSession(mockSong, 'mcq')

    store.answerLine(true)
    store.answerLine(true)
    store.answerLine(false)
    store.answerLine(true)

    expect(store.finished).toBe(true)
    expect(store.score).toBe(75)
  })

  it('finishes when all lines answered', () => {
    const store = usePracticeStore()
    store.startSession(mockSong, 'karaoke')

    for (let i = 0; i < 4; i++) store.answerLine(true)

    expect(store.finished).toBe(true)
    expect(store.score).toBe(100)
  })

  it('does not accept answers after finished', () => {
    const store = usePracticeStore()
    store.startSession(mockSong, 'karaoke')
    for (let i = 0; i < 4; i++) store.answerLine(true)

    store.answerLine(false) // should be ignored
    expect(store.state!.currentLine).toBe(4)
    expect(store.state!.correctLines).toBe(4)
  })

  it('resets state', () => {
    const store = usePracticeStore()
    store.startSession(mockSong, 'karaoke')
    store.answerLine(true)
    store.reset()

    expect(store.song).toBeNull()
    expect(store.mode).toBeNull()
    expect(store.state).toBeNull()
    expect(store.finished).toBe(false)
  })

  it('tracks answer history', () => {
    const store = usePracticeStore()
    store.startSession(mockSong, 'mcq')

    store.answerLine(true)
    store.answerLine(false)
    store.answerLine(true)

    expect(store.state!.answers).toEqual([true, false, true])
  })
})
