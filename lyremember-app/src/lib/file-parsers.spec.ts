import { describe, it, expect } from 'vitest';
import {
  parseTxt,
  parseJson,
  parseLrc,
  parseByExtension,
  FileImportError,
} from './file-parsers';

describe('parseTxt', () => {
  it('returns one entry per non-empty line', () => {
    const { lyrics } = parseTxt('Line A\nLine B\n\nLine C\n');
    expect(lyrics).toEqual(['Line A', 'Line B', 'Line C']);
  });

  it('skips comments starting with #', () => {
    const { lyrics } = parseTxt('# title\nLine A\n# comment\nLine B');
    expect(lyrics).toEqual(['Line A', 'Line B']);
  });

  it('handles CRLF line endings', () => {
    const { lyrics } = parseTxt('A\r\nB\r\n');
    expect(lyrics).toEqual(['A', 'B']);
  });
});

describe('parseJson', () => {
  it('parses a complete payload', () => {
    const r = parseJson(
      JSON.stringify({ title: 'T', artist: 'A', language: 'en', lyrics: ['x', 'y'] }),
    );
    expect(r).toEqual({ title: 'T', artist: 'A', language: 'en', lyrics: ['x', 'y'] });
  });

  it('allows optional fields to be missing', () => {
    const r = parseJson(JSON.stringify({ lyrics: ['x'] }));
    expect(r.lyrics).toEqual(['x']);
    expect(r.title).toBeUndefined();
  });

  it('throws on invalid JSON', () => {
    expect(() => parseJson('not json')).toThrow(FileImportError);
  });

  it('throws when lyrics is not an array of strings', () => {
    expect(() => parseJson(JSON.stringify({ lyrics: [1, 2] }))).toThrow(FileImportError);
    expect(() => parseJson(JSON.stringify({}))).toThrow(FileImportError);
  });
});

describe('parseLrc', () => {
  it('strips leading timestamps', () => {
    const r = parseLrc('[00:00.00]Line A\n[00:03.50]Line B');
    expect(r.lyrics).toEqual(['Line A', 'Line B']);
  });

  it('extracts ti/ar/la metadata tags', () => {
    const r = parseLrc('[ti:My Song]\n[ar:Some Artist]\n[la:fr]\n[00:00]Bonjour');
    expect(r.title).toBe('My Song');
    expect(r.artist).toBe('Some Artist');
    expect(r.language).toBe('fr');
    expect(r.lyrics).toEqual(['Bonjour']);
  });

  it('handles multiple timestamps on a single line', () => {
    const r = parseLrc('[00:01.00][00:30.00]Repeated line');
    expect(r.lyrics).toEqual(['Repeated line']);
  });

  it('skips empty lines and lines that become empty after stripping', () => {
    const r = parseLrc('\n[00:01]\n[00:02]Hello');
    expect(r.lyrics).toEqual(['Hello']);
  });
});

describe('parseByExtension', () => {
  it('dispatches to parseTxt for .txt', () => {
    const r = parseByExtension('song.txt', 'A\nB');
    expect(r.lyrics).toEqual(['A', 'B']);
  });

  it('dispatches to parseLrc for .lrc', () => {
    const r = parseByExtension('song.lrc', '[ti:T]\n[00:00]A');
    expect(r.title).toBe('T');
  });

  it('rejects unknown extensions', () => {
    expect(() => parseByExtension('song.md', 'x')).toThrow(FileImportError);
  });
});
