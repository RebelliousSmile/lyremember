import { describe, expect, it } from 'vitest';
import en from './locales/en.json';
import fr from './locales/fr.json';
import ja from './locales/ja.json';
import ko from './locales/ko.json';

type Json = Record<string, unknown>;

function flattenKeys(obj: Json, prefix = ''): string[] {
  const keys: string[] = [];
  for (const [k, v] of Object.entries(obj)) {
    const path = prefix ? `${prefix}.${k}` : k;
    if (v && typeof v === 'object' && !Array.isArray(v)) {
      keys.push(...flattenKeys(v as Json, path));
    } else {
      keys.push(path);
    }
  }
  return keys;
}

const reference = flattenKeys(en as Json).sort();

const locales: Array<[string, Json]> = [
  ['fr', fr as Json],
  ['ja', ja as Json],
  ['ko', ko as Json],
];

describe('i18n parity', () => {
  for (const [name, locale] of locales) {
    it(`${name}.json has the same keys as en.json`, () => {
      const localeKeys = flattenKeys(locale).sort();

      const missing = reference.filter((k) => !localeKeys.includes(k));
      const extra = localeKeys.filter((k) => !reference.includes(k));

      expect(
        missing,
        `Missing in ${name}.json:\n  ${missing.join('\n  ')}`,
      ).toEqual([]);
      expect(
        extra,
        `Extra keys in ${name}.json (not in en.json):\n  ${extra.join('\n  ')}`,
      ).toEqual([]);
    });
  }
});
