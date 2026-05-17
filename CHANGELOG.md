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

### Changed
- Réorganisation de la documentation : les 15 fichiers `.md` de design sont passés de la racine à `docs/` ; seul `README.md` reste à la racine.
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
