/**
 * Token-based similarity scoring for oral practice.
 *
 * The scoring deliberately stays simple and predictable so that it can be
 * unit-tested without depending on the Web Speech API:
 *
 *   - `normalize` lower-cases, strips ASCII punctuation, and collapses
 *     whitespace. It is *not* unicode-aware on diacritics (we keep accents
 *     because they are phonetically significant for French / IPA).
 *   - `tokenize` splits on whitespace after normalization.
 *   - `scoreSpoken(expected, spoken)` returns a number in [0, 1]:
 *       1.0 if every expected token is present in the spoken phrase,
 *       linearly interpolated otherwise.
 *
 * A real STT pipeline would use phoneme distance — this simple ratio is
 * good enough for the MVP feedback ("did the user say roughly that?").
 */

const PUNCT_RE = /[!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~]/g;

export function normalize(text: string): string {
  return text
    .toLowerCase()
    .replace(PUNCT_RE, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

export function tokenize(text: string): string[] {
  const n = normalize(text);
  return n === '' ? [] : n.split(' ');
}

/**
 * Returns a similarity score in [0, 1].
 *
 *   1.0 = every expected token is present in the spoken phrase.
 *   0.0 = no expected token recognized (or expected is empty).
 *
 * Order does not matter — this is a token-set recall on `expected`, which
 * matches the user-perceived contract of "did I say roughly the right
 * words".
 */
export function scoreSpoken(expected: string, spoken: string): number {
  const e = tokenize(expected);
  if (e.length === 0) return 0;
  const s = new Set(tokenize(spoken));
  let hits = 0;
  for (const w of e) {
    if (s.has(w)) hits += 1;
  }
  return hits / e.length;
}

/** Tells whether the runtime exposes a usable SpeechRecognition. */
export function hasSpeechRecognition(): boolean {
  if (typeof window === 'undefined') return false;
  const w = window as unknown as { SpeechRecognition?: unknown; webkitSpeechRecognition?: unknown };
  return Boolean(w.SpeechRecognition || w.webkitSpeechRecognition);
}
