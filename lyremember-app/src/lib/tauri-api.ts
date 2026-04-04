import { invoke } from '@tauri-apps/api/core';

// ==================== TYPES ====================

export interface User {
  id: string;
  username: string;
  email: string;
  genius_token: string | null;
  created_at: string;
}

export interface Song {
  id: string;
  title: string;
  artist: string;
  language: string;
  lyrics: string[];
  phonetic_lyrics: string[] | null;
  translations: Record<string, string[]> | null;
  genius_id: string | null;
  genius_url: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateSongResult {
  song: Song;
  warnings: string[];
}

export interface PracticeSession {
  id: string;
  user_id: string;
  song_id: string;
  mode: string;
  score: number;
  lines_practiced: number;
  lines_correct: number;
  duration_seconds: number;
  created_at: string;
}

export interface UserStats {
  total_sessions: number;
  total_practice_time: number;
  average_score: number;
  total_lines_practiced: number;
  total_lines_correct: number;
}

// ==================== AUTH API ====================

export async function register(
  username: string,
  email: string,
  password: string
): Promise<User> {
  return await invoke('cmd_register', { username, email, password });
}

export async function login(username: string, password: string): Promise<string> {
  return await invoke('cmd_login', { username, password });
}

export async function verifyToken(token: string): Promise<string> {
  return await invoke('cmd_verify_token', { token });
}

export async function getUser(userId: string): Promise<User> {
  return await invoke('cmd_get_user', { userId });
}

// ==================== SONGS API ====================

export async function createSong(
  title: string,
  artist: string,
  language: string,
  lyrics: string[]
): Promise<CreateSongResult> {
  return await invoke('cmd_create_song', { title, artist, language, lyrics });
}

export async function getSongs(): Promise<Song[]> {
  return await invoke('cmd_get_songs');
}

export async function getSong(songId: string): Promise<Song> {
  return await invoke('cmd_get_song', { songId });
}

export async function getUserSongs(userId: string): Promise<Song[]> {
  return await invoke('cmd_get_user_songs', { userId });
}

export async function addToRepertoire(userId: string, songId: string): Promise<void> {
  return await invoke('cmd_add_to_repertoire', { userId, songId });
}

export async function updateSong(
  songId: string,
  title?: string,
  artist?: string,
  lyrics?: string[]
): Promise<void> {
  return await invoke('cmd_update_song', { songId, title, artist, lyrics });
}

export async function deleteSong(songId: string): Promise<void> {
  return await invoke('cmd_delete_song', { songId });
}

// ==================== PRACTICE API ====================

export async function createPracticeSession(
  userId: string,
  songId: string,
  mode: string,
  score: number,
  linesPracticed: number,
  linesCorrect: number,
  durationSeconds: number
): Promise<PracticeSession> {
  return await invoke('cmd_create_practice_session', {
    userId, songId, mode, score, linesPracticed, linesCorrect, durationSeconds,
  });
}

export async function getUserSessions(
  userId: string,
  limit?: number
): Promise<PracticeSession[]> {
  return await invoke('cmd_get_user_sessions', { userId, limit });
}

export async function getUserStats(userId: string): Promise<UserStats> {
  return await invoke('cmd_get_user_stats', { userId });
}

export async function getSongMastery(
  userId: string,
  songId: string
): Promise<number> {
  return await invoke('cmd_get_song_mastery', { userId, songId });
}

// ==================== UTILITY API ====================

export async function translateText(
  text: string,
  sourceLang: string,
  targetLang: string
): Promise<string> {
  return await invoke('cmd_translate_text', { text, sourceLang, targetLang });
}

export async function generatePhonetic(
  text: string[],
  language: string
): Promise<string[]> {
  return await invoke('cmd_generate_phonetic', { text, language });
}

export async function healthCheck(): Promise<string> {
  return await invoke('cmd_health_check');
}
