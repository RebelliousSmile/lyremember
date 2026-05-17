# LyRemember 🎵

Application multiplateforme pour mémoriser et pratiquer des paroles de chansons dans plusieurs langues avec support phonétique et traduction automatique.

## 🎯 Vue d'Ensemble

LyRemember est une application moderne qui combine:
- **Desktop natif** (Tauri + Vue 3)
- **Backend Rust performant** (SQLite + PyO3)
- **Support multi-langues** (FR, EN, JP, KR)
- **Phonétique automatique** (Kanji → Romaji, Hangul → Latin, etc.)
- **Traduction automatique** (LibreTranslate gratuit)
- **Practice modes** (Karaoke, Fill-blank, MCQ, Oral)

## 🚀 Quick Start

### Pour Tester l'Intégration Tauri

```bash
# Installer les dépendances Python
cd rust-backend
pip install -r requirements.txt

# Installer les dépendances npm
cd ../lyremember-app
npm install

# Lancer l'application
npm run tauri dev

# Cliquer sur "Run Integration Test" dans l'interface
```

> **Note prod** : définir `LYREMEMBER_JWT_SECRET` (voir [`.env.example`](.env.example))
> avant tout déploiement. À défaut, un secret éphémère est généré au démarrage
> et un avertissement est loggué — les JWT deviendront invalides à chaque restart.

### Architecture

```
Vue 3 Frontend (TypeScript)
    ↓
Tauri Commands (21 commands)
    ↓
Rust Backend (5 services)
    ↓
SQLite + PyO3 Python Bridge
```

## 📂 Structure du Projet

### 1. Python CLI (Proof of Concept) - LEGACY (`legacy/python-cli/`)

```bash
cd legacy/python-cli/
pip install -r requirements.txt
pip install -e .
lyremember --help
```

Le CLI Python n'est plus la stack canonique — il vit comme référence dans
[`legacy/python-cli/`](legacy/python-cli/). La stack supportée est
Rust + Tauri + Vue 3.

### 2. Rust Backend (Production) - ✅ COMPLETE

```
rust-backend/
├── src/
│   ├── services/          # Auth, Phonetic, Translation, Songs, Practice
│   ├── models/            # User, Song, PracticeSession
│   └── db/                # SQLite with auto-init
├── Cargo.toml
└── requirements.txt       # pykakasi, hangul-romanize, epitran
```

**Fonctionnalités:**
- ✅ Authentication (bcrypt + JWT)
- ✅ SQLite persistence (4 tables)
- ✅ PyO3 pour phonétique JP/KR/FR/EN
- ✅ LibreTranslate pour traduction auto
- ✅ CRUD Songs avec auto-génération
- ✅ Practice session tracking + stats

**Documentation:** [rust-backend/README.md](rust-backend/README.md)

### 3. Tauri Application (Production) - ✅ COMPLETE

```
lyremember-app/
├── src/                   # Frontend Vue 3 + TypeScript
│   ├── App.vue           # Integration test UI
│   └── lib/
│       └── tauri-api.ts  # TypeScript API (21 commands)
│
└── src-tauri/            # Backend Tauri
    ├── src/
    │   ├── commands.rs   # 21 Tauri commands
    │   └── lib.rs        # Database initialization
    └── Cargo.toml        # Depends on rust-backend
```

**Fonctionnalités:**
- ✅ 21 Tauri commands (type-safe)
- ✅ TypeScript API complète
- ✅ Integration test UI
- ✅ Database auto-created in app data dir
- ✅ Ready for Vue Router + Pinia

**Documentation:** [lyremember-app/README.md](lyremember-app/README.md)

## 📚 Documentation Complète

Toute la documentation détaillée vit dans [`docs/`](docs/) — voir [`docs/INDEX.md`](docs/INDEX.md) pour un sommaire complet.

### Planning & Décisions
- [docs/FINAL_DECISIONS.md](docs/FINAL_DECISIONS.md) - Résumé de toutes les décisions techniques
- [docs/USER_STORIES_V2.md](docs/USER_STORIES_V2.md) - 29 user stories détaillées (8 epics, 105 story points)
- [docs/TECH_CHOICES.md](docs/TECH_CHOICES.md) - Comparaison des technologies (PWA, Tauri, Flutter, etc.)

### Architecture & Technique
- [docs/TAURI_INTEGRATION_COMPLETE.md](docs/TAURI_INTEGRATION_COMPLETE.md) - 📖 **START HERE** - Guide complet intégration
- [docs/VUE_TAURI_GUIDE.md](docs/VUE_TAURI_GUIDE.md) - Guide Vue + Tauri avec exemples
- [docs/ARCHITECTURE_EXPLAINED.md](docs/ARCHITECTURE_EXPLAINED.md) - Un codebase, plusieurs plateformes
- [docs/TAURI_FRONTEND_LINK.md](docs/TAURI_FRONTEND_LINK.md) - Comment Tauri et Vue interagissent
- [docs/TAURI_BACKEND_CLARIFICATION.md](docs/TAURI_BACKEND_CLARIFICATION.md) - Rôle de Tauri vs votre code

### Stratégies
- [docs/TRANSLATION_PHONETIC_STRATEGY.md](docs/TRANSLATION_PHONETIC_STRATEGY.md) - Stratégie "Generate Once, Store Forever"
- [docs/UI_LIBRARIES.md](docs/UI_LIBRARIES.md) - Comparaison Shadcn-vue, Material, etc.
- [docs/RUST_OPTION.md](docs/RUST_OPTION.md) - Analyse Rust/Tauri vs alternatives

### Implementation
- [docs/IMPLEMENTATION_GUIDE.md](docs/IMPLEMENTATION_GUIDE.md) - Guide step-by-step création projet
- [rust-backend/IMPLEMENTATION_SUMMARY.md](rust-backend/IMPLEMENTATION_SUMMARY.md) - Résumé de l'implémentation backend

## ✨ Fonctionnalités Principales

### 1. Multi-Langues avec Phonétique
- **Japonais**: 千本桜 → senbonzakura (via pykakasi)
- **Coréen**: 한글 → hangul (via hangul-romanize)
- **Français/Anglais**: texte → IPA (via epitran)

### 2. Traduction Automatique
- Traduction EN automatique lors de l'ajout de chanson
- Stockée dans SQLite → **usage offline**
- LibreTranslate API (gratuit)

### 3. Practice Modes
- **Karaoke**: Défilement auto ligne par ligne
- **Fill-blank**: Phrases à trous (style "N'oubliez pas les paroles")
- **MCQ**: Propositions multiples
- **Oral**: Reconnaissance vocale (à venir)

### 4. Progress Tracking
- Sessions de pratique enregistrées
- Statistiques par utilisateur
- Niveau de maîtrise par chanson
- Recommandations personnalisées

## 🧪 Test d'Intégration

L'application inclut un test d'intégration complet:

```bash
cd lyremember-app
npm run tauri dev
# Click "Run Integration Test"
```

**Tests:**
1. ✅ Health check (backend connecté)
2. ✅ User registration (bcrypt + SQLite)
3. ✅ User login (JWT tokens)
4. ✅ Song creation avec phonétique JP (PyO3 + pykakasi)
5. ✅ Auto-translation EN (LibreTranslate)
6. ✅ Add to repertoire (many-to-many)
7. ✅ Practice session tracking
8. ✅ User statistics aggregation

## 🎯 Roadmap

### Phase 1: ✅ COMPLETE - Backend & Integration
- [x] Backend Rust complet (2,400 lignes)
- [x] 21 Tauri commands type-safe
- [x] TypeScript API (200 lignes)
- [x] Integration test UI
- [x] Database auto-initialization
- [x] Documentation exhaustive

### Phase 2: Core UI (2-3 jours)
- [ ] Vue Router + multi-page navigation
- [ ] Pinia stores (state management)
- [ ] Login/Register views
- [ ] Dashboard view
- [ ] Song List view

### Phase 3: Practice Modes UI (3-4 jours)
- [ ] Karaoke mode component
- [ ] Fill-blank mode component
- [ ] MCQ mode component
- [ ] Progress visualization

### Phase 4: Advanced Features (2-3 jours)
- [ ] Genius API search & import
- [ ] Dark mode toggle
- [ ] i18n (FR/EN/KR/JP interface)
- [ ] Settings page

### Phase 5: Polish (1-2 jours)
- [ ] Icons & branding
- [ ] Animations & transitions
- [ ] Keyboard shortcuts
- [ ] Error handling UI

## 💻 API Examples

```typescript
import * as api from './lib/tauri-api';

// Register & Login
const user = await api.register('username', 'email', 'password');
const token = await api.login('username', 'password');

// Create song with auto phonetic + translation
const song = await api.createSong(
  '千本桜',           // Title (Japanese)
  '初音ミク',         // Artist
  'jp',               // Language
  ['千本桜', '夜ニ紛レ'],  // Lyrics
  true                // Auto-translate to EN
);
// → phonetic_lyrics: ['senbonzakura', 'yoru ni magire']
// → translations: { en: ['Thousand Cherry Blossoms', ...] }

// Add to repertoire
await api.addToRepertoire(user.id, song.id);

// Practice!
await api.createPracticeSession(
  user.id, song.id, 'karaoke', 85.5, 10, 8, 120
);

// Get stats
const stats = await api.getUserStats(user.id);
// → { total_sessions: 1, average_score: 85.5, ... }
```

## 📊 Stack Technique

**Frontend:**
- Vue 3 (Composition API + TypeScript)
- Vite (build tool + HMR)
- Tailwind CSS (à configurer)
- Shadcn-vue (à installer)

**Desktop:**
- Tauri 2.0 (native windows)
- WebView (OS native)
- 21 Tauri commands

**Backend:**
- Rust (lyremember_backend library)
- SQLite (rusqlite) - Auto-created in app data
- PyO3 (Rust ↔ Python bridge)
- bcrypt + JWT (authentication)

**Phonetic:**
- pykakasi (Japanese)
- hangul-romanize (Korean)
- epitran (French/English IPA)

**Translation:**
- LibreTranslate API (free, 5 req/min)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT License - See LICENSE file for details

---

## Legacy Python CLI Documentation

> **Note**: Le CLI Python ci-dessous est un proof of concept archivé dans
> [`legacy/python-cli/`](legacy/python-cli/). Toutes les commandes
> `cd lyremember`, les chemins `requirements.txt`, `data/`, etc. sont
> à interpréter relativement à `legacy/python-cli/`.
> **Pour production**, utiliser l'application Tauri (`lyremember-app/`).

## Features

✨ **Multi-Language Support** - Add and practice songs in any language (English, Spanish, French, German, etc.)

🎯 **Multiple Practice Modes:**
- **Fill-in-the-Blank**: Random words are hidden; fill them in to test your memory
- **Flashcard**: See the beginning of a line and recall the rest
- **Line-by-Line**: Type each line from memory

📊 **Progress Tracking** - Track your learning with detailed statistics:
- Practice time and session count
- Accuracy and mastery levels
- Personalized recommendations

🌍 **Translation Support** - Add translations to help with language learning

💾 **Data Persistence** - All your songs and progress are saved locally

## Installation

### Prerequisites
- Python 3.8 or higher
- pip (Python package installer)

### Install from source

```bash
# Clone the repository
git clone https://github.com/RebelliousSmile/lyremember.git
cd lyremember

# Install dependencies
pip install -r requirements.txt

# Install the package
pip install -e .
```

## Quick Start

### 1. Add Your First Song

```bash
lyremember add
```

Follow the prompts to enter the song title, artist, language, and lyrics.

### 2. List Your Songs

```bash
lyremember list
```

### 3. Start Practicing

```bash
# Practice with fill-in-the-blank mode (default)
lyremember practice <song-id>

# Try different modes
lyremember practice <song-id> --mode flashcard
lyremember practice <song-id> --mode line-by-line

# Adjust difficulty (0.0 - 1.0, higher = more difficult)
lyremember practice <song-id> --difficulty 0.5
```

### 4. View Your Progress

```bash
# Overall statistics
lyremember progress

# Progress for a specific song
lyremember progress <song-id>
```

## Commands Reference

### Add a Song
```bash
lyremember add [OPTIONS]
```
Options:
- `--title TEXT`: Song title
- `--artist TEXT`: Artist name
- `--language TEXT`: Language code (e.g., en, es, fr)

### List Songs
```bash
lyremember list [OPTIONS]
```
Options:
- `--language TEXT`: Filter by language
- `--search TEXT`: Search by title or artist

### View Song Lyrics
```bash
lyremember view <song-id>
```

### Practice
```bash
lyremember practice [song-id] [OPTIONS]
```
Options:
- `--mode [fill-blank|flashcard|line-by-line]`: Practice mode (default: fill-blank)
- `--difficulty FLOAT`: Difficulty level 0-1 (default: 0.3)

If no song-id is provided, a recommended song will be selected based on your progress.

### View Progress
```bash
lyremember progress [song-id]
```

### Delete a Song
```bash
lyremember delete <song-id>
```

## Usage Examples

### Example 1: Learning Spanish Songs

```bash
# Add a Spanish song
lyremember add --title "La Cucaracha" --artist "Traditional" --language es

# List Spanish songs
lyremember list --language es

# Practice with easier difficulty
lyremember practice <song-id> --difficulty 0.2
```

### Example 2: Preparing for a Performance

```bash
# Add the song you need to perform
lyremember add

# Practice intensively with increasing difficulty
lyremember practice <song-id> --difficulty 0.3
lyremember practice <song-id> --difficulty 0.5
lyremember practice <song-id> --difficulty 0.7

# Check your mastery level
lyremember progress <song-id>
```

### Example 3: Daily Practice Routine

```bash
# Let the app recommend what to practice
lyremember practice

# View overall progress
lyremember progress
```

## Sample Songs

The repository includes sample songs in multiple languages in the `data/samples/` directory:
- `sample_en.json` - Twinkle Twinkle Little Star (English)
- `sample_es.json` - La Cucaracha (Spanish with English translation)
- `sample_fr.json` - Frère Jacques (French with English translation)

## Data Storage

All your data is stored locally in the `data/` directory:
- `songs.json` - Your song collection
- `progress.json` - Your practice history and statistics
- `config.json` - User preferences

## Project Structure

```
lyremember/
├── README.md                   # This file
├── CONTRIBUTING.md             # Contribution workflow
├── CHANGELOG.md                # Notable changes (Keep a Changelog)
├── LICENSE                     # MIT
├── docs/                       # All design & strategy docs
│   ├── INDEX.md                # Documentation index
│   ├── USER_STORIES.md
│   ├── ARCHITECTURE.md
│   └── …                       # see docs/INDEX.md
├── rust-backend/               # Backend Rust + SQLite + PyO3
├── lyremember-app/             # Frontend Vue 3 + TypeScript + Tauri
└── legacy/
    └── python-cli/             # Archived Python CLI proof of concept
        ├── lyremember/         # Application package (Python)
        ├── tests/              # pytest suite
        ├── data/samples/       # Sample songs
        ├── demo.py
        ├── setup.py
        └── requirements.txt
```

## User Stories

See [docs/USER_STORIES.md](docs/USER_STORIES.md) for detailed user stories and feature roadmap.

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for technical architecture and design decisions.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for the workflow, tooling and conventions.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for the list of notable changes.

## License

MIT License — see [LICENSE](LICENSE) for the full text.

## Roadmap

### Current Features (v0.1.0)
- [x] Add and manage songs
- [x] Multiple language support
- [x] Fill-in-the-blank practice
- [x] Flashcard practice
- [x] Line-by-line practice
- [x] Progress tracking
- [x] Basic statistics

### Planned Features
- [ ] Web interface
- [ ] Audio playback integration
- [ ] Spaced repetition algorithm
- [ ] Community song database
- [ ] Mobile app
- [ ] Advanced analytics

## Support

For issues, questions, or suggestions, please open an issue on GitHub.

## Acknowledgments

Built with love for music and language learners everywhere! 🎶🌍
