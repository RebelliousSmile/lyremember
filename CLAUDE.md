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

## Development Principles

This project follows these core principles strictly. All contributions must adhere to them.

### TDD - Test-Driven Development
- **Write tests first**, then implementation code. Red → Green → Refactor.
- Every new feature or bug fix must have corresponding tests before the code is written.
- Rust: use `#[cfg(test)]` modules with `#[test]` functions. Use `tempfile` for DB tests.
- Vue/TypeScript: write unit tests for stores, composables, and utility functions.
- Never consider a feature complete without passing tests.
- Run `cargo test` (backend) before committing Rust changes.

### DRY - Don't Repeat Yourself
- Extract shared logic into reusable functions, modules, or composables.
- Rust: use shared service functions in `services/` rather than duplicating logic across commands.
- Vue: use composables for shared reactive logic, and reusable components in `components/ui/`.
- TypeScript types live in `types/index.ts` — do not redeclare the same interfaces elsewhere.
- Tauri API wrappers live in `lib/tauri-api.ts` — all invoke calls go through this single file.
- If you see the same code in 2+ places, refactor it into a shared abstraction.

### SOLID
- **S - Single Responsibility:** Each module/file/function does one thing. Services handle business logic, models hold data, DB handles persistence, commands handle Tauri bridging.
- **O - Open/Closed:** Extend behavior through new modules/services rather than modifying existing ones. Add new practice modes by creating new service functions, not by bloating existing ones.
- **L - Liskov Substitution:** Respect trait contracts. If a function accepts a trait, any implementation must behave correctly.
- **I - Interface Segregation:** Keep traits and type interfaces focused. Don't force types to implement methods they don't need.
- **D - Dependency Inversion:** High-level modules (commands, views) depend on abstractions (services, stores), not on low-level details (raw SQL, direct HTTP calls).

### KISS - Keep It Simple, Stupid
- Prefer the simplest solution that works. No premature optimization or over-engineering.
- Avoid unnecessary abstractions — a direct function call is better than an over-engineered pattern.
- Clear, readable code over clever code. Name things explicitly.
- If a feature can be implemented in 20 lines, don't write 100.
- Flat is better than nested — minimize deep callback chains and nesting levels.

## Master Deployment Plan

### Phase 1 — Foundation & CI/CD (v0.1.x) ✅ En cours

**Objectif :** Stabiliser le socle existant et mettre en place l'infrastructure de qualite.

#### 1A. Pipeline CI/CD (GitHub Actions)
- [ ] Workflow `ci.yml` : sur chaque push/PR
  - `cargo fmt --check` + `cargo clippy` (lint Rust)
  - `cargo test` (tests unitaires backend)
  - `npm run build` dans `lyremember-app/` (type-check + build Vue)
- [ ] Workflow `build.yml` : build Tauri multi-plateforme
  - Matrix : `ubuntu-latest`, `macos-latest`, `windows-latest`
  - Utiliser `tauri-apps/tauri-action` pour les builds
  - Artefacts uploadés sur chaque PR (pas de release)
- [ ] Cache : `actions/cache` pour `target/`, `node_modules/`, pip packages

#### 1B. Tests de base
- [ ] Tests unitaires Rust : services auth, songs, practice (avec `tempfile` pour SQLite)
- [ ] Tests d'integration Rust : flux complet register → login → create song → practice
- [ ] Validation TypeScript : `vue-tsc --noEmit` dans le CI

#### 1C. Qualite de code
- [ ] Ajouter `rustfmt.toml` (configuration formatage)
- [ ] Ajouter ESLint + Prettier pour Vue/TypeScript
- [ ] Pre-commit hooks via `husky` : lint + format avant chaque commit

**Livrable :** Pipeline verte, tests passent, code formatte automatiquement.

---

### Phase 2 — Core UI Complete (v0.2.0)

**Objectif :** Interface utilisateur fonctionnelle pour toutes les operations de base.

#### 2A. Vues principales
- [ ] **LoginView / RegisterView** : formulaires complets, validation, gestion erreurs
- [ ] **DashboardView** : statistiques utilisateur, chansons recentes, progression
- [ ] **SongsView** : liste des chansons avec recherche/filtre par langue/artiste
- [ ] **AddSongView** : formulaire d'ajout avec preview phonetique et traduction
- [ ] **SongDetailView** : affichage lyrics + phonetique + traductions cote a cote
- [ ] **ProfileView** : settings utilisateur, stats globales

#### 2B. Composants UI
- [ ] Design system Tailwind : couleurs, typographie, espacements coherents
- [ ] Composants reusables : Modal, Toast/Notification, Loading spinner, Empty state
- [ ] Layout responsive : sidebar collapsible, navigation mobile-friendly
- [ ] Dark mode fonctionnel (toggle dans le store `ui.ts`)

#### 2C. Tests frontend
- [ ] Tests unitaires Pinia stores (Vitest)
- [ ] Tests composants avec `@vue/test-utils`
- [ ] Ajouter `vitest` au CI pipeline

**Livrable :** App navigable, toutes les vues fonctionnelles, design coherent.

---

### Phase 3 — Practice Modes (v0.3.0)

**Objectif :** Modes d'entrainement interactifs — coeur de la valeur produit.

#### 3A. Mode Karaoke
- [ ] Affichage progressif des lyrics (ligne par ligne, scroll automatique)
- [ ] Toggle phonetique / traduction en overlay
- [ ] Timer et progression visuelle

#### 3B. Mode Fill-in-the-Blank
- [ ] Algorithme de selection de mots a masquer (frequence, difficulte)
- [ ] Input interactif avec validation temps-reel
- [ ] Score et feedback par ligne

#### 3C. Mode QCM (Multiple Choice)
- [ ] Generation de distracteurs (mots proches, meme chanson)
- [ ] Interface cartes avec feedback immediat
- [ ] Progression adaptative (difficulte croissante)

#### 3D. Statistiques de pratique
- [ ] Vue `PracticeStatsView` : graphiques de progression (chart.js ou equivalent leger)
- [ ] Historique des sessions par chanson
- [ ] Systeme de streaks / objectifs quotidiens

#### 3E. Tests
- [ ] Tests unitaires : algorithmes de scoring, generation de blanks, distracteurs
- [ ] Tests d'integration : flux complet d'une session de pratique
- [ ] Tests E2E avec WebDriver (optionnel, via `tauri-driver`)

**Livrable :** 3 modes de pratique jouables, tracking de progression.

---

### Phase 4 — Features Avancees (v0.4.0)

**Objectif :** Enrichir l'experience avec des fonctionnalites secondaires.

#### 4A. Import Genius API
- [ ] Integration Genius API dans le backend Rust (recherche + lyrics)
- [ ] Vue recherche Genius dans l'UI : search → preview → import
- [ ] Gestion du token API utilisateur (settings)

#### 4B. Internationalisation (i18n)
- [ ] Setup `vue-i18n` avec fichiers de traduction FR / EN / JP / KR
- [ ] Toutes les strings UI externalisees
- [ ] Switcher de langue dans les settings

#### 4C. UX avancee
- [ ] Keyboard shortcuts globaux (navigation, play/pause, next line)
- [ ] Animations/transitions entre vues (Vue Transition)
- [ ] Systeme de notifications in-app (Toast)
- [ ] Mode oral : integration SpeechRecognition (si PyAudio disponible)

#### 4D. Tests
- [ ] Tests i18n : verification que toutes les cles existent dans chaque locale
- [ ] Tests Genius API : mocks des appels HTTP
- [ ] Tests accessibilite (a11y) basiques

**Livrable :** App riche en fonctionnalites, multilingue, raccourcis clavier.

---

### Phase 5 — Pre-release & Polish (v0.9.0)

**Objectif :** Qualite production, packaging, documentation utilisateur.

#### 5A. Packaging multi-plateforme
- [ ] Configuration Tauri bundle :
  - **Linux** : `.deb`, `.AppImage`
  - **macOS** : `.dmg` (+ codesigning si Apple Developer account)
  - **Windows** : `.msi`, `.exe` (+ codesigning si certificat)
- [ ] Bundling Python runtime : embarquer pykakasi/hangul-romanize avec `PyOxidizer` ou sidecar Python
- [ ] Icones et branding finaux pour chaque plateforme
- [ ] Splash screen au demarrage

#### 5B. Gestion des dependances externes
- [ ] **PyO3/Python** : strategie de fallback si Python absent (phonetique desactivee, message clair)
- [ ] **LibreTranslate** : option d'instance locale bundlee OU fallback gracieux si API indisponible
- [ ] Auto-update : configurer `tauri-plugin-updater` avec endpoint GitHub Releases

#### 5C. Performance & securite
- [ ] Audit securite : CSP Tauri, sanitize inputs, pas de secrets dans le frontend
- [ ] Optimisation bundle Vite : code-splitting, lazy loading des vues
- [ ] Profilage memoire Rust (pas de memory leaks sur les sessions longues)
- [ ] SQLite : index sur les colonnes de recherche frequentes

#### 5D. Documentation
- [ ] README utilisateur : installation, premier lancement, guide rapide
- [ ] CONTRIBUTING.md : guide de contribution
- [ ] Changelog genere automatiquement (`git-cliff` ou `conventional-changelog`)

**Livrable :** Binaires prets a distribuer, documentation complete, zero bug critique.

---

### Phase 6 — Release & Distribution (v1.0.0)

**Objectif :** Premiere version publique stable.

#### 6A. Release pipeline
- [ ] Workflow `release.yml` (GitHub Actions) :
  - Declenche sur tag `v*`
  - Build Tauri multi-plateforme (matrix 3 OS)
  - Upload des binaires en GitHub Release
  - Generer changelog automatique
- [ ] Versioning : Semantic Versioning (`MAJOR.MINOR.PATCH`)
- [ ] Branches : `main` = stable, `develop` = integration, feature branches

#### 6B. Distribution
- [ ] **GitHub Releases** : binaires pour les 3 plateformes
- [ ] **Auto-update** : Tauri updater pointe sur GitHub Releases API
- [ ] (Optionnel futur) Homebrew tap (macOS), AUR (Arch Linux), winget (Windows)

#### 6C. Post-release monitoring
- [ ] Sentry ou equivalent pour crash reporting (optionnel, respecter la vie privee)
- [ ] GitHub Issues templates : bug report, feature request
- [ ] Metriques d'usage anonymes (opt-in uniquement)
- [ ] Canal feedback : GitHub Discussions ou Discord

#### 6D. Cycle de maintenance
- [ ] Hotfix : `v1.0.x` pour bugs critiques (branch depuis tag, cherry-pick)
- [ ] Minor : `v1.x.0` pour nouvelles features (depuis `develop`)
- [ ] Dependabot active pour les mises a jour de securite (Cargo + npm)

**Livrable :** v1.0.0 publiee, pipeline de release automatique, canal de feedback actif.

---

### Recapitulatif des versions

| Phase | Version | Focus | Critere de succes |
|-------|---------|-------|-------------------|
| 1 | v0.1.x | CI/CD + tests + lint | Pipeline verte, tests passent |
| 2 | v0.2.0 | Core UI complet | Toutes les vues fonctionnelles |
| 3 | v0.3.0 | Modes de pratique | 3 modes jouables avec scoring |
| 4 | v0.4.0 | Features avancees | i18n, Genius, raccourcis |
| 5 | v0.9.0 | Polish & packaging | Binaires multi-plateforme |
| 6 | v1.0.0 | Release publique | Distribution + auto-update |

### Ordre des priorites par phase

Chaque phase doit etre **complete et testee** avant de passer a la suivante. A l'interieur d'une phase, l'ordre recommande est :
1. Tests d'abord (TDD)
2. Implementation
3. Review + refactor
4. Merge dans `develop`

## Things to Watch Out For

- **PyO3 dependency:** Phonetic service requires Python 3 with pykakasi, hangul-romanize, and epitran installed. May fail if Python environment is not set up.
- **LibreTranslate:** Translation service calls an external API; needs network access and a running LibreTranslate instance.
- **DB location:** SQLite database lives in the OS app data directory (`~/.config/lyremember-app/lyremember.db` on Linux).
- **No CI/CD yet:** No automated pipelines; build and test locally.
- **Legacy Python CLI:** The `lyremember/` directory is a proof-of-concept CLI. The production app is in `lyremember-app/` + `rust-backend/`.
