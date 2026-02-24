# LyRemember Tauri Application

Application multiplateforme pour mémoriser des paroles de chansons avec support phonétique et traduction automatique.

## 🏗️ Architecture

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust (Tauri commands)
- **Database**: SQLite (dans app data directory)
- **Phonetic**: PyO3 bridge vers Python (pykakasi, hangul-romanize, epitran)
- **Translation**: LibreTranslate API (gratuit)

## 📋 Prérequis

### Système

Pour le développement desktop, vous devez avoir installé :

**Linux:**
```bash
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Microsoft Visual Studio C++ Build Tools

### Runtime

- **Rust**: 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **Node.js**: 18+ avec npm
- **Python**: 3.8+ (pour PyO3 phonetics)

## 🚀 Installation

### 1. Installer les dépendances npm

```bash
npm install
```

### 2. Installer les dépendances Python (pour phonétique)

```bash
cd ../rust-backend
pip install -r requirements.txt
cd ../lyremember-app
```

Les packages installés :
- `pykakasi>=2.2.1` - Japonais (kanji → romaji)
- `hangul-romanize>=0.1.0` - Coréen (hangul → latin)
- `epitran>=1.24` - Français/Anglais (→ IPA)

### 3. Build le backend Rust

```bash
cd src-tauri
cargo build
cd ..
```

## 🔧 Développement

### Mode développement

```bash
npm run tauri dev
```

Cela va :
1. Compiler le backend Rust avec toutes les commandes Tauri
2. Démarrer le serveur Vite dev (Hot Module Reload)
3. Ouvrir la fenêtre Tauri avec l'application
4. Initialiser la base de données SQLite dans le répertoire app data

### Structure du projet

```
lyremember-app/
├── src/                      # Frontend Vue
│   ├── App.vue              # Test d'intégration UI
│   ├── lib/
│   │   └── tauri-api.ts     # API client TypeScript
│   └── main.ts
│
├── src-tauri/               # Backend Tauri
│   ├── src/
│   │   ├── lib.rs           # Setup Tauri + DB initialization
│   │   └── commands.rs      # Toutes les commandes Tauri
│   └── Cargo.toml           # Dépendances Rust (inclut lyremember_backend)
│
└── package.json
```

## 🧪 Test d'Intégration

L'application inclut un test d'intégration complet accessible via l'interface.

**Cliquez sur "Run Integration Test"** pour tester :

1. ✅ Health check (connexion backend)
2. ✅ User registration (bcrypt + SQLite)
3. ✅ User login (JWT tokens)
4. ✅ Song creation avec phonétique auto (PyO3 + pykakasi)
5. ✅ Song creation avec traduction auto (LibreTranslate)
6. ✅ Add song to repertoire (relation many-to-many)
7. ✅ Practice session tracking
8. ✅ User statistics aggregation

### Logs attendus

```
[14:23:45] 🚀 LyRemember - Backend Integration Test Ready
[14:23:47] 🔍 Testing health check...
[14:23:47] ✅ Health check: Backend is healthy!
[14:23:48] 👤 Testing user registration...
[14:23:48] ✅ User registered: user_1708267428123 (ID: ...)
[14:23:48] 🔐 Testing login...
[14:23:48] ✅ Login successful, token: eyJ0eXAiOiJKV1QiLCJ...
[14:23:48] 🎵 Testing song creation with phonetics...
[14:23:50] ✅ Song created: 千本桜 by 初音ミク
[14:23:50]    📝 Phonetic: senbonzakura, yoru ni magire, kimi no koe mo todoka nai yo
[14:23:50]    🌐 Translation available: en
[14:23:50] 📚 Adding song to user's repertoire...
[14:23:50] ✅ Song added to repertoire
[14:23:50] 📖 Getting user's songs...
[14:23:50] ✅ User has 1 songs
[14:23:50] 🎮 Creating practice session...
[14:23:50] ✅ Practice session created: karaoke, score: 85.5%
[14:23:50] 📊 Getting user statistics...
[14:23:50] ✅ Stats: 1 sessions, avg score: 85.5%
[14:23:50] 🎉 Integration test completed successfully!
```

## 📦 Build Production

### Desktop

```bash
npm run tauri build
```

Crée les executables dans `src-tauri/target/release/bundle/` :

- **Linux**: `.deb`, `.AppImage`
- **macOS**: `.dmg`, `.app`
- **Windows**: `.msi`, `.exe`

### Configuration

Éditer `src-tauri/tauri.conf.json` pour configurer :
- Nom de l'app
- Icônes
- Permissions
- Version

## 🗄️ Base de Données

La base de données SQLite est créée automatiquement au premier lancement.

**Emplacement :**

- **Linux**: `~/.local/share/com.runner.lyremember-app/lyremember.db`
- **macOS**: `~/Library/Application Support/com.runner.lyremember-app/lyremember.db`
- **Windows**: `%APPDATA%\com.runner.lyremember-app\lyremember.db`

**Schema :**

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    genius_token TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE songs (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    language TEXT NOT NULL,
    lyrics TEXT NOT NULL,              -- JSON array
    phonetic_lyrics TEXT,              -- JSON array (generated once)
    translations TEXT,                 -- JSON object (generated once)
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE user_songs (
    user_id TEXT,
    song_id TEXT,
    added_at TEXT NOT NULL,
    PRIMARY KEY (user_id, song_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (song_id) REFERENCES songs(id)
);

CREATE TABLE practice_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    song_id TEXT NOT NULL,
    mode TEXT NOT NULL,                -- karaoke, fill-blank, mcq, oral
    score REAL NOT NULL,
    lines_practiced INTEGER NOT NULL,
    lines_correct INTEGER NOT NULL,
    duration_seconds INTEGER NOT NULL,
    practiced_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (song_id) REFERENCES songs(id)
);
```

## 🔌 API Tauri

Toutes les fonctions backend sont exposées via l'API TypeScript dans `src/lib/tauri-api.ts`.

### Exemple d'utilisation

```typescript
import * as api from './lib/tauri-api';

// Register
const user = await api.register('username', 'email@example.com', 'password');

// Login
const token = await api.login('username', 'password');

// Create song with auto phonetic + translation
const song = await api.createSong(
  '千本桜',
  '初音ミク',
  'jp',
  ['千本桜', '夜ニ紛レ'],
  true  // auto-translate to EN
);

// song.phonetic_lyrics = ['senbonzakura', 'yoru ni magire']
// song.translations = { en: ['Thousand Cherry Blossoms', 'Lost in the night'] }

// Add to repertoire
await api.addToRepertoire(user.id, song.id);

// Create practice session
const session = await api.createPracticeSession(
  user.id,
  song.id,
  'karaoke',
  85.5,  // score
  10,    // lines practiced
  8,     // lines correct
  120    // duration in seconds
);

// Get stats
const stats = await api.getUserStats(user.id);
console.log(stats.average_score);
```

## 🐛 Dépannage

### Erreur Python packages

Si les packages Python ne sont pas installés :

```bash
pip install pykakasi hangul-romanize epitran
```

### Erreur compilation Rust

Assurez-vous d'avoir installé les dépendances système (voir Prérequis).

### Base de données verrouillée

Si la DB est verrouillée, fermez toutes les instances de l'application.

## 📝 Prochaines Étapes

- [ ] Créer les vues Vue (Login, Songs, Practice)
- [ ] Implémenter Pinia stores
- [ ] Ajouter Vue Router
- [ ] Créer les composants de practice (Karaoke, Fill-blank, MCQ)
- [ ] Ajouter Genius API integration
- [ ] Implémenter dark mode
- [ ] Ajouter i18n (FR/EN/KR/JP)

## 📄 License

MIT
