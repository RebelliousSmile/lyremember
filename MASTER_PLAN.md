# Master Development Plan — LyRemember

> **Principes du plan :** Les phases sont des jalons logiques, pas des portes sequentielles rigides.
> Les taches sans dependance dure peuvent avancer en parallele. Chaque phase a un **milestone de
> feedback utilisateur** pour valider la direction avant d'investir plus.

---

## Phase 0 — Assainissement (v0.1.1) ← IMMEDIATE

**Objectif :** Corriger la dette technique et les bloqueurs avant toute nouvelle feature.

### 0A. Migration PyO3 → Pure Rust (BLOQUEUR N1)

Le bridging Python via PyO3 est le **principal obstacle** de developpement ET de CI. Il doit etre
resolu maintenant, pas en Phase 5.

| Option | Description | Complexite | Recommandation |
|--------|-------------|------------|----------------|
| **A (Choisie)** | Pure-Rust : `wana_kana` (JP romaji), lookup table (KR), feature flag `python-phonetics` | Faible | Elimine la dependance Python |
| B | Bundler Python avec `pyembed`/`PyOxidizer` | Elevee | +30-50 MB, cross-compile complexe |
| C | Python en sidecar Tauri | Moyenne | Fragile, necessite Python installe |

**Actions concretes :**
- [ ] Rendre PyO3 optionnel dans `rust-backend/Cargo.toml` :
  ```toml
  [features]
  default = []
  python-phonetics = ["pyo3"]
  [dependencies]
  pyo3 = { version = "0.20", features = ["auto-initialize"], optional = true }
  ```
- [ ] Implementer fallback pure-Rust dans `phonetic.rs` : `wana_kana` (JP), lookup table (KR)
- [ ] Compiler et tester sans feature `python-phonetics` — ca doit passer

### 0B. Bugs critiques
- [ ] **`get_user_sessions` signature mismatch** — `commands.rs` accepte `limit: Option<i32>` mais `practice.rs:get_user_sessions()` ne le prend pas. Aligner les deux.
- [ ] **Blocking HTTP dans async** — `translation.rs` utilise `reqwest::blocking::Client` dans un `async fn`. Migrer vers `reqwest::Client` async ou wrapper avec `tokio::task::spawn_blocking`.
- [ ] **LibreTranslate URL hardcodee** — extraire dans un `config.rs` lisant les settings depuis le repertoire app data.
- [ ] **App identifier placeholder** — remplacer `com.runner.lyremember-app` par `com.lyremember.app`.

### 0C. Migrations DB
- [ ] Implementer un systeme de migration SQLite simple (table `schema_version`, fonctions `migrate_to_vN`).
- [ ] Necessaire des maintenant : chaque phase va ajouter/modifier des colonnes. Sans migrations, les DB existantes des testeurs alpha cassent.

### 0D. Traduction : rendre offline-resilient
- [ ] La traduction via LibreTranslate doit etre **100% optionnelle et non-bloquante**.
- [ ] Si l'API est indisponible : afficher un message clair, ne pas bloquer la creation de chanson.
- [ ] Cache agressif : les traductions sont deja stockees dans `songs.translations` — ne jamais re-traduire.

**Livrable :** Backend sans dependance Python, bugs corriges, app qui fonctionne offline (sauf traduction).

---

## Phase 1 — CI minimal + Tests (v0.1.x)

**Objectif :** Filet de securite minimum pour developper sereinement. Pas de sur-engineering.

### 1A. Pipeline CI legere (GitHub Actions)
- [ ] Un seul workflow `ci.yml` sur push/PR :
  - `cargo test --manifest-path rust-backend/Cargo.toml` (sans feature `python-phonetics`)
  - `vue-tsc --noEmit` dans `lyremember-app/`
- [ ] Cache `actions/cache` pour `target/` et `node_modules/`
- [ ] Pas de build Tauri multi-plateforme en CI pour l'instant (trop lent, pas utile a ce stade)

> **Lint/format (ESLint, Prettier, Husky, rustfmt) :** Differe. On les ajoute quand l'equipe grandit
> ou quand le code style diverge. Pour un dev solo, c'est de la friction inutile.

### 1B. Tests de base
- [ ] Tests unitaires Rust : services auth, songs, practice (avec `tempfile`)
- [ ] Tests d'integration Rust : flux register → login → create song → practice
- [ ] Installer Vitest + `@vue/test-utils` pour le frontend
- [ ] Tests unitaires des 3 Pinia stores (auth, songs, ui)

**Livrable :** Pipeline verte, `cargo test` + `vue-tsc` passent, Vitest configure.

---

## Phase 2 — Core UI (v0.2.0) + Alpha interne

**Objectif :** Interface fonctionnelle, premier feedback utilisateur.

### 2A. Vues principales
- [ ] **LoginView / RegisterView** : formulaires complets, validation, gestion erreurs
- [ ] **DashboardView** : stats reelles (wirer `getUserStats`), chansons recentes
- [ ] **SongsView** : liste avec recherche/filtre par langue/artiste
- [ ] **AddSongView** : formulaire avec preview phonetique et traduction optionnelle
- [ ] **SongDetailView** : affichage lyrics + phonetique + traductions cote a cote
- [ ] **ProfileView** : settings utilisateur, stats globales

### 2B. Composants UI
- [ ] Design system Tailwind : couleurs, typographie, espacements coherents
- [ ] Composants reusables : Modal, Toast/Notification, Loading spinner, Empty state
- [ ] Layout adaptatif desktop : sidebar collapsible, largeur min 800px
- [ ] Dark mode fonctionnel (toggle `ui.ts` + `localStorage` + detection OS)

### 2C. Tests frontend
- [ ] Tests Vitest pour les stores et composants critiques
- [ ] Ajouter `vitest run` au CI

### 2D. ALPHA INTERNE
- [ ] Distribuer un build Tauri non-signe a 2-3 testeurs
- [ ] Collecter feedback sur : navigation, ajout de chanson, lisibilite phonetique
- [ ] Valider que le flux principal fonctionne avant de construire les modes de pratique

**Livrable :** App navigable, feedback reel d'utilisateurs, direction validee.

---

## Phase 3 — Practice Modes (v0.3.0) + Beta fermee

**Objectif :** Modes d'entrainement interactifs — coeur de la valeur produit.

### 3A. Mode Karaoke
- [ ] Affichage progressif des lyrics (ligne par ligne, scroll automatique)
- [ ] Toggle phonetique / traduction en overlay
- [ ] Timer et progression visuelle

### 3B. Mode Fill-in-the-Blank
- [ ] Algorithme de selection de mots a masquer (frequence, difficulte)
- [ ] Input interactif avec validation temps-reel
- [ ] Score et feedback par ligne

### 3C. Mode QCM (Multiple Choice)
- [ ] Generation de distracteurs (mots proches, meme chanson)
- [ ] Interface cartes avec feedback immediat
- [ ] Progression adaptative (difficulte croissante)

### 3D. Statistiques de pratique
- [ ] Vue `PracticeStatsView` : graphiques de progression (chart.js ou equivalent leger)
- [ ] Historique des sessions par chanson

### 3E. Tests
- [ ] Tests unitaires : algorithmes de scoring, generation de blanks, distracteurs
- [ ] Tests d'integration : flux complet d'une session de pratique

### 3F. BETA FERMEE
- [ ] Distribuer aux testeurs alpha + 5-10 nouveaux testeurs
- [ ] Collecter feedback sur : difficulte, engagement, modes preferes
- [ ] Prioriser Phase 4 en fonction du feedback (i18n utile ? import lyrics demande ?)

**Livrable :** 3 modes jouables, tracking de progression, feedback beta reel.

---

## Phase 4 — Enrichissement (v0.4.0)

**Objectif :** Features secondaires, priorisees par le feedback beta.

### 4A. Import de lyrics (remplacement de Genius)

> **Pourquoi pas Genius :** L'API Genius ne retourne **pas** les lyrics — seulement des metadonnees
> et un lien web. Recuperer les paroles necessite du scraping HTML, ce qui viole leurs CGU et pose
> des problemes de copyright (les lyrics sont la propriete des editeurs musicaux).

**Strategie retenue : user-provided + LRCLIB en assistance**

| Source | Legalite | Usage |
|--------|----------|-------|
| **Saisie manuelle** (existant) | Aucun risque | Modele principal — l'utilisateur colle ses paroles |
| **LRCLIB** (lrclib.net) | Zone grise (communautaire, pas de licence editeur) | Recherche optionnelle : pre-remplit le champ, l'utilisateur valide/edite |
| **Musixmatch** (si l'app grandit) | 100% legal (licencie) | Plan Creator ~$10-20/mois, snippets gratuits (30%) |

Analyse complete des alternatives :

| Service | Fournit les lyrics ? | Legalite | Prix | Couverture |
|---------|---------------------|----------|------|------------|
| **Genius API** | Non (metadonnees + URL) | Scraping = violation CGU | Gratuit | N/A |
| **LRCLIB** | Oui (plain + synced/LRC) | Zone grise (communautaire) | Gratuit, pas d'API key | Variable, meilleur en EN |
| **Musixmatch** | Snippet 30% (free) / Full (payant) | 100% legal (licencie) | $0-20+/mois | Excellent |
| **Lyrics.ovh** | Oui | Flou | Gratuit | Defunct/instable |
| **ChartLyrics** | Oui (SOAP/XML) | Flou | Gratuit | Vieux, uptime mauvais |
| **Saisie manuelle** | N/A | Aucun risque | Gratuit | Tout |

**Actions :**
- [ ] Service `lyrics_search.rs` : appels REST vers LRCLIB (`GET /api/search`, `GET /api/get`)
- [ ] Bouton "Rechercher les paroles" dans AddSongView : resultats LRCLIB → pre-remplissage → validation utilisateur
- [ ] Disclaimer UI : "Paroles fournies par la communaute — verifiez avant de sauvegarder"
- [ ] Aucun token API necessaire (LRCLIB est ouvert)

### 4B. Internationalisation (i18n)
- [ ] Setup `vue-i18n` avec fichiers de traduction FR / EN
- [ ] Strings UI critiques externalisees (menus, boutons, messages d'erreur)
- [ ] Switcher de langue dans les settings
- [ ] JP / KR : ajouter plus tard si demande par les utilisateurs

### 4C. UX
- [ ] Keyboard shortcuts (navigation, play/pause, next line)
- [ ] Animations/transitions entre vues (Vue Transition)
- [ ] Systeme de notifications in-app (Toast)

### 4D. Tests
- [ ] Tests i18n : verification que toutes les cles existent dans chaque locale
- [ ] Tests LRCLIB : mocks des appels HTTP
- [ ] Tests accessibilite (a11y) basiques

**Livrable :** Import lyrics assiste, i18n FR/EN, UX amelioree.

> **Hors scope v1.0 :** Mode oral (SpeechRecognition + PyAudio). C'est un projet a part entiere
> (dep Python supplementaire, gestion micro cross-platform, reconnaissance multi-langues).
> A reconsiderer en v2.0 si la demande existe.

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
| 0 | v0.1.1 | Assainissement + PyO3 | Build sans Python, bugs corriges | — |
| 1 | v0.1.x | CI + tests | Pipeline verte | — |
| 2 | v0.2.0 | Core UI | Toutes les vues fonctionnelles | Alpha interne (2-3 testeurs) |
| 3 | v0.3.0 | Modes de pratique | 3 modes jouables avec scoring | Beta fermee (10 testeurs) |
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
Phase 3 (modes jouables) → Phase 6 (pas de release sans le coeur du produit)
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
