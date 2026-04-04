# CLAUDE.md - LyRemember

## Project Overview

LyRemember is a multi-platform desktop app for memorizing and practicing song lyrics across multiple languages (French, English, Japanese, Korean). It features automatic phonetic transcription, auto-translation, and multiple practice modes.

**Version:** 0.1.0 | **Status:** Phase 1 complete (backend & integration), Phases 2-5 in progress (UI & features)

## Tech Stack

- **Frontend:** Vue 3 (Composition API) + TypeScript + Vite + Tailwind CSS 3 + Pinia + Vue Router
- **Desktop:** Tauri 2.0 (Rust-based native shell, OS WebView)
- **Backend:** Rust 2021 edition (library crate `lyremember_backend`)
- **Database:** SQLite via rusqlite (bundled)
- **Auth:** bcrypt + JWT
- **Python Bridge:** PyO3 (phonetic generation: pykakasi, hangul-romanize, epitran)
- **Translation:** LibreTranslate API via reqwest
- **Legacy CLI:** Python + Click (proof of concept, `lyremember/` directory)

## Repository Structure

```
lyremember/
├── rust-backend/          # Rust backend library
│   └── src/
│       ├── lib.rs         # Crate root - re-exports modules
│       ├── error.rs       # Custom error types (thiserror)
│       ├── models/        # Data structs: User, Song, PracticeSession
│       ├── db/            # SQLite init & schema (4 tables)
│       └── services/      # Business logic: auth, songs, practice, phonetic, translation
├── lyremember-app/        # Production Tauri + Vue app
│   ├── src/               # Vue 3 frontend
│   │   ├── main.ts        # App entry point
│   │   ├── App.vue        # Root component
│   │   ├── lib/tauri-api.ts   # TypeScript wrappers for 16 Tauri commands
│   │   ├── types/index.ts     # Shared TS type definitions
│   │   ├── router/index.ts    # Vue Router (7 routes, auth guard)
│   │   ├── stores/            # Pinia: auth.ts, songs.ts, ui.ts
│   │   ├── components/        # ui/ (Button, Input, Card, Alert) + layout/
│   │   └── views/             # 7 page components (Login, Register, Dashboard, etc.)
│   └── src-tauri/         # Tauri Rust backend
│       └── src/
│           ├── lib.rs     # Tauri setup, DB init, command registration
│           ├── main.rs    # App entry point
│           └── commands.rs # 16 #[tauri::command] handlers
├── lyremember/            # Legacy Python CLI (Click-based)
├── data/samples/          # Sample song JSON files
└── docs/, tests/          # Documentation and tests
```

## Build & Development Commands

All commands run from `lyremember-app/`:

```bash
# Frontend dev server (http://localhost:1420)
npm run dev

# Full desktop app with HMR (compiles Rust + Vue)
npm run tauri dev

# Production build (type-check + bundle)
npm run build

# Production desktop binary
npm run tauri build
```

Rust backend only (from `rust-backend/`):

```bash
cargo build
cargo test
```

## Architecture

```
Vue 3 Frontend (TypeScript)
    ↓ Tauri invoke()
Tauri 2.0 Command Layer (16 commands)
    ↓ function calls
Rust Backend Services (auth, songs, practice, phonetic, translation)
    ↓
SQLite (local)  |  Python libs (via PyO3)  |  LibreTranslate API
```

**Database schema** (auto-created on startup):
- `users` - id, username, email, password_hash, genius_token, created_at
- `songs` - id, title, artist, language, lyrics, phonetic_lyrics, translations, genius_id, timestamps
- `user_songs` - many-to-many join (user_id, song_id, added_at)
- `practice_sessions` - id, user_id, song_id, mode, score, lines stats, duration, created_at

**Tauri state:** Database connection wrapped in `Mutex<Connection>` managed by Tauri state.

## Code Conventions

### Rust
- snake_case for functions/variables, PascalCase for types/structs
- Custom `Result<T>` type via `thiserror` in `error.rs`
- Services pattern: business logic in `services/`, data structs in `models/`, DB in `db/`
- All models derive `Serialize`/`Deserialize` (serde)
- Async with Tokio for HTTP operations (translation)

### Vue / TypeScript
- Composition API with `<script setup lang="ts">`
- PascalCase for component names, camelCase for functions/variables
- Type imports: `import type { User } from '../types'`
- All Tauri calls go through `lib/tauri-api.ts` (single source of truth)
- Pinia stores for shared state (auth, songs, ui)
- Views = full pages, Components = reusable pieces

### General
- Supported languages: `fr`, `en`, `jp`, `kr`
- Phonetic strategy: "Generate Once, Use Forever" - computed at song creation time
- IDs use UUIDs (uuid crate)
- Timestamps use chrono

## Key Files to Know

| Purpose | File |
|---|---|
| Tauri command API (frontend) | `lyremember-app/src/lib/tauri-api.ts` |
| Tauri command handlers | `lyremember-app/src-tauri/src/commands.rs` |
| Tauri app setup | `lyremember-app/src-tauri/src/lib.rs` |
| Backend library root | `rust-backend/src/lib.rs` |
| DB schema & init | `rust-backend/src/db/sqlite.rs` |
| Auth service (register/login/JWT) | `rust-backend/src/services/auth.rs` |
| Song CRUD | `rust-backend/src/services/songs.rs` |
| Phonetic generation (PyO3) | `rust-backend/src/services/phonetic.rs` |
| Translation (LibreTranslate) | `rust-backend/src/services/translation.rs` |
| Vue Router & routes | `lyremember-app/src/router/index.ts` |
| TypeScript types | `lyremember-app/src/types/index.ts` |
| Tailwind entry | `lyremember-app/src/styles/main.css` |
| Tauri config | `lyremember-app/src-tauri/tauri.conf.json` |

## Configuration

- **Vite:** `lyremember-app/vite.config.ts` - Vue plugin, port 1420, Tauri HMR
- **TypeScript:** `lyremember-app/tsconfig.json` - ES2020 target, strict mode, noUnusedLocals/Params
- **Tauri:** `lyremember-app/src-tauri/tauri.conf.json` - identifier `com.runner.lyremember-app`
- **Rust backend:** `rust-backend/Cargo.toml` - rusqlite (bundled), pyo3, reqwest, bcrypt, jwt

## Things to Watch Out For

- **PyO3 dependency:** Phonetic service requires Python 3 with pykakasi, hangul-romanize, and epitran installed. May fail if Python environment is not set up.
- **LibreTranslate:** Translation service calls an external API; needs network access and a running LibreTranslate instance.
- **DB location:** SQLite database lives in the OS app data directory (`~/.config/lyremember-app/lyremember.db` on Linux).
- **No CI/CD yet:** No automated pipelines; build and test locally.
- **Legacy Python CLI:** The `lyremember/` directory is a proof-of-concept CLI. The production app is in `lyremember-app/` + `rust-backend/`.
