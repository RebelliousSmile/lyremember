# Contributing to LyRemember

Merci de votre intérêt ! Ce document explique comment proposer une contribution.

## Avant de commencer

- Lire le [README](README.md) et l'index docs ([docs/INDEX.md](docs/INDEX.md)).
- Vérifier les [issues ouvertes](https://github.com/RebelliousSmile/lyremember/issues) et la roadmap dans [docs/USER_STORIES_V2.md](docs/USER_STORIES_V2.md).
- Pour une feature non triviale, ouvrir une issue de discussion avant d'écrire du code.

## Stack

Trois bases de code coexistent dans le repo — choisir la bonne selon le sujet :

| Dossier | Stack | Domaine |
|---|---|---|
| `lyremember/` + `tests/` | Python 3.8+ | CLI / proof of concept |
| `rust-backend/` | Rust + SQLite + PyO3 | Backend Tauri |
| `lyremember-app/` | Vue 3 + TypeScript + Tauri | Desktop / Mobile UI |

## Workflow

1. Forker le repo et créer une branche depuis `main` : `feat/<sujet>`, `fix/<sujet>`, `docs/<sujet>`, `refactor/<sujet>`.
2. Écrire les tests **avant** le code (TDD red → green → refactor).
3. Lancer la suite de tests locale (voir ci-dessous) et s'assurer qu'elle est verte.
4. Commit en [Conventional Commits](https://www.conventionalcommits.org/) : `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`. Inclure `(#NN)` si une issue est liée.
5. Pousser la branche et ouvrir une Pull Request claire : contexte, changements, sortie observable, tests joués.

## Lancer les tests

```bash
# Python CLI
pip install -r requirements.txt pytest
python -m pytest tests/ -q

# Rust backend
cd rust-backend && cargo test

# Frontend Vue + Tauri
cd lyremember-app && npm install && npm run test
```

## Style & conventions

- **Python** : respecter les patterns existants (Click, pyyaml, type hints quand utile). Pas d'`except Exception` nu.
- **Rust** : `cargo fmt`, `cargo clippy --all-targets -- -D warnings` avant push.
- **TypeScript/Vue** : ESLint et conventions Vue 3 `<script setup>`.
- **Markdown** : pas de duplication entre docs ; un sujet, un fichier canonique, on référence depuis les autres.
- **Pas de secrets** : jamais de clé API en clair ; utiliser les variables d'environnement / store Tauri.

## Code review

Toute PR passe en revue. Critères :
- Tests qui décrivent le comportement (pas le type).
- DRY : pas de logique dupliquée 3+ fois.
- Logs sur les chemins de production critiques.
- Aucune référence à des chemins, URLs ou secrets hard-codés.
- Documentation mise à jour si la PR touche une convention publique.

## Reporting de bug

Ouvrir une issue avec : environnement (OS, version Node/Rust/Python), étapes de reproduction, comportement observé vs attendu, logs.

## Licence

En contribuant, vous acceptez que votre travail soit publié sous la même licence que le projet ([MIT](LICENSE)).
