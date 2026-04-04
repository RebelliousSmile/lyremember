import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useUiStore } from '../ui'

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

// Mock matchMedia
Object.defineProperty(globalThis, 'matchMedia', {
  value: vi.fn().mockReturnValue({ matches: false }),
})

describe('useUiStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorageMock.clear()
    document.documentElement.classList.remove('dark')
  })

  it('has correct initial state', () => {
    const store = useUiStore()
    expect(store.sidebarOpen).toBe(true)
    expect(store.darkMode).toBe(false)
    expect(store.mobileMenuOpen).toBe(false)
  })

  it('toggles sidebar', () => {
    const store = useUiStore()
    store.toggleSidebar()
    expect(store.sidebarOpen).toBe(false)
    store.toggleSidebar()
    expect(store.sidebarOpen).toBe(true)
  })

  it('opens and closes sidebar', () => {
    const store = useUiStore()
    store.closeSidebar()
    expect(store.sidebarOpen).toBe(false)
    store.openSidebar()
    expect(store.sidebarOpen).toBe(true)
  })

  it('toggles dark mode and persists to localStorage', () => {
    const store = useUiStore()
    store.toggleDarkMode()
    expect(store.darkMode).toBe(true)
    expect(document.documentElement.classList.contains('dark')).toBe(true)
    expect(localStorageMock.setItem).toHaveBeenCalledWith('darkMode', 'true')

    store.toggleDarkMode()
    expect(store.darkMode).toBe(false)
    expect(document.documentElement.classList.contains('dark')).toBe(false)
  })

  it('initializes dark mode from localStorage', () => {
    localStorageMock.store['darkMode'] = 'true'
    const store = useUiStore()
    store.initializeDarkMode()
    expect(store.darkMode).toBe(true)
    expect(document.documentElement.classList.contains('dark')).toBe(true)
  })

  it('initializes dark mode from system preference when no saved value', () => {
    vi.mocked(globalThis.matchMedia).mockReturnValueOnce({ matches: true } as MediaQueryList)
    const store = useUiStore()
    store.initializeDarkMode()
    expect(store.darkMode).toBe(true)
  })

  it('toggles mobile menu', () => {
    const store = useUiStore()
    store.toggleMobileMenu()
    expect(store.mobileMenuOpen).toBe(true)
    store.closeMobileMenu()
    expect(store.mobileMenuOpen).toBe(false)
  })
})
