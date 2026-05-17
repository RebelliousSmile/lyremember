# Changelog

Format basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/), versionnage suivant [SemVer](https://semver.org/lang/fr/).

## [Unreleased]

### Added
- Animations de feedback dans les modes pratique (#38) :
  - Keyframes CSS `pulse-correct` (scale doux sur bonne réponse) et `shake-wrong` (translation latérale sur erreur) ajoutées à `src/styles/main.css`.
  - McqMode : pulse sur la réponse choisie correcte, shake sur la réponse choisie incorrecte.
  - FillBlankMode : pulse / shake sur chaque blank révélé selon `token.correct`.
  - `@media (prefers-reduced-motion: reduce)` global qui désactive toutes les animations + transitions + scroll smooth.
  - Transition router-view `fade` déjà en place dans `App.vue` (héritage Phase 2).
- Raccourcis clavier dans les modes pratique (#37) :
  - Karaoké : `Space` toggle play/pause, `←` `→` ligne précédente/suivante.
  - QCM : `1`-`4` sélectionne la réponse, `Enter` passe à la question suivante.
  - Pratique : `Esc` quitte le mode actif (retour à la song detail).
  - Nouveau composable `useShortcuts(bindings)` partagé, ignore les `INPUT`/`TEXTAREA`/`SELECT`.
  - Overlay help (`?`) étendu avec les nouvelles entrées, traductions en/fr/ja/ko.
- Génération automatique de release notes via `git-cliff` (#40) : `cliff.toml` à la racine + job `changelog` dans `.github/workflows/release.yml` qui exécute `orhun/git-cliff-action` et injecte les commits Conventional groupés (Added/Fixed/Changed/…) dans le body de la GitHub Release. CONTRIBUTING.md liste les préfixes acceptés.
- Suite complète lint & formatage (#36) :
  - `cargo fmt --all -- --check` ajouté à la CI Rust (rust-backend) + nouveau job `fmt-tauri` pour `lyremember-app/src-tauri`.
  - ESLint flat config (`lyremember-app/eslint.config.js`) avec `typescript-eslint` + `eslint-plugin-vue` + `eslint-config-prettier` ; globals browser/node.
  - Prettier (`.prettierrc.json` + `.prettierignore`) ; scripts `npm run lint` (check) et `npm run lint:fix`.
  - Step CI Frontend `npm run lint` (eslint + prettier --check).
  - Pre-commit hook `.husky/pre-commit` qui lance `lint-staged` (ESLint + Prettier sur fichiers stagés) ; activation : `git config core.hooksPath .husky`.
  - Config `lint-staged` dans `lyremember-app/package.json`.
- Test automatique de parité des clés i18n (`lyremember-app/src/i18n/parity.spec.ts`) : vérifie que `fr.json` / `ja.json` / `ko.json` contiennent exactement les mêmes clés que `en.json` (référence). Échec immédiat sur clé manquante ou orpheline (#42).
- Roadmap post-MVP `docs/MASTER_PLAN.md` (phases 0-3 done, phases 4-6 → issues #33-#44 avec décisions contradictoires arbitrées). Source : MASTER_PLAN.md de la branche `claude/add-claude-documentation-tgOOO` audité contre les issues #6-#31.
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
- `services/phonetic.rs` court-circuite désormais l'input vide sans démarrer le runtime Python (perf + simplicité de test).

### Docs / Spikes
- `docs/spikes/2026-05-genius-api.md` : rapport R&D sur l'API métadonnées Genius — endpoints utilisables (search, songs, artists), valeur produit limitée (auto-complétion à la création), recommandation **NO-GO court terme**. Lien sortant `genius_url` (#17) suffit pour le cas d'usage légal (#26).
- `docs/spikes/2026-05-song-detection.md` : rapport R&D sur la détection de chanson en cours d'écoute (Shazam-like) — comparatif AudD / ACRCloud / Shazam / Chromaprint+MusicBrainz, faisabilité capture audio Tauri desktop+Android, aspects légaux RGPD, coûts estimés 100/1000 utilisateurs. Recommandation **NO-GO MVP** (effort 1-2 semaines + coûts API récurrents) (#27).
- `docs/INDEX.md` : nouvelle section "Spikes R&D" référençant les deux rapports.

### Testing
- Tests `services::phonetic` étoffés (#23) : empty input passes through tous langs ; ajout de tests pour la branche stub (`#[cfg(not(feature = "python"))]`) couvrant erreur sur langs supportés + passthrough sur lang non-supportée ; ajout test FR (`fra-Latn`) ignored ; ajout dispatcher test ; documentation des prérequis Python dans le module. Suite globale : 92 passed, 6 ignored, 0 failed avec feature `python` ; 91 passed, 0 ignored, 0 failed sans (couvre les deux branches CI).
- Vitest infrastructure côté frontend (#24) : `vitest.config.ts` (jsdom + alias `@/*`), scripts npm `test:unit` (run-once) et `test:unit:watch`, devDeps `vitest` + `@vue/test-utils` + `jsdom`. Trois tests pilotes : `Button.spec.ts` (4 cas : rendering slot, click, loading, variant), `Alert.spec.ts` (3 cas : hidden, message+variant, close emit), `songs.spec.ts` (4 cas : filter search, filter lang, group by lang, count). Workflow CI Frontend exécute `npm run test:unit` entre le typecheck et le build. README + CONTRIBUTING.md mis à jour.
- Tests `ui.spec.ts` couvrant dark mode (#31) : `toggleDarkMode` flip + DOM class + localStorage, `initializeDarkMode` saved value priorité, fallback `prefers-color-scheme`.
- Mode oral **live (expérimental)** dans `OralMode.vue` (#28). Utilise la Web Speech API native du webview (Chrome/WebView2/WRY-WebKit selon plateforme), avec **fallback gracieux** sur le self-assessment manuel existant si l'API n'est pas exposée. Bouton "Speak" qui démarre l'écoute, mappe la langue de la chanson (`fr`→`fr-FR`, `jp`→`ja-JP`, `kr`→`ko-KR`, `en`→`en-US`), calcule un score via `scoreSpoken` (token-set recall, seuil 70% pour auto-validation). Module pur `lib/oral-scoring.ts` (`normalize`, `tokenize`, `scoreSpoken`, `hasSpeechRecognition`) + 12 tests Vitest. Pas de capture audio Tauri native — donc fonctionnel sur desktop si le webview expose `SpeechRecognition`, sur Android dépend de la version de WebView. Spike de #27 (capture native + STT serveur) reste pertinent pour le mode oral live définitif.
- Stats : streak quotidien + recommandations de chansons (#29). Côté Rust : `compute_streak_from_dates` (pure, testable avec NaiveDate contrôlée — gère cas vide, sessions multiples par jour dédupliquées, gap > 1 jour reset à 0, anchor today/yesterday accepté), `get_user_streak` (DB) et `get_recommendations` (chansons les moins maîtrisées, ORDER BY AVG(score) ASC, limit configurable). 2 commandes Tauri : `cmd_get_user_streak`, `cmd_get_recommendations`. Frontend : `getUserStreak` / `getRecommendations` dans tauri-api, `useUserStats` composable étendu, badge "🔥 N jours" sur `DashboardView` quand streak > 0. i18n FR/EN/JP/KR : `dashboard.streakDays` (pluriel) + `dashboard.streakTooltip`. 9 tests Rust ajoutés.
- Import de fichier `.txt` / `.json` / `.lrc` dans `AddSongView` (#30). Parsers purs dans `lib/file-parsers.ts` (sans dépendance Tauri, lit `File.text()` côté navigateur) : `parseTxt` (1 ligne = 1 lyric, ignore `#comment` et lignes vides), `parseJson` (`{title?, artist?, language?, lyrics: string[]}`), `parseLrc` (timestamps `[mm:ss.xx]` stripés, tags `[ti:]` `[ar:]` `[la:]` extraits en métadonnées). Input `<input type="file" accept=".txt,.json,.lrc">` dans le formulaire, mappe sur les champs existants. 16 tests Vitest dans `file-parsers.spec.ts`. i18n FR/EN/JP/KR mis à jour (`addSong.importFile`, `addSong.importFileHint`).
- Couverture de tests étendue côté `legacy/python-cli/` (#25) : `test_models.py` (User/Song/PracticeSession/SongProgress, round-trip + verify_password), `test_practice_engine.py` (fill-blank, flashcard, line-by-line, session accounting), `test_user_manager.py` (register, login, repertoire, genius token — via Storage mocké car la couche storage legacy n'expose pas les méthodes user), `test_genius_api.py` (init + degradation gracieuse sans token). 68 tests passent, **coverage moyenne 81% sur les 4 modules ciblés** (practice_engine 100%, models 100%, user_manager 96%, genius_api 30%).
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
