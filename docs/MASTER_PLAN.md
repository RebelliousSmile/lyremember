# Master Plan — LyRemember

Roadmap de développement post-MVP. Les phases 0-3 sont livrées (cf. `CHANGELOG.md`). Les phases 4-6 sont éclatées en issues GitHub trackables : chaque sous-section pointe vers l'issue correspondante.

> **Origine** : ce plan a été initialement rédigé dans `MASTER_PLAN.md` sur la branche `claude/add-claude-documentation-tgOOO`. Il a été audité contre les issues #6-#31 (synthèse 2026-05-17) et arbitré par l'équipe ; cette version `docs/MASTER_PLAN.md` est la source canonique.

---

## Phases livrées ✅

### Phase 0 — Assainissement (v0.1.1)
- Migration PyO3 → arbitrage : **PyO3 gardé actif par défaut sur desktop**, `--no-default-features` sur Android (#15). Le fallback pure-Rust initialement prévu n'a pas été réalisé.
- Bugs critiques : `get_user_sessions` LIMIT bind, `spawn_blocking` HTTP, `LYREMEMBER_LIBRETRANSLATE_URL` env var, app identifier `com.lyremember.app`.
- Migrations DB : système `schema_version` + `migrate_to_vN` en place. Migration ad-hoc `genius_url` (#17).
- Gestion erreur LibreTranslate non-bloquante.
- Revue : JWT secret env var (#16), CSP active, SQL LIMIT param, types `LoginCredentials` / `f64` mastery, `UserStats: Serialize`, helpers `parse_song_row` / `serialize_song_json`, `CreateSongResult { song, warnings }`.

### Phase 1 — CI + Tests (v0.1.x)
- Workflow `ci.yml` (cargo test + clippy + vue-tsc + vitest) + cache.
- Tests Rust : 14 unit + 2 intégration + complément phonétique avec feature `python` (#23).
- Vitest + jsdom configuré ; tests pilotes stores/composants/practice (#24). **Couverture complète stores → issue #41**.

### Phase 2 — Core UI (v0.2.0) + Alpha interne
- Vues : Login, Register, Dashboard, Songs (recherche/filtre), AddSong (`CreateSongResult` + toasts, retrait UI Genius #18, import fichier #30), SongDetail (vue 3 colonnes VO/Phonétique/Trad #21), Profile.
- Design system Tailwind, toasts + `useToast`, `StatsCard` + `useUserStats`, layout adaptatif desktop (sidebar), **dark mode + toggle SettingsView + `prefers-color-scheme` (#31)**.
- Alignement `tauri-api.ts` ↔ backend.
- Alpha interne = activité de distribution (hors livrable code).

### Phase 3 — Practice Modes (v0.3.0) + Beta fermée
- Mode Karaoke (reveal, overlay, progress, contexte).
- Mode Fill-in-the-Blank (sélection déterministe, validation).
- Mode QCM avec distracteurs déterministes — amélioration de la sélection des distracteurs (catégorie grammaticale, fréquence) (#22).
- Infrastructure : `usePracticeStore`, `PracticeView` transformé en hub sélection chanson+mode (#19), routes.
- Stats : **streaks + recommandations + badge HomeView (#29)**. **Graphiques + historique par chanson + heat-map → issue #44**.
- Beta fermée = activité de distribution (hors livrable code).

---

## Phase 4 — Enrichissement (v0.4.0)

### 4A. Import lyrics
- **Arbitrage 2026-05-17** : import fichier local `.txt/.lrc/.json` livré (#30). **LRCLIB (API communautaire) n'est PAS retenu** pour éviter dépendance externe + disclaimer juridique.

### 4B. Internationalisation
- Setup `vue-i18n` + en/fr livrés en Phase 2. Locales `ja.json` / `ko.json` + sélecteur SettingsView (#20). **Test automatique de parité des clés → issue #42**.

### 4C. UX
- **Keyboard shortcuts pratique → issue #37** (Espace / Tab / 1-4 / Esc + composable + help overlay).
- **Animations / transitions Vue → issue #38** (router fade + reveal karaoké + feedback réponse + respect `prefers-reduced-motion`).

### 4D. Export / Import profil utilisateur
- **Export + import JSON profil complet → issue #35** (backup/restore : songs + sessions + stats + settings, versionné).

### 4E. Tests
- Tests parsers import fichier (#30 — done).
- **Tests parité i18n → issue #42**.
- **Tests a11y axe-core en CI → issue #39**.

---

## Phase 5 — Packaging & Polish (v0.5.0)

### 5A. Packaging multi-plateforme
- **Bundles desktop (.deb / .AppImage / .dmg / .msi) + icônes finales + splash → issue #33**.
- Android APK : livré (#15 build cross-compilation).

### 5B. Sécurité & performance
- **Audit complet : CSP, sanitization XSS, `npm audit` / `cargo audit`, index SQLite, lazy loading vues, bundle Vite → issue #43**.
- Bases déjà couvertes : JWT env var (#16), retrait token Genius UI (#18).

### 5C. Documentation
- **`git-cliff` pour changelog auto + Conventional Commits → issue #40**.
- README utilisateur complet avec captures : **non retenu** dans l'arbitrage 2026-05-17 (README actuel suffit).
- CONTRIBUTING.md détaillé : **non retenu** (CONTRIBUTING.md minimal existant suffit).

### 5D. Lint & formatage
- **Suite complète rustfmt + ESLint + Prettier + husky pre-commit + lint-staged → issue #36**.

---

## Phase 6 — Release & Distribution (v1.0.0)

### 6A. Release pipeline
- **`release.yml` matrix 4 OS (Win/macOS/Linux/Android) via `tauri-action` + auto-update Tauri (`tauri-plugin-updater` + manifest GitHub Releases) → issue #34**.

### 6B. Distribution
- GitHub Releases avec binaires des 4 plateformes : couvert par issue #34.
- Code signing Windows/macOS : hors scope v1.0 (coût + admin).
- App Store / Microsoft Store : post-v1.0.

### 6C. Post-release
- Issue templates (bug / feature) : à créer en parallèle de la v1.0 (peut être ajouté à issue #34 ou suivante).
- GitHub Discussions : décision à prendre au moment de l'ouverture publique.
- Dependabot Cargo + npm : à activer dans les settings GitHub (action manuelle, pas de code).

### 6D. Cycle de maintenance
- Process (semestriel minor, hotfix selon besoin) — pas un livrable code.

---

## Hors scope v1.0 (post-v1.0 / v2.0)

- **Mode oral live avec micro** (issue #28 — Web Speech API ou STT serveur, capture micro Tauri desktop+Android).
- **Détection de chanson type Shazam** (spike #27 conclu NO-GO MVP, à reconsidérer post-v1).
- Sync cloud du profil utilisateur (extension de l'export/import #35).
- Customisation raccourcis clavier par utilisateur.
- Screenreader walkthrough complet (au-delà de l'audit axe-core #39).

---

## Vue d'ensemble des issues post-MVP

| Issue | Phase | Sujet |
|---|---|---|
| #33 | 5A | Bundles desktop + icônes + splash |
| #34 | 6A | release.yml multi-OS + auto-update |
| #35 | 4D | Export/Import JSON profil |
| #36 | 5D | Lint suite (rustfmt + ESLint + husky) |
| #37 | 4C | Keyboard shortcuts pratique |
| #38 | 4C | Animations / transitions Vue |
| #39 | 4E | a11y + axe-core CI |
| #40 | 5C | git-cliff changelog auto |
| #41 | 1B | Couverture complète Pinia stores |
| #42 | 4E | Test parité i18n |
| #43 | 5B | Audit sécurité + performance |
| #44 | 3F | Stats avancées (graphiques + heat-map) |

## Décisions contradictoires arbitrées

| Sujet | Décision MASTER_PLAN initial | Arbitrage final (2026-05-17) |
|---|---|---|
| PyO3 vs Pure Rust | Migrer 100% vers Rust pur | **PyO3 conservé desktop**, `--no-default-features` Android (#15) |
| Import lyrics | API LRCLIB communautaire | **Import fichier local** uniquement (#30) — LRCLIB rejeté |
| README utilisateur complet | À écrire | Non retenu (README technique actuel suffit) |
| CONTRIBUTING détaillé | À étoffer | Non retenu (minimal suffit pour MVP) |
