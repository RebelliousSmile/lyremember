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
