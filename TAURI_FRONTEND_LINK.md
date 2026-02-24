# Comment Tauri et le Frontend Interagissent

## La Réponse Simple

**Tauri NE construit PAS les pages.**

**Votre code Vue construit les pages, comme d'habitude.**

**Tauri = Conteneur natif qui affiche votre site web Vue**

---

## Analogie Simple

Imaginez :

### Sans Tauri (Web classique)
```
┌─────────────────────────────────────┐
│      Navigateur Chrome              │
│  ┌───────────────────────────────┐  │
│  │  Votre site Vue               │  │
│  │  (HTML/CSS/JavaScript)        │  │
│  │                               │  │
│  │  construit par Vite/Webpack  │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

### Avec Tauri (Desktop)
```
┌─────────────────────────────────────┐
│      Application Desktop Tauri      │
│  ┌───────────────────────────────┐  │
│  │  WebView (navigateur intégré) │  │
│  │  ┌─────────────────────────┐  │  │
│  │  │ Votre site Vue          │  │  │
│  │  │ (HTML/CSS/JavaScript)   │  │  │
│  │  │                         │  │  │
│  │  │ construit par Vite      │  │  │
│  │  └─────────────────────────┘  │  │
│  └───────────────────────────────┘  │
│              ↕                      │
│  ┌───────────────────────────────┐  │
│  │  Backend Rust (bonus!)        │  │
│  │  (SQLite, fichiers, etc.)     │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

**Tauri = Chrome/Edge intégré dans votre app + Backend Rust**

---

## Comment Ça Marche Exactement

### Étape 1 : Vous développez normalement en Vue

```vue
<!-- App.vue -->
<script setup>
import { ref } from 'vue'

const message = ref('Hello from Vue!')
</script>

<template>
  <div>
    <h1>{{ message }}</h1>
    <button @click="message = 'Clicked!'">Click me</button>
  </div>
</template>

<style>
h1 { color: blue; }
</style>
```

### Étape 2 : Vite compile votre Vue en HTML/CSS/JS

Quand vous lancez `npm run dev` ou `npm run build` :

**Vite transforme votre Vue en :**
```html
<!-- dist/index.html -->
<!DOCTYPE html>
<html>
  <head>
    <link rel="stylesheet" href="assets/index.css">
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="assets/index.js"></script>
  </body>
</html>
```

```javascript
// dist/assets/index.js (simplifié)
const message = ref('Hello from Vue!')
// ... tout votre code Vue compilé
```

```css
/* dist/assets/index.css */
h1 { color: blue; }
```

**C'est EXACTEMENT comme un site web normal !**

### Étape 3 : Tauri affiche ce HTML/CSS/JS

```rust
// src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Ce que fait ce code Rust :**
1. Créer une fenêtre native (Windows/Mac/Linux)
2. Intégrer un WebView (mini navigateur)
3. Charger votre `index.html` dedans
4. Voilà ! Votre app Vue tourne

---

## Le WebView : Le Navigateur Caché

### Qu'est-ce qu'un WebView ?

**WebView = Navigateur sans barre d'adresse, ni onglets, ni boutons**

**Chaque OS a son WebView natif :**

| OS | WebView utilisé |
|---|---|
| **Windows** | WebView2 (basé sur Edge/Chromium) |
| **macOS** | WKWebView (basé sur Safari/WebKit) |
| **Linux** | WebKitGTK (basé sur WebKit) |

**Votre Vue app tourne DANS ce WebView, comme dans un navigateur !**

---

## Flux Complet

### En Développement (`npm run tauri dev`)

```
1. Vite démarre serveur de dev
   → http://localhost:5173
   → Votre Vue app est servie ici
   
2. Tauri ouvre une fenêtre
   → WebView charge http://localhost:5173
   → Vous voyez votre app Vue dans une fenêtre desktop
   
3. Hot Module Replacement (HMR)
   → Vous modifiez App.vue
   → Vite recharge automatiquement
   → La fenêtre Tauri se met à jour
```

### En Production (`npm run tauri build`)

```
1. Vite build votre Vue
   → Génère dist/
      ├── index.html
      ├── assets/
          ├── index.js (votre Vue compilé)
          └── index.css
   
2. Tauri compile le Rust
   → Crée l'exécutable (.exe, .app, etc.)
   
3. Tauri EMBED les fichiers dist/ dans l'exe
   → L'exe contient :
      - WebView runtime
      - Votre HTML/CSS/JS
      - Backend Rust
      
4. Résultat : Un seul fichier exécutable
   → lyremember.exe (Windows)
   → Tout est dedans !
```

---

## Communication Frontend ↔ Backend

### Frontend (Vue) appelle Backend (Rust)

```vue
<script setup>
import { invoke } from '@tauri-apps/api/tauri'

async function getSongs() {
  // Appel au backend Rust
  const songs = await invoke('get_songs')
  console.log(songs)
}
</script>

<template>
  <button @click="getSongs">Get Songs</button>
</template>
```

### Backend (Rust) répond

```rust
// src-tauri/src/main.rs
#[tauri::command]
fn get_songs() -> Vec<String> {
    vec!["Song 1".to_string(), "Song 2".to_string()]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_songs])
        .run(tauri::generate_context!())
        .expect("error");
}
```

**Comment ça communique ?**

```
Frontend Vue (JavaScript)
    │
    │ invoke('get_songs')
    │
    ▼
IPC (Inter-Process Communication)
    │
    ▼
Backend Rust
    │
    │ get_songs() → Vec<String>
    │
    ▼
IPC (retour)
    │
    ▼
Frontend Vue (JavaScript)
    │
    │ reçoit ["Song 1", "Song 2"]
```

**IPC = Message passing entre JavaScript et Rust**

---

## Comparaison avec d'Autres Technologies

### Application Web Classique
```
Frontend (Vue) 
    ↓ HTTP
Backend (Node.js/Python sur serveur)
    ↓
Base de données (serveur séparé)
```
**3 processus séparés, sur le réseau**

### Electron (Concurrent de Tauri)
```
Frontend (Vue)
    ↓ IPC
Backend (Node.js embarqué)
    ↓
Base de données (fichier local)
```
**Tout dans l'app, mais lourd (100+ MB)**

### Tauri
```
Frontend (Vue dans WebView)
    ↓ IPC
Backend (Rust embarqué)
    ↓
Base de données (fichier local)
```
**Tout dans l'app, ultra léger (5 MB)**

---

## Exemple Concret

### Votre code Vue (normal)

```vue
<!-- components/SongList.vue -->
<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const songs = ref([])

onMounted(async () => {
  // Appel Rust
  songs.value = await invoke('get_songs_from_db')
})
</script>

<template>
  <div>
    <h1>Mes Chansons</h1>
    <ul>
      <li v-for="song in songs" :key="song.id">
        {{ song.title }}
      </li>
    </ul>
  </div>
</template>
```

### Ce que Vite compile (HTML/JS)

```html
<!-- dist/index.html -->
<!DOCTYPE html>
<html>
<body>
  <div id="app"></div>
  <script src="assets/index.js"></script>
</body>
</html>
```

```javascript
// dist/assets/index.js (pseudo-code simplifié)
function SongList() {
  const songs = ref([])
  
  onMounted(async () => {
    // Ce invoke() envoie un message à Rust
    songs.value = await window.__TAURI__.invoke('get_songs_from_db')
  })
  
  // Render logic
  return html`
    <div>
      <h1>Mes Chansons</h1>
      <ul>
        ${songs.value.map(song => `<li>${song.title}</li>`)}
      </ul>
    </div>
  `
}
```

### Backend Rust qui répond

```rust
// src-tauri/src/commands/songs.rs
use rusqlite::Connection;

#[tauri::command]
fn get_songs_from_db() -> Result<Vec<Song>, String> {
    let conn = Connection::open("songs.db")
        .map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare("SELECT * FROM songs")
        .map_err(|e| e.to_string())?;
    
    let songs = stmt.query_map([], |row| {
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
        })
    }).map_err(|e| e.to_string())?
      .collect::<Result<Vec<_>, _>>()
      .map_err(|e| e.to_string())?;
    
    Ok(songs)
}
```

### Workflow complet

```
1. Utilisateur ouvre l'app
   → Tauri lance WebView
   → WebView charge index.html
   
2. Vue s'initialise
   → onMounted() s'exécute
   → invoke('get_songs_from_db')
   
3. Message envoyé à Rust via IPC
   → Rust reçoit 'get_songs_from_db'
   → Exécute la fonction get_songs_from_db()
   → Lit SQLite
   → Retourne Vec<Song>
   
4. Vue reçoit la réponse
   → songs.value = [...]
   → Vue re-render
   → Liste affichée
```

---

## Donc, Qui Fait Quoi ?

### Vue + Vite
✅ Construit vos pages (HTML/CSS/JS)
✅ Gère la réactivité
✅ Affiche l'interface
✅ Gère le routing, state, etc.

**Vite compile Vue → HTML/CSS/JS standard**

### Tauri
✅ Crée la fenêtre native
✅ Intègre le WebView (mini navigateur)
✅ Charge votre HTML/CSS/JS dedans
✅ Fournit le backend Rust
✅ Gère la communication JS ↔ Rust (IPC)
✅ Compile tout en un seul exécutable

**Tauri = Conteneur + Backend + IPC**

---

## Avantages de Cette Architecture

### Pourquoi pas juste un site web ?
❌ Pas d'accès fichiers locaux
❌ Pas d'accès SQLite local
❌ Pas offline complet
❌ Pas d'icône sur bureau
❌ Nécessite serveur backend séparé

### Pourquoi pas Electron ?
❌ Lourd (100+ MB, embarque tout Chromium)
❌ Consomme beaucoup de RAM
❌ Backend Node.js moins performant que Rust

### Pourquoi Tauri ? ✅
✅ Léger (5 MB, utilise WebView de l'OS)
✅ Performant (Rust ultra rapide)
✅ Accès système complet (fichiers, SQLite, etc.)
✅ App native (icône, installeur, etc.)
✅ Code web standard (Vue fonctionne normalement)

---

## En Résumé

### Question : Tauri construit-il les pages ?
**Réponse : NON**

### Qui construit les pages ?
**Vite compile votre Vue en HTML/CSS/JS**

### Que fait Tauri ?
**Tauri affiche cet HTML/CSS/JS dans un WebView natif**

### Bonus ?
**Tauri ajoute un backend Rust accessible depuis JavaScript**

---

## Schéma Final

```
VOUS ÉCRIVEZ
    │
    ├─ Frontend Vue (.vue files)
    │     │
    │     ▼
    │  VITE COMPILE
    │     │
    │     ▼
    │  HTML/CSS/JS (standard web)
    │     │
    │     ▼
    │  TAURI AFFICHE dans WebView natif
    │     │
    │     ▼
    │  App Desktop (avec icône, fenêtre, etc.)
    │
    └─ Backend Rust (.rs files)
          │
          ▼
       TAURI COMPILE
          │
          ▼
       Code natif (rapide)
          │
          ▼
       Accessible depuis Vue via invoke()
```

---

## C'est Plus Clair ?

**L'important à retenir :**

1. **Vue reste Vue** (vous codez normalement)
2. **Vite compile Vue** (en HTML/CSS/JS)
3. **Tauri affiche** cet HTML/CSS/JS dans une fenêtre
4. **Tauri ajoute** un backend Rust puissant

**Tauri = Smart WebView + Backend Rust**

**Pas de magie, juste du web dans une fenêtre native !** ✨
