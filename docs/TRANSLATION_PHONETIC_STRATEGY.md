# Recommandations Techniques : Traduction & Phonétique

## 1. Stratégie de Traduction (Translate Once, Store Locally)

### ✅ Votre Approche Est EXCELLENTE

**Principe : "Translate Once, Use Forever"**

```
1. Utilisateur ajoute chanson en japonais
2. App traduit EN automatiquement (via API)
3. Traduction stockée dans SQLite
4. Usage offline ensuite (lecture depuis DB)
```

**Avantages :**
- ✅ **Offline** : Fonctionne sans internet après traduction initiale
- ✅ **Rapide** : Pas d'appel API à chaque affichage
- ✅ **Gratuit** : Une seule traduction par chanson (économise quota API)
- ✅ **Cohérent** : Même traduction à chaque fois
- ✅ **Éditable** : Utilisateur peut corriger traduction si besoin

---

## Architecture Traduction

### Base de Données (SQLite)

```sql
-- Table songs avec colonne translations
CREATE TABLE songs (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    language TEXT NOT NULL,  -- 'fr', 'en', 'jp', 'kr'
    lyrics TEXT NOT NULL,    -- JSON array: ["line 1", "line 2", ...]
    
    -- Traductions stockées en JSON
    translations TEXT,       -- JSON: {"en": ["line 1 EN", ...], "fr": [...]}
    
    -- Métadonnées traduction
    translated_languages TEXT,  -- JSON: ["en", "fr"]
    translation_date TEXT,      -- ISO timestamp dernière traduction
    
    created_at TEXT,
    updated_at TEXT
);
```

### Workflow Détaillé

```
┌─────────────────────────────────────────────────┐
│  Utilisateur ajoute chanson "千本桜" (JP)        │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────┐
│  1. Sauvegarder chanson avec lyrics originaux   │
│     {                                            │
│       language: "jp",                            │
│       lyrics: ["千本桜", "夜ニ紛レ", ...],       │
│       translations: {}  ← vide au début          │
│     }                                            │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────┐
│  2. Détecter que language != "en"               │
│     → Proposer traduction automatique           │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────┐
│  3. Appeler API traduction (une seule fois)     │
│     Service: Google Translate / DeepL / LibreT  │
│     Input: ["千本桜", "夜ニ紛レ", ...]            │
│     Output: ["Cherry blossoms", "At night", ...]│
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────┐
│  4. Sauvegarder traduction dans SQLite          │
│     UPDATE songs SET                             │
│       translations = '{"en": [...]}'             │
│       translation_date = NOW()                   │
│     WHERE id = ...                               │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────┐
│  5. Usage offline illimité                      │
│     SELECT lyrics, translations FROM songs       │
│     → Affichage VO + EN côte à côte             │
│     → Pas d'appel API !                         │
└─────────────────────────────────────────────────┘
```

---

## Services de Traduction Recommandés

### Option 1 : LibreTranslate (RECOMMANDÉ pour gratuit)

**Service open-source, auto-hébergeable**

```rust
// Rust backend
use reqwest;

async fn translate_with_libretranslate(
    text: Vec<String>,
    source_lang: &str,  // "ja", "ko", etc.
    target_lang: &str,  // "en"
) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    
    let mut translated = Vec::new();
    
    for line in text {
        let response = client
            .post("https://libretranslate.com/translate")  // API publique gratuite
            .json(&serde_json::json!({
                "q": line,
                "source": source_lang,
                "target": target_lang,
                "format": "text"
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        let data: serde_json::Value = response.json()
            .await
            .map_err(|e| e.to_string())?;
        
        translated.push(
            data["translatedText"]
                .as_str()
                .unwrap_or("")
                .to_string()
        );
    }
    
    Ok(translated)
}
```

**Avantages :**
- ✅ **Gratuit** (API publique : 5 req/min)
- ✅ **Open-source**
- ✅ **Auto-hébergeable** si besoin
- ✅ **Pas de compte** nécessaire
- ✅ **Bonne qualité** pour usage basique

**Limites :**
- ⚠️ Rate limit (5 req/min sur instance publique)
- ⚠️ Qualité inférieure à Google/DeepL
- ⚠️ Langues limitées (mais FR/EN/JP/KR OK)

---

### Option 2 : Google Translate (Unofficial via googletrans-rs)

```rust
// Utiliser crate googletrans-rs (scraping, pas officiel)
// Gratuit mais peut casser

use googletrans::{Translator, Lang};

async fn translate_with_google_unofficial(
    text: Vec<String>,
    target_lang: Lang,
) -> Result<Vec<String>, String> {
    let translator = Translator::new();
    
    let mut translated = Vec::new();
    
    for line in text {
        let result = translator
            .translate(&line, Lang::Auto, target_lang)
            .await
            .map_err(|e| e.to_string())?;
        
        translated.push(result.text);
    }
    
    Ok(translated)
}
```

**Avantages :**
- ✅ **Gratuit** (scraping)
- ✅ **Qualité Google Translate**
- ✅ **Pas de clé API**

**Inconvénients :**
- ❌ **Non officiel** (peut être bloqué)
- ❌ **Peut casser** à tout moment
- ❌ **Rate limiting** agressif

---

### Option 3 : DeepL API (Meilleure qualité, limité gratuit)

```rust
use reqwest;

async fn translate_with_deepl(
    text: Vec<String>,
    source_lang: &str,
    target_lang: &str,
    api_key: &str,
) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    
    // DeepL supporte batch translation
    let response = client
        .post("https://api-free.deepl.com/v2/translate")
        .header("Authorization", format!("DeepL-Auth-Key {}", api_key))
        .json(&serde_json::json!({
            "text": text,
            "source_lang": source_lang.to_uppercase(),
            "target_lang": target_lang.to_uppercase(),
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let data: serde_json::Value = response.json()
        .await
        .map_err(|e| e.to_string())?;
    
    let translated: Vec<String> = data["translations"]
        .as_array()
        .ok_or("Invalid response")?
        .iter()
        .map(|t| t["text"].as_str().unwrap_or("").to_string())
        .collect();
    
    Ok(translated)
}
```

**Avantages :**
- ✅ **Meilleure qualité** de traduction
- ✅ **Plan gratuit** : 500k caractères/mois
- ✅ **Officiel et stable**
- ✅ **Batch translation** (plusieurs lignes en une requête)

**Inconvénients :**
- ⚠️ **Nécessite API key** (inscription)
- ⚠️ **Quota limité** (500k caractères gratuits)
- ⚠️ **Payant** après quota (5€/mois pour 500k supplémentaires)

---

### Recommandation Traduction

**Pour MVP (gratuit, usage personnel) :**
```
1. LibreTranslate (API publique)
   - Gratuit illimité (5 req/min)
   - Bonne qualité
   - Auto-hébergeable si besoin
   
2. Fallback : Traduction manuelle
   - Utilisateur peut ajouter traduction manuellement
   - Champ éditable dans l'interface
```

**Pour usage intensif :**
```
DeepL API (500k caractères/mois gratuit)
- Inscription simple
- Excellente qualité
- Batch translation efficace
```

---

## 2. Solution Phonétique Recommandée

### Stratégie : Hybride (Bibliothèques + FFI Python)

### Langues à Supporter

**Niveau 1 (Prioritaire) :**
- 🇯🇵 **Japonais** → Romaji (Latin)
- 🇰🇷 **Coréen** → Romanization (Latin)

**Niveau 2 (Bonus) :**
- 🇫🇷 **Français** → IPA ou phonétique simplifiée
- 🇬🇧 **Anglais** → IPA ou phonétique simplifiée

---

### Solution Recommandée : PyO3 (Appeler Python depuis Rust)

**Pourquoi ?**
- Excellentes libs Python pour phonétique (pykakasi, hangul-romanize)
- Pas d'équivalent mature en Rust
- PyO3 permet d'appeler Python depuis Rust
- Facile à maintenir

### Architecture

```
┌──────────────────────────────────────┐
│  Frontend Vue (JavaScript)           │
└───────────────┬──────────────────────┘
                │ invoke('generate_phonetic')
┌───────────────▼──────────────────────┐
│  Backend Rust (Tauri)                │
│                                      │
│  #[tauri::command]                   │
│  fn generate_phonetic(               │
│      text: Vec<String>,              │
│      language: String                │
│  ) -> Vec<String> {                  │
│      match language.as_str() {       │
│          "jp" => call_pykakasi(text),│ ← PyO3
│          "kr" => call_hangul(text),  │ ← PyO3
│          "fr" => call_epitran(text), │ ← PyO3
│          "en" => call_epitran(text), │ ← PyO3
│          _ => text                   │
│      }                                │
│  }                                   │
│                                      │
│  ┌────────────────────────────────┐ │
│  │  PyO3 Bridge                   │ │
│  │  (Rust appelle Python)         │ │
│  │                                │ │
│  │  ┌──────────────────────────┐ │ │
│  │  │  Python Environment      │ │ │
│  │  │  - pykakasi (JP)         │ │ │
│  │  │  - hangul-romanize (KR)  │ │ │
│  │  │  - epitran (FR/EN)       │ │ │
│  │  └──────────────────────────┘ │ │
│  └────────────────────────────────┘ │
└──────────────────────────────────────┘
```

---

### Implémentation PyO3

#### 1. Setup PyO3 dans Cargo.toml

```toml
# src-tauri/Cargo.toml
[dependencies]
pyo3 = { version = "0.20", features = ["auto-initialize"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### 2. Code Rust avec PyO3

```rust
// src-tauri/src/phonetic.rs
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Convertir japonais (kanji) → romaji
pub fn japanese_to_romaji(text: Vec<String>) -> Result<Vec<String>, String> {
    Python::with_gil(|py| {
        // Importer pykakasi
        let kakasi_module = py.import("pykakasi")
            .map_err(|e| format!("Failed to import pykakasi: {}", e))?;
        
        // Créer instance kakasi
        let kakasi_class = kakasi_module.getattr("kakasi")
            .map_err(|e| e.to_string())?;
        let kakasi = kakasi_class.call0()
            .map_err(|e| e.to_string())?;
        
        let mut result = Vec::new();
        
        for line in text {
            // Convertir chaque ligne
            let converted = kakasi
                .call_method1("convert", (line,))
                .map_err(|e| e.to_string())?;
            
            // Extraire romaji de chaque segment
            let py_list: &PyList = converted.downcast()
                .map_err(|e| e.to_string())?;
            
            let mut romaji_line = String::new();
            for item in py_list {
                let dict = item.downcast::<pyo3::types::PyDict>()
                    .map_err(|e| e.to_string())?;
                let romaji = dict.get_item("hepburn")
                    .ok_or("No hepburn key")?
                    .ok_or("hepburn is None")?
                    .extract::<String>()
                    .map_err(|e| e.to_string())?;
                romaji_line.push_str(&romaji);
            }
            
            result.push(romaji_line);
        }
        
        Ok(result)
    })
}

/// Convertir coréen (hangul) → romanization
pub fn korean_to_roman(text: Vec<String>) -> Result<Vec<String>, String> {
    Python::with_gil(|py| {
        // Importer hangul_romanize
        let module = py.import("hangul_romanize")
            .map_err(|e| format!("Failed to import hangul_romanize: {}", e))?;
        
        let romanize_fn = module.getattr("Transliter")
            .map_err(|e| e.to_string())?;
        
        let mut result = Vec::new();
        
        for line in text {
            let romanized = romanize_fn
                .call1((line,))
                .map_err(|e| e.to_string())?
                .extract::<String>()
                .map_err(|e| e.to_string())?;
            
            result.push(romanized);
        }
        
        Ok(result)
    })
}

/// Convertir français/anglais → IPA
pub fn to_ipa(text: Vec<String>, lang: &str) -> Result<Vec<String>, String> {
    Python::with_gil(|py| {
        let epitran_module = py.import("epitran")
            .map_err(|e| format!("Failed to import epitran: {}", e))?;
        
        let epitran_class = epitran_module.getattr("Epitran")
            .map_err(|e| e.to_string())?;
        
        // Créer instance pour la langue
        let lang_code = match lang {
            "fr" => "fra-Latn",
            "en" => "eng-Latn",
            _ => return Err("Unsupported language".to_string()),
        };
        
        let epitran = epitran_class.call1((lang_code,))
            .map_err(|e| e.to_string())?;
        
        let mut result = Vec::new();
        
        for line in text {
            let ipa = epitran
                .call_method1("transliterate", (line,))
                .map_err(|e| e.to_string())?
                .extract::<String>()
                .map_err(|e| e.to_string())?;
            
            result.push(ipa);
        }
        
        Ok(result)
    })
}

/// Fonction principale exposée à Tauri
pub fn generate_phonetic(text: Vec<String>, language: &str) -> Result<Vec<String>, String> {
    match language {
        "jp" => japanese_to_romaji(text),
        "kr" => korean_to_roman(text),
        "fr" => to_ipa(text, "fr"),
        "en" => to_ipa(text, "en"),
        _ => Ok(text), // Retourner texte original si langue non supportée
    }
}
```

#### 3. Commande Tauri

```rust
// src-tauri/src/commands/phonetic.rs
use crate::phonetic;

#[tauri::command]
pub async fn generate_phonetic_lyrics(
    lyrics: Vec<String>,
    language: String,
) -> Result<Vec<String>, String> {
    // Appeler fonction phonetic avec PyO3
    phonetic::generate_phonetic(lyrics, &language)
}
```

#### 4. Enregistrer dans main.rs

```rust
// src-tauri/src/main.rs
mod phonetic;
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::phonetic::generate_phonetic_lyrics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### Setup Python Environment pour PyO3

#### requirements.txt pour Python
```txt
pykakasi>=2.2.1
hangul-romanize>=0.1.0
epitran>=1.24
```

#### Installation

```bash
# Dans le projet Tauri
pip install pykakasi hangul-romanize epitran

# Ou via script de build
```

#### Distribution

**Problème :** L'app doit avoir Python installé pour utiliser PyO3

**Solutions :**

**Option A : Embedded Python (Recommandé)**
```toml
# Cargo.toml
[dependencies]
pyo3 = { version = "0.20", features = ["auto-initialize", "extension-module"] }
```
- Embarquer Python dans l'app
- Utilisateur n'a pas besoin d'installer Python
- App standalone

**Option B : Require Python**
- Documenter que Python 3.8+ requis
- Script d'installation vérifie Python
- Plus simple mais moins user-friendly

---

### Alternative Pure Rust (Plus complexe)

Si vous voulez éviter PyO3, implémentation Rust pure :

#### Japonais : wana_kana crate

```rust
// Japonais basique (hiragana/katakana → romaji)
use wana_kana;

fn japanese_basic_romaji(text: String) -> String {
    wana_kana::to_romaji(&text)
}
```

**Limite :** Ne gère PAS kanji → romaji (nécessite dictionnaire)

#### Coréen : Implémenter manuellement

```rust
// Algorithme de romanization coréen
// Complexe mais faisable
fn korean_to_roman_pure_rust(hangul: &str) -> String {
    // Décomposer hangul en jamo (consonnes/voyelles)
    // Appliquer règles romanization
    // ~200 lignes de code
    todo!()
}
```

**Complexité :** Haute, nécessite maintenir dictionnaire

---

## Recommandation Finale Phonétique

### Pour MVP (Recommandé)

**PyO3 + Python libs**
```
✅ Japonais : pykakasi (excellent)
✅ Coréen : hangul-romanize (bon)
✅ Français : epitran (basique)
✅ Anglais : epitran (basique)
```

**Avantages :**
- Qualité excellente (libs matures)
- Facile à implémenter
- Maintenable

**Inconvénient :**
- Dépendance Python (mais embarquable)

---

### Pour Production (Plus tard)

Si vous voulez éliminer dépendance Python :

**Services externes**
- API cloud pour phonétique
- Translate once, store locally (même stratégie)

**Ou Rust pur + dictionnaires**
- Complexe mais pas de dépendance
- Nécessite beaucoup plus de code

---

## Stockage Phonétique (Même Stratégie que Traduction)

### "Phonetize Once, Use Forever"

```sql
CREATE TABLE songs (
    id TEXT PRIMARY KEY,
    language TEXT NOT NULL,
    lyrics TEXT NOT NULL,           -- Original
    phonetic_lyrics TEXT,            -- Phonétique (généré une fois)
    phonetic_date TEXT,              -- Timestamp génération
    translations TEXT                -- {"en": [...]}
);
```

### Workflow

```
1. Utilisateur ajoute chanson japonaise
   → lyrics: ["千本桜", ...]
   → phonetic_lyrics: NULL

2. App génère phonétique (PyO3 + pykakasi)
   → phonetic_lyrics: ["Senbonzakura", ...]
   → Stocke dans SQLite

3. Usage offline illimité
   → SELECT lyrics, phonetic_lyrics
   → Affichage côte à côte
   → Pas de recalcul !
```

---

## Architecture Complète : Traduction + Phonétique

```
┌────────────────────────────────────────────────────┐
│  Utilisateur ajoute "上を向いて歩こう" (JP)          │
└──────────────────┬─────────────────────────────────┘
                   │
    ┌──────────────┴─────────────────┐
    │                                │
    ▼                                ▼
┌─────────────────┐      ┌──────────────────────┐
│  Traduction EN  │      │  Phonétique Romaji   │
│  (API - 1 fois) │      │  (PyO3 - 1 fois)     │
└────────┬────────┘      └──────────┬───────────┘
         │                          │
         ▼                          ▼
┌─────────────────────────────────────────────────┐
│           SQLite Database                       │
│                                                 │
│  lyrics:           ["上を向いて歩こう"]          │
│  phonetic_lyrics:  ["Ue wo muite arukou"]      │
│  translations: {                                │
│    "en": ["Let's walk looking up"]             │
│  }                                              │
└─────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────┐
│  Usage Offline Illimité                         │
│  ┌───────────────────────────────────────────┐ │
│  │ Original (JP) : 上を向いて歩こう           │ │
│  │ Phonétique    : Ue wo muite arukou        │ │
│  │ Traduction EN : Let's walk looking up     │ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

---

## Résumé Recommandations

### Traduction
✅ **LibreTranslate** (gratuit, public API)
- Traduire lors de l'ajout de chanson
- Stocker dans SQLite (JSON field)
- Offline ensuite

**Alternative :** DeepL (meilleure qualité, 500k/mois gratuit)

### Phonétique
✅ **PyO3 + Python libs**
- pykakasi (JP → romaji)
- hangul-romanize (KR → roman)
- epitran (FR/EN → IPA)
- Générer lors de l'ajout
- Stocker dans SQLite
- Offline ensuite

**Alternative :** Rust pur (plus complexe, pas de dépendance Python)

### Architecture
```
┌──────────────────────────────┐
│  Add Song Flow               │
├──────────────────────────────┤
│  1. Save original lyrics     │
│  2. Translate (LibreTranslate)│
│  3. Generate phonetic (PyO3) │
│  4. Store all in SQLite      │
└──────────────────────────────┘
         │
         ▼
┌──────────────────────────────┐
│  Display Flow (Offline)      │
├──────────────────────────────┤
│  SELECT FROM songs           │
│  → Original                  │
│  → Phonetic                  │
│  → Translation               │
│  No API calls!               │
└──────────────────────────────┘
```

---

## Prêt à Implémenter ?

**Phase 1 :** Backend Rust avec PyO3
**Phase 2 :** Traduction avec LibreTranslate
**Phase 3 :** Stockage SQLite
**Phase 4 :** Frontend Vue affichage

**Voulez-vous que je commence l'implémentation ?** 🚀
