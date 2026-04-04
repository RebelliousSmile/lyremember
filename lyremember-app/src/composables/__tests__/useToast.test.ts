import { describe, it, expect, vi, beforeEach } from 'vitest'
import { useToast } from '../useToast'

describe('useToast', () => {
  beforeEach(() => {
    const { toasts } = useToast()
    toasts.value = []
  })

  it('adds a toast', () => {
    const { toasts, add } = useToast()
    add('Hello', 'info', 0)
    expect(toasts.value).toHaveLength(1)
    expect(toasts.value[0].message).toBe('Hello')
    expect(toasts.value[0].type).toBe('info')
  })

  it('adds success/error/info shortcuts', () => {
    const { toasts, success, error, info } = useToast()
    success('ok')
    error('fail')
    info('note')
    expect(toasts.value).toHaveLength(3)
    expect(toasts.value[0].type).toBe('success')
    expect(toasts.value[1].type).toBe('error')
    expect(toasts.value[2].type).toBe('info')
  })

  it('removes a toast by id', () => {
    const { toasts, add, remove } = useToast()
    add('first', 'info', 0)
    add('second', 'info', 0)
    const id = toasts.value[0].id
    remove(id)
    expect(toasts.value).toHaveLength(1)
    expect(toasts.value[0].message).toBe('second')
  })

  it('auto-removes after duration', async () => {
    vi.useFakeTimers()
    const { toasts, add } = useToast()
    add('temp', 'info', 1000)
    expect(toasts.value).toHaveLength(1)
    vi.advanceTimersByTime(1000)
    expect(toasts.value).toHaveLength(0)
    vi.useRealTimers()
  })

  it('shares state across instances', () => {
    const a = useToast()
    const b = useToast()
    a.add('shared', 'info', 0)
    expect(b.toasts.value).toHaveLength(1)
  })
})
