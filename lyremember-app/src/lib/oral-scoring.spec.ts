import { describe, it, expect } from 'vitest';
import { normalize, tokenize, scoreSpoken, hasSpeechRecognition } from './oral-scoring';

describe('normalize', () => {
  it('lowers case, strips punctuation, collapses whitespace', () => {
    expect(normalize('Hello,  WORLD!')).toBe('hello world');
  });

  it('keeps diacritics intact (phonetically meaningful)', () => {
    expect(normalize('À la française')).toBe('à la française');
  });

  it('returns empty string for whitespace-only input', () => {
    expect(normalize('   \t\n')).toBe('');
  });
});

describe('tokenize', () => {
  it('returns words from a normalized phrase', () => {
    expect(tokenize('Twinkle twinkle little star!')).toEqual([
      'twinkle',
      'twinkle',
      'little',
      'star',
    ]);
  });

  it('returns empty list for empty input', () => {
    expect(tokenize('')).toEqual([]);
  });
});

describe('scoreSpoken', () => {
  it('returns 1.0 when every expected token is present', () => {
    expect(scoreSpoken('Twinkle little star', 'twinkle little star bright')).toBe(1);
  });

  it('returns 0 when no expected token is heard', () => {
    expect(scoreSpoken('Twinkle little star', 'hello world')).toBe(0);
  });

  it('returns 0 for empty expected', () => {
    expect(scoreSpoken('', 'anything')).toBe(0);
  });

  it('ignores order — recall on expected, not on spoken', () => {
    expect(scoreSpoken('one two three', 'three two one')).toBe(1);
  });

  it('returns a fractional score when some tokens are missing', () => {
    // 2 out of 3 expected words heard.
    const s = scoreSpoken('twinkle little star', 'twinkle star');
    expect(s).toBeCloseTo(2 / 3, 5);
  });

  it('is case- and punctuation-insensitive', () => {
    expect(scoreSpoken('Hello, world.', 'HELLO world')).toBe(1);
  });
});

describe('hasSpeechRecognition', () => {
  it('returns false in jsdom (no SpeechRecognition global)', () => {
    expect(hasSpeechRecognition()).toBe(false);
  });
});
