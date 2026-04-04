# Master Development Plan — LyRemember

> Les phases sont des jalons logiques, pas des portes sequentielles rigides.
> Les taches sans dependance dure peuvent avancer en parallele.

---

## Phase 0 — Assainissement (v0.1.1) ✅

**Objectif :** Corriger la dette technique et les bloqueurs avant toute nouvelle feature.

### 0A. Migration PyO3 → Pure Rust
- [x] Rendre PyO3 optionnel dans `rust-backend/Cargo.toml` (feature flag `python-phonetics`)
- [x] Implementer fallback pure-Rust dans `phonetic.rs` :
  - **JP** : `wana_kana` (kana → romaji, kanji passed through)
  - **KR** : table de correspondance jamo → romanisation (~100 lignes Rust)
- [x] Compiler et tester sans feature `python-phonetics`

### 0B. Bugs critiques
- [x] **`get_user_sessions` signature mismatch** — ajoute `limit: Option<i32>` avec `LIMIT ?2` parametre bind
- [x] **Blocking HTTP dans async** — wrap `translate_text` et `generate_phonetic` dans `spawn_blocking`
- [x] **LibreTranslate URL hardcodee** — extrait dans `config.rs` avec env var `LYREMEMBER_LIBRETRANSLATE_URL`
- [x] **App identifier placeholder** — remplace par `com.lyremember.app`

### 0C. Migrations DB
- [x] Systeme de migration SQLite simple (table `schema_version`, fonctions `migrate_to_vN`)

### 0D. Gestion d'erreur LibreTranslate
- [x] `try/catch` propre : si API indisponible, message clair, ne pas bloquer la creation de chanson

### Corrections supplementaires (review)
- [x] JWT secret → env var `LYREMEMBER_JWT_SECRET`
- [x] CSP active dans `tauri.conf.json`
- [x] SQL injection LIMIT → parametre bind
- [x] Fix `LoginData` → `LoginCredentials`, `SongMastery` → `f64`, `RegisterData.genius_token` retire
- [x] `UserStats` derive `Serialize`
- [x] DRY : `parse_song_row()` + `serialize_song_json()` helpers
- [x] `CreateSongResult { song, warnings }` au lieu de `eprintln!` silencieux

**Livrable :** Backend sans dependance Python, bugs corriges, app qui fonctionne offline (sauf traduction).

---

## Phase 1 — CI minimal + Tests (v0.1.x) ✅

**Objectif :** Filet de securite minimum.

### 1A. Pipeline CI legere (GitHub Actions)
- [x] Workflow `ci.yml` sur push/PR : cargo test + clippy + vue-tsc + vitest
- [x] Cache `actions/cache` pour `target/` et `node_modules/`

### 1B. Tests de base
- [x] Tests unitaires Rust : 14 tests (auth, songs, practice, phonetic, translation, db)
- [x] Tests d'integration Rust : 2 tests (full flow + migration idempotent)
- [x] Vitest + `@vue/test-utils` + jsdom installe et configure
- [x] Tests unitaires Pinia stores : auth (8), songs (10), ui (8)

**Livrable :** 16 Rust + 26 frontend = 42 tests, tous passent.

---

## Phase 2 — Core UI (v0.2.0) + Alpha interne ✅

**Objectif :** Interface fonctionnelle, premier feedback utilisateur.

### 2A. Vues principales
- [x] **LoginView / RegisterView** : formulaires complets, validation, gestion erreurs
- [x] **DashboardView** : stats reelles via `getUserStats`, chansons recentes
- [x] **SongsView** : liste avec recherche/filtre par langue/artiste
- [x] **AddSongView** : formulaire avec `CreateSongResult` + warnings via toast
- [x] **SongDetailView** : affichage lyrics + phonetique + traductions cote a cote
- [x] **ProfileView** : stats reelles + `formatTime` pour le temps de pratique

### 2B. Composants UI
- [x] Design system Tailwind : couleurs, typographie, espacements coherents
- [x] Toast/Notification composant + `useToast` composable global
- [x] `StatsCard` composant reutilisable + `useUserStats` composable partage
- [x] Layout adaptatif desktop : sidebar collapsible, largeur min 800px
- [x] Dark mode fonctionnel (toggle `ui.ts` + `localStorage` + detection OS)

### 2C. Tests frontend
- [x] Tests Vitest pour les stores (auth, songs, ui) + composables (useToast)
- [x] Vitest dans le CI

### 2D. Alignement frontend/backend
- [x] `tauri-api.ts` aligne : `CreateSongResult`, `verifyToken→string`, `getSongMastery→f64`
- [x] Fix `cmd_verify_token` return type `Result<String>` (etait `Result<User>`)
- [x] Auth store : `verifyToken` → userId → `getUser(userId)`

### 2E. ALPHA INTERNE
- [ ] Distribuer un build Tauri non-signe a 2-3 testeurs
- [ ] Collecter feedback

**Livrable :** 16 Rust + 31 frontend = 47 tests, tous passent.

---

## Phase 3 — Practice Modes (v0.3.0) + Beta fermee ✅

**Objectif :** Modes d'entrainement interactifs — coeur de la valeur produit.

### 3A. Mode Karaoke
- [x] Affichage progressif des lyrics (ligne par ligne, reveal mechanic)
- [x] Toggle phonetique en overlay
- [x] Progression visuelle (barre de progression + score temps reel)
- [x] 3 lignes precedentes affichees en contexte

### 3B. Mode Fill-in-the-Blank
- [x] Selection deterministe de mots a masquer (seed base sur le numero de ligne)
- [x] Input interactif avec validation case-insensitive
- [x] Score et feedback par ligne (vert/rouge)
- [x] Auto-pass pour les lignes d'un seul mot

### 3C. Mode QCM (Multiple Choice)
- [x] Generation de distracteurs uniques (Set) depuis les autres lignes de la chanson
- [x] Fallback word-rotation pour les chansons courtes (<4 lignes)
- [x] Interface cartes avec feedback immediat (vert/rouge)
- [x] Position deterministe de la bonne reponse

### 3D. Infrastructure de pratique
- [x] `usePracticeStore` : state machine (start → answer → finish → save → reset)
- [x] `PracticeView` : routeur de modes, barre de progression, quit confirmation
- [x] `PracticeResult` : score, message, retry/back
- [x] Routes `/practice/:songId/:mode` (karaoke, fill-blank, mcq)
- [x] `MODE_LABELS` centralise dans le store, `PracticeMode` type unique
- [x] Save session au backend via `createPracticeSession` sur unmount

### 3E. Tests
- [x] 8 tests practice store : start, answer, score, finish, reset, answer history
- [x] Typecheck complet passe

### 3F. Statistiques et retention
- [ ] Vue `PracticeStatsView` : graphiques de progression
- [ ] Historique des sessions par chanson
- [ ] Systeme de streaks / objectifs quotidiens

### 3G. BETA FERMEE
- [ ] Distribuer aux testeurs alpha + 5-10 nouveaux testeurs
- [ ] Collecter feedback

**Livrable :** 16 Rust + 39 frontend = 55 tests, tous passent. 3 modes jouables.

---

## Phase 4 — Enrichissement (v0.4.0)

**Objectif :** Features secondaires, priorisees par le feedback beta.

### 4A. Import de lyrics via LRCLIB
- [ ] Service `lyrics_search.rs` : appels REST vers LRCLIB (`GET /api/search`, `GET /api/get`)
- [ ] Bouton "Rechercher les paroles" dans AddSongView : resultats LRCLIB → pre-remplissage → validation utilisateur
- [ ] Disclaimer UI : "Paroles fournies par la communaute — verifiez avant de sauvegarder"
- [ ] Gestion d'erreur si LRCLIB est indisponible (non-bloquant)

### 4B. Internationalisation (i18n)
- [ ] Setup `vue-i18n` avec fichiers de traduction FR / EN
- [ ] Strings UI critiques externalisees (menus, boutons, messages d'erreur)
- [ ] Switcher de langue dans les settings
- [ ] JP / KR : ajouter si demande utilisateurs

### 4C. UX
- [ ] Keyboard shortcuts (navigation, play/pause, next line)
- [ ] Animations/transitions entre vues (Vue Transition)
- [ ] Systeme de notifications in-app (Toast)

### 4D. Export / Import des donnees utilisateur
- [ ] Export JSON depuis ProfileView : chansons, progression, settings
- [ ] Import JSON : restaurer les donnees sur une nouvelle machine

### 4E. Tests
- [ ] Tests i18n : verification que toutes les cles existent dans chaque locale
- [ ] Tests LRCLIB : mocks des appels HTTP
- [ ] Tests export/import : round-trip JSON valide
- [ ] Tests accessibilite (a11y) basiques

**Livrable :** Import lyrics assiste, i18n FR/EN, UX amelioree, donnees portables.

> **Hors scope v1.0 :** Mode oral (SpeechRecognition + PyAudio) → v2.0.

---

## Phase 5 — Packaging & Polish (v0.5.0)

**Objectif :** Binaires distribuables, qualite production.

### 5A. Packaging multi-plateforme
- [ ] Configuration Tauri bundle :
  - **Linux** : `.deb`, `.AppImage`
  - **macOS** : `.dmg` (+ codesigning si Apple Developer account)
  - **Windows** : `.msi`, `.exe` (NSIS)
- [ ] Icones et branding finaux
- [ ] Splash screen au demarrage

### 5B. Performance & securite
- [ ] Audit securite : CSP Tauri, sanitize inputs, pas de secrets dans le frontend
- [ ] Optimisation bundle Vite : code-splitting, lazy loading des vues
- [ ] SQLite : index sur les colonnes de recherche frequentes

### 5C. Documentation
- [ ] README utilisateur : installation, premier lancement, guide rapide
- [ ] CONTRIBUTING.md : guide de contribution
- [ ] Changelog genere automatiquement (`git-cliff`)

### 5D. Lint & formatage (maintenant pertinent)
- [ ] `rustfmt.toml` + `cargo fmt --check` en CI
- [ ] ESLint + Prettier pour Vue/TypeScript
- [ ] Pre-commit hooks via `husky` (si equipe > 1)

**Livrable :** Binaires prets, documentation complete, code propre.

---

## Phase 6 — Release & Distribution (v1.0.0)

**Objectif :** Premiere version publique stable.

### 6A. Release pipeline
- [ ] Workflow `release.yml` (GitHub Actions) :
  - Declenche sur tag `v*`
  - Build Tauri multi-plateforme (matrix 3 OS) via `tauri-apps/tauri-action`
  - Upload des binaires en GitHub Release + changelog auto
- [ ] Auto-update : `tauri-plugin-updater` pointe sur GitHub Releases API

### 6B. Distribution
- [ ] **GitHub Releases** : binaires pour les 3 plateformes
- [ ] (Optionnel futur) Homebrew tap, AUR, winget

### 6C. Post-release
- [ ] GitHub Issues templates : bug report, feature request
- [ ] Canal feedback : GitHub Discussions
- [ ] Dependabot pour mises a jour de securite (Cargo + npm)

### 6D. Cycle de maintenance
- [ ] Hotfix : `v1.0.x` pour bugs critiques
- [ ] Minor : `v1.x.0` pour nouvelles features

**Livrable :** v1.0.0 publiee, pipeline release auto, canal feedback actif.

---

## Recapitulatif

| Phase | Version | Focus | Critere de succes | Feedback |
|-------|---------|-------|-------------------|----------|
| 0 | v0.1.1 | Assainissement + PyO3 | ✅ Build sans Python, bugs corriges | — |
| 1 | v0.1.x | CI + tests | ✅ Pipeline verte, 42 tests | — |
| 2 | v0.2.0 | Core UI | ✅ Vues fonctionnelles, 47 tests | Alpha interne (a planifier) |
| 3 | v0.3.0 | Modes de pratique | ✅ 3 modes jouables, 55 tests | Beta fermee (a planifier) |
| 4 | v0.4.0 | Enrichissement | LRCLIB, i18n FR/EN, UX | Beta ouverte |
| 5 | v0.5.0 | Packaging & polish | Binaires multi-plateforme | Release candidate |
| 6 | v1.0.0 | Release publique | Distribution + auto-update | Public |

---

## Parallelisme et dependances

### Taches parallelisables (pas de dependance dure)

| Tache | Peut avancer des |
|-------|------------------|
| Dark mode | Phase 2 (c'est 10 lignes de code) |
| i18n setup | Phase 2 (structure, sans tout traduire) |
| Keyboard shortcuts | Phase 3 (quand les vues existent) |
| LRCLIB integration backend | Phase 2 (backend independant de l'UI) |
| `git-cliff` + changelog | Phase 1 (une fois les commits conventionnels adoptes) |

### Dependances dures (NE PAS paralleliser)

```
Phase 0 (PyO3 + bugs) → Phase 1 (CI ne marche pas sans ca)
Phase 1 (tests) → Phase 2 (besoin du filet de securite)
Phase 2 (UI de base) → Phase 3 (les modes de pratique s'affichent dans l'UI)
Phase 3 (modes jouables) → Phase 5 (pas de packaging sans le coeur du produit)
Phase 5 (packaging) → Phase 6 (pas de release sans binaires)
Phase 4 (enrichissement) est optionnelle avant Phase 6, priorisee par le feedback beta
```

---

## Conventions transversales

### Versioning et synchronisation

La version doit etre synchronisee dans **4 fichiers** a chaque release :
1. `lyremember-app/package.json` (`"version"`)
2. `lyremember-app/src-tauri/tauri.conf.json` (`"version"`)
3. `lyremember-app/src-tauri/Cargo.toml` (`version`)
4. `rust-backend/Cargo.toml` (`version`)

**Format :** Semantic Versioning `MAJOR.MINOR.PATCH`
**Commits :** Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`)
**Changelog :** Genere avec `git-cliff` au format [Keep a Changelog](https://keepachangelog.com)

### Branching strategy

- `main` — toujours releasable
- `develop` — branche d'integration
- `feature/*` — branches de fonctionnalite (ex: `feature/practice-karaoke`)
- `release/vX.Y.Z` — branches de release (pour hotfixes si necessaire)

### Ordre des priorites dans chaque phase

1. Tests d'abord (TDD)
2. Implementation
3. Review + refactor
4. Merge dans `develop`

### CI/CD Secrets a configurer (quand pret)

| Secret | Usage | Quand |
|--------|-------|-------|
| `APPLE_CERTIFICATE` + `APPLE_CERTIFICATE_PASSWORD` | Code signing macOS | Phase 5 |
| `APPLE_ID` + `APPLE_PASSWORD` + `APPLE_TEAM_ID` | Notarization macOS | Phase 5 |
| `WINDOWS_CERTIFICATE` | Code signing Windows (optionnel) | Phase 5 |
| `TAURI_SIGNING_PRIVATE_KEY` | Auto-updater Tauri | Phase 6 |

Pour le developpement, **skip le code signing** — les apps non-signees fonctionnent pour tester.
