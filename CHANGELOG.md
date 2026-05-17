# Changelog

Format basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/), versionnage suivant [SemVer](https://semver.org/lang/fr/).

## [Unreleased]

### Added
- Index documentaire `docs/INDEX.md` regroupant les 16 fichiers de design par thème.
- Mode invité : utiliser l'application sans créer de compte.
- Tests e2e WebDriverIO avec le driver Tauri.
- Suite de tests Rust complète sur le backend (77 tests).
- Workflows CI/CD pour frontend, Python et Rust (lint, build, release multi-plateforme).
- Build Android (APK debug installable) et release Windows + Android.
- Internationalisation (i18n) sur les vues principales, page Settings, intégration UI Genius API.
- Refonte UI : thème dark gold aligné sur le site, navigation mobile en bas.
- Modes de pratique : Karaoke, Fill-blank, MCQ, Oral (UI Phase 4).

### Removed
- `docs/TECH_STACK_FINAL.md` (#7) : doublait `docs/FINAL_DECISIONS.md` et décrivait une stack (PWA/React/FastAPI) abandonnée. La source canonique est désormais `docs/FINAL_DECISIONS.md`.

### Changed
- Réorganisation de la documentation : les 15 fichiers `.md` de design sont passés de la racine à `docs/` ; seul `README.md` reste à la racine.
- Corrections doc : références à `deep-translator` remplacées par `LibreTranslate` (service réellement utilisé dans `rust-backend/src/services/translation.rs`) dans `docs/TECH_CHOICES.md` et `docs/RUST_OPTION.md` (#8).
- `docs/FINAL_DECISIONS.md` aligné sur la réalité du code : UI library = composants maison `lyremember-app/src/components/ui/` (Button, Card, Input, Alert, Spinner). Note ajoutée en tête de `docs/UI_LIBRARIES.md` pour marquer ce comparatif comme historique (#9).
- Commandes Tauri : doc corrigée de 16 → 19 (5 auth + 7 songs + 4 practice + 3 util), avec liste exhaustive synchronisée sur `commands.rs`. Mentions "16" alignées dans `README.md` et `docs/TAURI_INTEGRATION_COMPLETE.md` (#10).
- `docs/ARCHITECTURE.md` clarifie en tête : ce doc décrit le POC Python (legacy, storage JSON), la prod canonique est Rust+Tauri+Vue avec storage SQLite (`rust-backend/src/db/sqlite.rs`). Référence à `legacy/python-cli/` et à `FINAL_DECISIONS.md` (#11).
- `docs/USER_STORIES_V2.md` : Epic 5 (Mode oral) descendu en CouldHave, scope MVP = self-assessment manuel, mode micro live → post-MVP avec pointeur vers #28 (#12).
- `docs/USER_STORIES_V2.md` : Epics 4-7 préfacés par une note de navigation expliquant les rôles distincts `PracticeView` (apprendre) vs `SongDetailView` (lyrics), pointeur vers #19 (#13).
- `docs/USER_STORIES_V2.md` : US-1.3 (Genius) refondue — import de lyrics interdit (ToS + API), remplacée par champ `genius_url` optionnel (#17) + spike API métadonnées (#26) + cleanup UI (#18). US-2.1 ajustée en conséquence (#14).
- Build Tauri : feature `python` (PyO3 → phonétique JP/KR/FR/EN) **activée par défaut** sur desktop dans `lyremember-app/src-tauri/Cargo.toml`. Le build Android passe `--no-default-features` (PyO3 non cross-compilable vers Android). `docs/ARCHITECTURE.md` documente le flag (#15).

### Security
- JWT signing secret n'est plus hardcoded (`rust-backend/src/services/auth.rs`). Le secret est désormais lu depuis la variable d'environnement `LYREMEMBER_JWT_SECRET`. Fallback dev : secret éphémère aléatoire (32 bytes via `uuid::Uuid::new_v4`) + warning sur stderr — les tokens deviennent invalides à chaque redémarrage. `.env.example`, `README.md`, `docs/ARCHITECTURE.md` mis à jour (#16).

### Added
- Champ optionnel `genius_url` sur les chansons : exposé via `CreateSongData` / `UpdateSongData` côté Rust, transmis par `cmd_create_song` et `cmd_update_song`, saisi dans `AddSongView` et affiché en lien sortant (`target="_blank" rel="noopener noreferrer"`) sur `SongDetailView`. La colonne SQL existait déjà — seul le wiring service/commands/UI manquait. Sémantique update : `None` = ne pas toucher, `Some("")` = clear, `Some(url)` = écrase. i18n FR/EN ajoutés. **Aucune extraction de paroles** depuis Genius (interdit par ToS) — c'est un simple lien (#17).

### Removed
- `SettingsView.vue` : section "Genius API" supprimée (token, sauvegarde localStorage, recherche, import). L'interface laissait croire à un import de paroles alors que ce n'est pas légalement faisable. Clés i18n `settings.integrations`, `settings.geniusApi`, `settings.geniusToken*`, `settings.geniusDesc`, `settings.geniusHelp`, `settings.tokenSaved`, `settings.searchSongs`, `settings.searchPlaceholder`, `settings.search`, `settings.import`, `settings.noResults` retirées de FR/EN (#18).

### Improved
- `PracticeView.vue` accepte désormais un query param `?songId=<id>` qui pré-sélectionne (déplie) la chanson dans la liste. `SongDetailView.vue` expose un raccourci "Ouvrir dans Practice →" à côté de la section Modes qui route vers `/practice?songId=...`. Confirme la séparation des rôles : Practice = hub d'apprentissage, SongDetail = gestion lyrics + raccourcis directs vers les modes (#19).
- i18n : locales `ja.json` (日本語) et `ko.json` (한국어) ajoutées ; sélecteur de langue dans Settings (4 langues). Couverture 100% des 151 clés UI. Détection automatique de `navigator.language` étendue aux 4 codes (#20).
- `SongDetailView.vue` : vue lyrics enrichie en 3 niveaux **VO + Phonétique + Traduction**. La phonétique (générée par PyO3 → pykakasi/hangul-romanize/epitran, voir #15) s'affiche en italique mono sous chaque ligne VO quand `song.phonetic_lyrics` est présent. La traduction reste en italique dorée sous la phonétique quand sélectionnée. Pas de toggle horizontal pour éviter la friction mobile ; l'ordre vertical respecte l'esprit "3 colonnes" promis (#21).
- `McqMode.vue` : génération de distractors améliorée. Les distractors sont désormais classés par proximité de longueur avec la bonne réponse (∆ characters) avant d'être mélangés — plus difficile à éliminer visuellement que des lignes aléatoires. Fallback "word-scrambled" puise dans les autres lignes plutôt que de répéter la bonne réponse. Pas de doublons garanti (#22).
- Le CLI Python (proof of concept) est archivé dans `legacy/python-cli/` (#6) : `lyremember/`, `tests/`, `data/`, `demo.py`, `setup.py`, `requirements.txt` y vivent désormais. La stack canonique est Rust + Tauri + Vue 3. Le workflow `ci-python` cible ce nouveau chemin.
- Polish `SongDetailView` : layout des paroles et états hover affinés.
- PyO3 rendu optionnel pour faciliter les builds cross-platform.
- `reqwest` basculé d'OpenSSL vers rustls pour la compatibilité Android.

### Fixed
- Lien cassé vers `IMPLEMENTATION_SUMMARY.md` dans le `README.md` (pointe désormais vers `rust-backend/`).
- Erreurs de lint TypeScript : imports et paramètres inutilisés retirés.
- Warnings clippy : closures redondantes remplacées par des références de fonction.
- Couleurs grises résiduelles dans les vues et le mode karaoké.
- Signatures de Tauri commands alignées sur l'API backend réelle.
- Compilation cross-Android des commandes Tauri.
- Installation APK via `termux-open` et création de chanson (#5).

## Notes

Ce changelog est introduit en cours de route ; l'historique complet est accessible via `git log`. Les prochaines releases déclencheront des sections versionnées dédiées.
