// Re-export types from tauri-api for convenience
export type {
  User,
  Song,
  CreateSongResult,
  PracticeSession,
  UserStats,
} from '../lib/tauri-api';

// Additional UI-specific types
export interface LoginForm {
  username: string;
  password: string;
}

export interface RegisterForm {
  username: string;
  email: string;
  password: string;
  confirmPassword: string;
}

export interface CreateSongForm {
  title: string;
  artist: string;
  language: string;
  lyrics: string; // Will be split into array
  autoTranslate: boolean;
}

export type PracticeMode = 'karaoke' | 'fill-blank' | 'mcq' | 'oral';

export interface RouteGuardContext {
  requiresAuth: boolean;
  redirectTo?: string;
}
