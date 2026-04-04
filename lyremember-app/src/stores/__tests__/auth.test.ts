import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useAuthStore } from '../auth'

// Mock tauri-api
vi.mock('../../lib/tauri-api', () => ({
  register: vi.fn(),
  login: vi.fn(),
  verifyToken: vi.fn(),
}))

import * as api from '../../lib/tauri-api'

// Mock localStorage
const localStorageMock = {
  store: {} as Record<string, string>,
  getItem: vi.fn((key: string) => localStorageMock.store[key] ?? null),
  setItem: vi.fn((key: string, value: string) => { localStorageMock.store[key] = value }),
  removeItem: vi.fn((key: string) => { delete localStorageMock.store[key] }),
  clear: vi.fn(() => { localStorageMock.store = {} }),
  get length() { return Object.keys(localStorageMock.store).length },
  key: vi.fn((_: number) => null),
}
Object.defineProperty(globalThis, 'localStorage', { value: localStorageMock })

const mockUser = {
  id: 'user-123',
  username: 'testuser',
  email: 'test@example.com',
  password_hash: '',
  genius_token: null,
  created_at: '2024-01-01T00:00:00Z',
}

describe('useAuthStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorageMock.clear()
    vi.clearAllMocks()
  })

  it('has correct initial state', () => {
    const store = useAuthStore()
    expect(store.user).toBeNull()
    expect(store.token).toBeNull()
    expect(store.loading).toBe(false)
    expect(store.error).toBeNull()
    expect(store.isAuthenticated).toBe(false)
    expect(store.username).toBe('')
  })

  it('registers a user', async () => {
    vi.mocked(api.register).mockResolvedValueOnce(mockUser)
    const store = useAuthStore()

    const result = await store.register('testuser', 'test@example.com', 'password123')
    expect(result).toEqual(mockUser)
    expect(store.user).toEqual(mockUser)
    expect(store.loading).toBe(false)
    expect(api.register).toHaveBeenCalledWith('testuser', 'test@example.com', 'password123')
  })

  it('handles register error', async () => {
    vi.mocked(api.register).mockRejectedValueOnce(new Error('Username taken'))
    const store = useAuthStore()

    await expect(store.register('testuser', 'test@example.com', 'pw')).rejects.toThrow()
    expect(store.error).toBe('Username taken')
    expect(store.loading).toBe(false)
  })

  it('logs in and saves token', async () => {
    vi.mocked(api.login).mockResolvedValueOnce('jwt-token-123')
    vi.mocked(api.verifyToken).mockResolvedValueOnce(mockUser)
    const store = useAuthStore()

    await store.login('testuser', 'password123')
    expect(store.token).toBe('jwt-token-123')
    expect(store.user).toEqual(mockUser)
    expect(store.isAuthenticated).toBe(true)
    expect(store.username).toBe('testuser')
    expect(localStorageMock.setItem).toHaveBeenCalledWith('auth_token', 'jwt-token-123')
  })

  it('logs out and clears state', async () => {
    const store = useAuthStore()
    store.user = mockUser
    store.token = 'jwt-token-123'

    await store.logout()
    expect(store.user).toBeNull()
    expect(store.token).toBeNull()
    expect(store.isAuthenticated).toBe(false)
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('auth_token')
  })

  it('checkAuth restores session from localStorage', async () => {
    localStorageMock.store['auth_token'] = 'saved-token'
    vi.mocked(api.verifyToken).mockResolvedValueOnce(mockUser)
    const store = useAuthStore()

    const result = await store.checkAuth()
    expect(result).toBe(true)
    expect(store.user).toEqual(mockUser)
    expect(store.token).toBe('saved-token')
  })

  it('checkAuth returns false when no saved token', async () => {
    const store = useAuthStore()
    const result = await store.checkAuth()
    expect(result).toBe(false)
  })

  it('checkAuth clears invalid token', async () => {
    localStorageMock.store['auth_token'] = 'expired-token'
    vi.mocked(api.verifyToken).mockRejectedValueOnce(new Error('Token expired'))
    const store = useAuthStore()

    const result = await store.checkAuth()
    expect(result).toBe(false)
    expect(store.user).toBeNull()
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('auth_token')
  })

  it('clears error', () => {
    const store = useAuthStore()
    store.error = 'some error'
    store.clearError()
    expect(store.error).toBeNull()
  })
})
