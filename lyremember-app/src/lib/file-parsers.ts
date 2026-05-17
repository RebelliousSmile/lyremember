/**
 * Parsers for song files that an end user can import into LyRemember.
 *
 * Three formats are supported:
 *  - .txt  : one lyric line per file line. Lines starting with `#` are
 *            treated as comments and skipped. Empty lines are skipped.
 *  - .json : { title?, artist?, language?, lyrics: string[] }.
 *  - .lrc  : LRC karaoke format. Optional `[mm:ss.xx]` (or `[mm:ss]`)
 *            timestamps at the start of lines are stripped; the remaining
 *            text is kept. `[ti:]`, `[ar:]`, `[la:]` metadata tags are
 *            extracted into title/artist/language.
 *
 * No file system access — callers pass a raw string read from a File or
 * a Tauri fs plugin call.
 */

export interface ParsedSong {
  title?: string;
  artist?: string;
  language?: string;
  lyrics: string[];
}

export class FileImportError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'FileImportError';
  }
}

export function parseTxt(content: string): ParsedSong {
  const lyrics = content
    .split(/\r?\n/)
    .map(line => line.replace(/\r$/, ''))
    .filter(line => !line.trimStart().startsWith('#'))
    .filter(line => line.trim() !== '');
  return { lyrics };
}

export function parseJson(content: string): ParsedSong {
  let data: unknown;
  try {
    data = JSON.parse(content);
  } catch (e) {
    throw new FileImportError(`Invalid JSON: ${(e as Error).message}`);
  }
  if (data === null || typeof data !== 'object' || Array.isArray(data)) {
    throw new FileImportError('JSON file must contain an object.');
  }
  const obj = data as Record<string, unknown>;
  const lyrics = obj.lyrics;
  if (!Array.isArray(lyrics) || !lyrics.every(l => typeof l === 'string')) {
    throw new FileImportError("'lyrics' must be an array of strings.");
  }
  return {
    title: typeof obj.title === 'string' ? obj.title : undefined,
    artist: typeof obj.artist === 'string' ? obj.artist : undefined,
    language: typeof obj.language === 'string' ? obj.language : undefined,
    lyrics: lyrics as string[],
  };
}

const LRC_META_TAGS: Record<string, keyof Pick<ParsedSong, 'title' | 'artist' | 'language'>> = {
  ti: 'title',
  ar: 'artist',
  la: 'language',
};

/** Match a leading timestamp like `[01:23.45]` or `[01:23]`. */
const LRC_TIMESTAMP_RE = /^\[\d{1,2}:\d{2}(?:[.:]\d{1,3})?\]/;
/** Match a metadata tag like `[ti:Title]`. */
const LRC_META_RE = /^\[([a-z]{2,3}):([^\]]*)\]\s*$/i;

export function parseLrc(content: string): ParsedSong {
  const out: ParsedSong = { lyrics: [] };
  for (const rawLine of content.split(/\r?\n/)) {
    const line = rawLine.replace(/\r$/, '');
    if (line.trim() === '') continue;

    const meta = LRC_META_RE.exec(line);
    if (meta) {
      const tag = meta[1].toLowerCase();
      const value = meta[2].trim();
      const field = LRC_META_TAGS[tag];
      if (field && value !== '') {
        out[field] = value;
      }
      continue;
    }

    // Strip one or more leading timestamps (some LRC files have several).
    let stripped = line;
    while (LRC_TIMESTAMP_RE.test(stripped)) {
      stripped = stripped.replace(LRC_TIMESTAMP_RE, '');
    }
    const text = stripped.trim();
    if (text !== '') {
      out.lyrics.push(text);
    }
  }
  return out;
}

export function parseByExtension(filename: string, content: string): ParsedSong {
  const ext = filename.toLowerCase().split('.').pop();
  switch (ext) {
    case 'txt':
      return parseTxt(content);
    case 'json':
      return parseJson(content);
    case 'lrc':
      return parseLrc(content);
    default:
      throw new FileImportError(`Unsupported file extension: .${ext ?? '<none>'}`);
  }
}
