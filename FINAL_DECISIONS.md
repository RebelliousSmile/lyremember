# LyRemember - Décisions Finales et Résumé

## 📋 Vue d'Ensemble

**Projet :** Application desktop + mobile pour mémoriser paroles de chansons en plusieurs langues

**Langues supportées :** Français, Anglais, Coréen, Japonais

**Utilisateur cible :** Usage personnel (gratuit)

---

## ✅ Décisions Technologiques Validées

### Stack Technique

**Frontend :**
- Framework : Vue 3 (Composition API)
- Language : TypeScript
- Build Tool : Vite
- UI Library : Shadcn-vue
- Styling : Tailwind CSS
- Icons : Lucide Vue
- State : Pinia
- Router : Vue Router
- i18n : Vue I18n

**Backend :**
- Framework : Tauri (Rust)
- Language : Rust
- Database : SQLite (rusqlite)
- HTTP Client : reqwest
- Serialization : serde + serde_json
- Auth : bcrypt + jsonwebtoken
- Python Bridge : PyO3

**Desktop :**
- Tauri (Windows, macOS, Linux)
- Apps natives (3-5 MB)

**Mobile :**
- PWA (Progressive Web App) - stable
- Alternative : Tauri Mobile (beta)

---

## 🌐 Stratégie Traduction

### Principe : "Translate Once, Use Forever"

**Service choisi : LibreTranslate**
- API publique gratuite
- Open-source
- Auto-hébergeable
- Support FR/EN/JP/KR

**Workflow :**
1. Utilisateur ajoute chanson en langue étrangère
2. App appelle LibreTranslate API (une fois)
3. Traduction stockée dans SQLite (JSON field)
4. Usage offline illimité (lecture depuis DB)

**Alternative : DeepL API**
- Meilleure qualité
- 500k caractères/mois gratuit
- Nécessite inscription

**Stockage :**
```sql
CREATE TABLE songs (
    ...
    translations TEXT,        -- JSON: {"en": [...], "fr": [...]}
    translation_date TEXT
);
```

---

## 🔤 Stratégie Phonétique

### Principe : "Generate Once, Use Forever"

**Solution : PyO3 (Rust appelle Python)**

**Bibliothèques Python :**
- 🇯🇵 **pykakasi** : Kanji → Romaji
- 🇰🇷 **hangul-romanize** : Hangul → Latin
- 🇫🇷🇬🇧 **epitran** : Texte → IPA

**Workflow :**
1. Utilisateur ajoute chanson
2. App génère phonétique via PyO3 (une fois)
3. Phonétique stockée dans SQLite
4. Usage offline illimité

**Stockage :**
```sql
CREATE TABLE songs (
    ...
    phonetic_lyrics TEXT,     -- JSON: ["Romaji line 1", ...]
    phonetic_date TEXT
);
```

**Alternative rejetée :** Rust pur (pas de libs matures pour kanji/hangul)

---

## 📊 Architecture Finale

```
┌────────────────────────────────────────┐
│  Frontend Vue                          │
│  - Pages (Login, Songs, Practice)      │
│  - Components (SongCard, Karaoke)      │
│  - Shadcn-vue UI                       │
└──────────────┬─────────────────────────┘
               │ IPC (invoke)
┌──────────────▼─────────────────────────┐
│  Backend Rust (Tauri)                  │
│  ┌──────────────────────────────────┐  │
│  │  Commands                        │  │
│  │  - auth, songs, practice         │  │
│  └──────────────────────────────────┘  │
│  ┌──────────────────────────────────┐  │
│  │  Services                        │  │
│  │  - phonetic (PyO3)               │  │
│  │  - translation (LibreTranslate)  │  │
│  │  - genius (HTTP)                 │  │
│  └──────────────────────────────────┘  │
│  ┌──────────────────────────────────┐  │
│  │  Database (SQLite)               │  │
│  │  - users, songs, sessions        │  │
│  └──────────────────────────────────┘  │
└────────────────────────────────────────┘
```

---

## 🎯 User Stories Prioritaires (MVP)

### Must Have
1. ✅ Créer compte utilisateur
2. ✅ Ajouter chanson (titre, artiste, paroles)
3. ✅ Générer traduction EN automatique (si VO ≠ EN)
4. ✅ Générer phonétique (JP/KR → Latin, FR/EN → IPA)
5. ✅ Afficher chanson avec 3 vues : VO + Phonétique + Traduction
6. ✅ Mode défilement phrase par phrase (karaoke)
7. ✅ Mode phrases à trous (style "N'oubliez pas les paroles")
8. ✅ Sauvegarder progression

### Should Have
9. ⭐ Import depuis Genius API
10. ⭐ Mode QCM (propositions multiples)
11. ⭐ Statistiques basiques

### Could Have
12. 💡 Mode reconnaissance vocale (Web Speech API)
13. 💡 PWA installable (offline)
14. 💡 Dark mode
15. 💡 Interface multilingue (FR/EN/JP/KR)

---

## 📁 Structure Projet

```
lyremember/
├── README.md                        # Documentation principale
├── IMPLEMENTATION_GUIDE.md          # Ce fichier
├── lyremember/                      # Ancien code Python (garder)
└── lyremember-app/                  # Nouvelle app Tauri + Vue
    ├── src/                         # Frontend Vue
    │   ├── main.ts
    │   ├── App.vue
    │   ├── router/
    │   ├── stores/
    │   ├── views/
    │   ├── components/
    │   └── lib/
    ├── src-tauri/                   # Backend Rust
    │   ├── Cargo.toml
    │   ├── requirements.txt         # Python deps
    │   └── src/
    │       ├── main.rs
    │       ├── models/
    │       ├── db/
    │       ├── services/
    │       └── commands/
    ├── package.json
    └── vite.config.ts
```

---

## 🚀 Roadmap

### Sprint 1 (Semaine 1) - Foundation
- [x] Décisions techniques
- [x] Architecture documentée
- [ ] Setup projet Tauri + Vue
- [ ] Backend SQLite + modèles
- [ ] PyO3 setup

### Sprint 2 (Semaine 2) - Core Features
- [ ] Traduction LibreTranslate
- [ ] Phonétique PyO3
- [ ] CRUD Songs backend
- [ ] Auth backend
- [ ] Frontend login/register

### Sprint 3 (Semaine 3) - UI & Practice
- [ ] Frontend songs list/detail
- [ ] Affichage 3 colonnes (VO + Phonétique + Traduction)
- [ ] Mode karaoke
- [ ] Mode phrases à trous
- [ ] Progress tracking

### Sprint 4 (Semaine 4) - Polish
- [ ] Genius API integration
- [ ] Mode QCM
- [ ] Dark mode
- [ ] i18n
- [ ] Build production

---

## 💾 Base de Données

### Schema SQLite

```sql
-- Users
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    genius_token TEXT,
    created_at TEXT NOT NULL
);

-- Songs
CREATE TABLE songs (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    language TEXT NOT NULL,           -- 'fr', 'en', 'jp', 'kr'
    lyrics TEXT NOT NULL,             -- JSON array
    phonetic_lyrics TEXT,             -- JSON array (generated once)
    translations TEXT,                -- JSON object: {"en": [...], "fr": [...]}
    genius_id TEXT,
    genius_url TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- User Songs (Répertoire)
CREATE TABLE user_songs (
    user_id TEXT NOT NULL,
    song_id TEXT NOT NULL,
    added_at TEXT NOT NULL,
    PRIMARY KEY (user_id, song_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (song_id) REFERENCES songs(id)
);

-- Practice Sessions
CREATE TABLE practice_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    song_id TEXT NOT NULL,
    mode TEXT NOT NULL,               -- 'karaoke', 'fill-blank', 'mcq', 'oral'
    score REAL NOT NULL,
    lines_practiced INTEGER NOT NULL,
    lines_correct INTEGER NOT NULL,
    duration_seconds INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (song_id) REFERENCES songs(id)
);
```

---

## 🔧 Configuration

### Environment Variables

```bash
# .env (dev)
GENIUS_ACCESS_TOKEN=your_token_here
LIBRETRANSLATE_API_URL=https://libretranslate.com/translate
```

### Tauri Config

```json
{
  "build": {
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "LyRemember",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false
      }
    },
    "bundle": {
      "identifier": "com.lyremember.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "title": "LyRemember",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ]
  }
}
```

---

## 📚 Ressources

### Documentation
- Tauri : https://tauri.app
- Vue 3 : https://vuejs.org
- Shadcn-vue : https://www.shadcn-vue.com
- PyO3 : https://pyo3.rs
- LibreTranslate : https://libretranslate.com

### Libraries Python
- pykakasi : https://github.com/miurahr/pykakasi
- hangul-romanize : https://github.com/youknowone/hangul-romanize
- epitran : https://github.com/dmort27/epitran

---

## ✅ Checklist de Validation

### Fonctionnalités
- [ ] Utilisateur peut créer compte
- [ ] Utilisateur peut ajouter chanson
- [ ] Traduction EN générée automatiquement
- [ ] Phonétique générée automatiquement
- [ ] Affichage 3 colonnes fonctionne
- [ ] Mode karaoke fonctionne
- [ ] Mode phrases à trous fonctionne
- [ ] Progression sauvegardée
- [ ] Fonctionne offline après setup initial

### Performance
- [ ] App démarre en < 2 secondes
- [ ] Interface réactive (< 100ms)
- [ ] Génération phonétique < 5 secondes
- [ ] Génération traduction < 10 secondes

### Qualité
- [ ] Code Rust compile sans warnings
- [ ] Code Vue lint propre
- [ ] Tests unitaires backend passent
- [ ] Tests E2E passent

### Distribution
- [ ] Build Windows fonctionne
- [ ] Build macOS fonctionne
- [ ] Build Linux fonctionne
- [ ] PWA fonctionne sur mobile

---

## 🎉 État Actuel

**Phase :** Planning & Architecture ✅ COMPLETE

**Prochaine étape :** Créer projet Tauri + Vue

**Commande à exécuter :**
```bash
npm create tauri-app@latest
```

---

Dernière mise à jour : 2026-02-17
