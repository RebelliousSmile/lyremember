# Tauri Backend - Clarification

## ⚠️ Correction Importante

**Tauri NE gère PAS automatiquement SQLite et les APIs !**

**Tauri = Infrastructure/Framework**  
**VOUS écrivez le code backend (SQLite, API calls, etc.)**

---

## Qu'est-ce que Tauri FOURNIT ?

### Tauri fournit :

1. **Le wrapper natif** (fenêtre, WebView)
2. **Le système IPC** (communication JS ↔ Rust)
3. **L'infrastructure** pour écrire du code backend
4. **Des APIs système** (accès fichiers, notifications, etc.)

### Tauri NE fournit PAS :

❌ Code SQLite (vous devez l'écrire)
❌ Code API Genius (vous devez l'écrire)
❌ Logique métier (vous devez l'écrire)
❌ Gestion base de données (vous devez l'écrire)

---

## Concrètement : Qui Écrit Quoi ?

### VOUS Écrivez le Backend

```rust
// src-tauri/src/commands/songs.rs
// ↓ VOUS écrivez tout ça ↓

use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
}

// ↓ VOUS écrivez cette fonction ↓
#[tauri::command]
pub fn get_songs_from_db() -> Result<Vec<Song>, String> {
    // ↓ VOUS gérez SQLite ↓
    let conn = Connection::open("songs.db")
        .map_err(|e| e.to_string())?;
    
    // ↓ VOUS écrivez les requêtes SQL ↓
    let mut stmt = conn.prepare("SELECT id, title, artist FROM songs")
        .map_err(|e| e.to_string())?;
    
    // ↓ VOUS parsez les résultats ↓
    let songs = stmt.query_map([], |row| {
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>, _>>()
      .map_err(|e| e.to_string())?;
    
    Ok(songs)
}

// ↓ VOUS appelez l'API Genius ↓
#[tauri::command]
pub async fn search_genius(query: String) -> Result<Vec<GeniusResult>, String> {
    // ↓ VOUS gérez les requêtes HTTP ↓
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.genius.com/search")
        .query(&[("q", query)])
        .header("Authorization", "Bearer YOUR_TOKEN")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    // ↓ VOUS parsez la réponse JSON ↓
    let data: GeniusResponse = response.json()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(data.results)
}
```

### Tauri Fournit Juste l'Infrastructure

```rust
// src-tauri/src/main.rs
// ↓ Tauri setup (boilerplate) ↓

fn main() {
    tauri::Builder::default()
        // ↓ VOUS enregistrez VOS fonctions ↓
        .invoke_handler(tauri::generate_handler![
            get_songs_from_db,   // ← Votre fonction
            search_genius,       // ← Votre fonction
            create_song,         // ← Votre fonction
            // ... toutes VOS fonctions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Tauri dit juste : "Appelle ces fonctions depuis JS via `invoke()`"**

---

## Analogie avec le Web

### Backend Web Classique (Express/Node.js)

```javascript
// Express fournit l'infrastructure HTTP
const express = require('express')
const app = express()

// VOUS écrivez les routes et la logique
app.get('/songs', async (req, res) => {
    // ↓ VOUS gérez SQLite ↓
    const db = await sqlite.open('songs.db')
    const songs = await db.all('SELECT * FROM songs')
    res.json(songs)
})

// VOUS écrivez l'appel API
app.get('/genius/search', async (req, res) => {
    // ↓ VOUS faites la requête HTTP ↓
    const response = await fetch('https://api.genius.com/search')
    const data = await response.json()
    res.json(data)
})

// Express démarre le serveur (infrastructure)
app.listen(3000)
```

**Express = Infrastructure HTTP**  
**VOUS = Logique métier (SQLite, API, etc.)**

### Backend Tauri (Rust)

```rust
// Tauri fournit l'infrastructure IPC
tauri::Builder::default()
    // VOUS écrivez les commandes
    .invoke_handler(tauri::generate_handler![
        get_songs,      // ← VOUS gérez SQLite
        search_genius,  // ← VOUS faites l'appel API
    ])
    .run(...)
```

**Tauri = Infrastructure IPC**  
**VOUS = Logique métier (SQLite, API, etc.)**

---

## Schéma Complet : Qui Fait Quoi ?

```
┌─────────────────────────────────────────────────────┐
│                 FRONTEND (Vue)                      │
│                                                     │
│  Vous écrivez :                                     │
│  - App.vue                                          │
│  - SongCard.vue                                     │
│  - etc.                                             │
│                                                     │
│  Vite compile → HTML/CSS/JS                         │
└──────────────────────┬──────────────────────────────┘
                       │
                       │ invoke('get_songs')
                       │
┌──────────────────────▼──────────────────────────────┐
│              TAURI (Infrastructure)                 │
│                                                     │
│  Tauri fournit :                                    │
│  - WebView                                          │
│  - Système IPC (invoke)                             │
│  - APIs système                                     │
│                                                     │
│         ┌─────────────────────┐                     │
│         │  Votre Code Rust    │ ← VOUS écrivez !    │
│         │                     │                     │
│         │  get_songs() {      │                     │
│         │    // SQLite        │ ← VOUS gérez !      │
│         │  }                  │                     │
│         │                     │                     │
│         │  search_genius() {  │                     │
│         │    // HTTP request  │ ← VOUS appelez !    │
│         │  }                  │                     │
│         └─────────────────────┘                     │
└─────────────────────────────────────────────────────┘
```

---

## Ce que VOUS Devez Coder

### 1. Code SQLite (VOUS)

```rust
// src-tauri/src/db/sqlite.rs
use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("lyremember.db")?;
    
    // Créer les tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS songs (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            language TEXT NOT NULL,
            lyrics TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT NOT NULL,
            password_hash TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

pub fn get_all_songs(conn: &Connection) -> Result<Vec<Song>> {
    let mut stmt = conn.prepare("SELECT * FROM songs")?;
    
    let songs = stmt.query_map([], |row| {
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            language: row.get(3)?,
            lyrics: serde_json::from_str(&row.get::<_, String>(4)?).unwrap(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(songs)
}

pub fn insert_song(conn: &Connection, song: &Song) -> Result<()> {
    conn.execute(
        "INSERT INTO songs (id, title, artist, language, lyrics) VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            &song.id,
            &song.title,
            &song.artist,
            &song.language,
            &serde_json::to_string(&song.lyrics).unwrap(),
        ],
    )?;
    Ok(())
}
```

**Tauri ne fait RIEN de ça. VOUS l'écrivez !**

---

### 2. Code API Genius (VOUS)

```rust
// src-tauri/src/services/genius.rs
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GeniusSearchResponse {
    response: GeniusResponseData,
}

#[derive(Deserialize)]
struct GeniusResponseData {
    hits: Vec<GeniusHit>,
}

#[derive(Deserialize, Serialize)]
pub struct GeniusHit {
    result: GeniusSong,
}

#[derive(Deserialize, Serialize)]
pub struct GeniusSong {
    pub id: u64,
    pub title: String,
    pub primary_artist: GeniusArtist,
}

#[derive(Deserialize, Serialize)]
pub struct GeniusArtist {
    pub name: String,
}

pub async fn search_genius(query: String, token: String) -> Result<Vec<GeniusSong>, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.genius.com/search")
        .query(&[("q", query)])
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !response.status().is_success() {
        return Err(format!("Genius API error: {}", response.status()));
    }
    
    let data: GeniusSearchResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;
    
    let songs = data.response.hits
        .into_iter()
        .map(|hit| hit.result)
        .collect();
    
    Ok(songs)
}

pub async fn get_lyrics(song_id: u64, token: String) -> Result<String, String> {
    // ... VOUS implémentez le scraping ou l'appel API
    todo!("Implémenter récupération lyrics")
}
```

**Tauri ne fait RIEN de ça. VOUS l'écrivez !**

---

### 3. Code Commandes Tauri (VOUS)

```rust
// src-tauri/src/commands/mod.rs
use crate::db::sqlite;
use crate::services::genius;

#[tauri::command]
pub async fn get_songs() -> Result<Vec<Song>, String> {
    let conn = sqlite::init_db()
        .map_err(|e| e.to_string())?;
    
    sqlite::get_all_songs(&conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_song(song: Song) -> Result<Song, String> {
    let conn = sqlite::init_db()
        .map_err(|e| e.to_string())?;
    
    sqlite::insert_song(&conn, &song)
        .map_err(|e| e.to_string())?;
    
    Ok(song)
}

#[tauri::command]
pub async fn search_genius_api(query: String, token: String) -> Result<Vec<GeniusSong>, String> {
    genius::search_genius(query, token).await
}
```

**VOUS écrivez toute cette logique !**

---

### 4. Enregistrement dans Tauri (Boilerplate)

```rust
// src-tauri/src/main.rs
mod commands;
mod db;
mod services;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_songs,
            commands::create_song,
            commands::search_genius_api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Tauri fournit juste `.invoke_handler()` - VOUS fournissez les fonctions !**

---

## Ce que Tauri FOURNIT (APIs Built-in)

Tauri a quelques APIs système intégrées :

```javascript
// Frontend Vue
import { 
  save, 
  open 
} from '@tauri-apps/api/dialog'

import { 
  writeTextFile, 
  readTextFile 
} from '@tauri-apps/api/fs'

import { 
  sendNotification 
} from '@tauri-apps/api/notification'

// Ouvrir dialogue fichier (fourni par Tauri)
const file = await open()

// Lire fichier (fourni par Tauri)
const content = await readTextFile(file)

// Notification (fourni par Tauri)
await sendNotification('Titre', 'Message')
```

**Mais SQLite, API calls, logique métier = VOUS !**

---

## Résumé : Division des Responsabilités

### Tauri (Framework)
✅ Créer fenêtre native
✅ Intégrer WebView
✅ Système IPC (invoke)
✅ APIs système (fichiers, dialogues, notifications)
✅ Build natif (compile en .exe/.app)

### VOUS (Développeur Backend)
✅ Gérer SQLite (connexion, requêtes, migrations)
✅ Appeler API Genius (HTTP requests, parsing JSON)
✅ Implémenter logique métier (auth, validation, etc.)
✅ Générer phonétique (appeler libs ou Python)
✅ Traduire (appeler services de traduction)
✅ Sauvegarder progression
✅ Gérer sessions practice
✅ Tout le reste !

---

## Analogie Finale

### Express (Web)
```
Express dit : "Je gère HTTP"
VOUS dites : "Je gère SQLite, APIs, logique"
```

### Tauri (Desktop)
```
Tauri dit : "Je gère WebView et IPC"
VOUS dites : "Je gère SQLite, APIs, logique"
```

---

## Ce que Ça Signifie pour Votre Projet

### Vous devez apprendre/coder :

1. **Rust** (bases)
   - Syntax
   - Ownership
   - Error handling (Result<T, E>)
   - Async/await

2. **rusqlite** (SQLite en Rust)
   - Connexion DB
   - Requêtes SQL
   - Mapping rows → structs

3. **reqwest** (HTTP client Rust)
   - GET/POST requests
   - Headers (Authorization)
   - Parsing JSON

4. **serde** (Sérialisation Rust)
   - Serialize/Deserialize
   - JSON <-> Rust structs

5. **Logique métier**
   - Authentification
   - CRUD songs
   - Practice sessions
   - Progress tracking

### Tauri s'occupe de :
- ✅ Afficher votre Vue app
- ✅ Communiquer JS ↔ Rust
- ✅ Build natif

---

## Bon ou Mauvais ?

### Avantage
✅ **Flexibilité totale** - Vous codez exactement ce dont vous avez besoin
✅ **Pas de bloatware** - Pas de code inutile
✅ **Contrôle** - Vous savez ce qui se passe

### Défi
❌ **Plus de code à écrire** qu'avec un framework full-stack (Django, Rails)
❌ **Apprendre Rust** nécessaire
❌ **Gérer erreurs** Rust (strict mais sûr)

---

## Donc : Tauri = Backend ?

### ❌ Non, Tauri ≠ Backend
**Tauri = Infrastructure pour VOTRE backend**

### ✅ Oui, vous écrivez un backend en Rust
**Backend = Votre code Rust (SQLite, API calls, logique)**

### 🎯 Tauri facilite :
- Communication JS ↔ Rust
- Build natif
- APIs système

### 👨‍💻 VOUS implémentez :
- SQLite
- API Genius
- Phonétique
- Traduction
- Toute la logique

---

## C'est Plus Clair ?

**Tauri ne fait PAS la magie.**

**Tauri donne les outils, VOUS construisez le backend.**

**Mais c'est une bonne chose ! Vous avez le contrôle total.** 💪
