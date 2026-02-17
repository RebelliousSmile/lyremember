# Décisions Technologiques Finales - LyRemember

## Contexte
Application pour mémoriser paroles de chansons en plusieurs langues avec modes de pratique variés.

## Contraintes Utilisateur
1. **Plateformes :** Desktop ET Mobile
2. **Budget :** 0€ (gratuit)
3. **Timeline :** Flexible
4. **Langues :** Français, Anglais, Coréen, Japonais

---

## ✅ DÉCISION FINALE : Progressive Web App (PWA)

### Pourquoi PWA ?

**Avantages :**
- ✅ **Un seul codebase** pour desktop + mobile + tablette
- ✅ **100% gratuit** (pas de frais App Store/Play Store)
- ✅ **Accessible partout** via navigateur
- ✅ **Installable** comme application native
- ✅ **Fonctionne offline** (avec Service Workers)
- ✅ **Mises à jour instantanées** (pas d'app store review)
- ✅ **Responsive** (s'adapte à toutes tailles d'écran)
- ✅ **Accès micro/audio** pour mode vocal
- ✅ **Notifications** possibles
- ✅ **Partage facile** (juste un lien)

**Sur Desktop :**
- Fonctionne dans Chrome, Firefox, Safari, Edge
- Installable via "Ajouter à l'écran d'accueil" (Chrome/Edge)
- Icône dans barre de tâches/dock
- Fenêtre standalone (pas de barre d'adresse)

**Sur Mobile :**
- Fonctionne dans tous navigateurs
- Installable comme app (Android + iOS)
- Icône sur écran d'accueil
- Plein écran
- Gestes tactiles

---

## Stack Technique Complète

### Frontend
**Framework :** React 18 + TypeScript
- **Pourquoi React ?** Écosystème riche, bon pour PWA, composants réutilisables
- **TypeScript :** Type safety, meilleur DX

**Build Tool :** Vite
- **Pourquoi Vite ?** Ultra rapide, HMR excellent, PWA plugin disponible

**Styling :** Tailwind CSS
- **Pourquoi Tailwind ?** Rapide, responsive facile, design system cohérent

**State Management :** Zustand (ou Context API)
- **Pourquoi Zustand ?** Simple, léger, pas de boilerplate

**Routing :** React Router v6
- **Pourquoi ?** Standard React, bon pour SPA

**UI Components :** Headless UI + custom components
- **Pourquoi ?** Accessible, customizable, gratuit

**Icônes :** Lucide React
- **Pourquoi ?** Léger, beau, open-source

**PWA :** vite-plugin-pwa
- **Pourquoi ?** Génère Service Worker et manifest automatiquement

---

### Backend
**Framework :** Python FastAPI
- **Pourquoi ?** 
  - Réutilise code existant
  - Async/await (performant)
  - Auto-documentation (Swagger)
  - Excellentes libs pour NLP/traduction

**Base de données :** SQLite + SQLAlchemy
- **Pourquoi SQLite ?**
  - Gratuit, pas de serveur
  - Fichier unique, portable
  - Suffisant pour usage personnel
  - Peut migrer vers PostgreSQL plus tard

**Auth :** JWT (JSON Web Tokens)
- **Pourquoi ?** Stateless, fonctionne bien avec SPA, sécurisé

**CORS :** FastAPI CORS Middleware
- **Pourquoi ?** Frontend et backend sur domaines différents

---

### Services Gratuits

**1. Import Paroles : Genius API**
- **Service :** https://genius.com/api-clients
- **Coût :** Gratuit avec token personnel
- **Limite :** Raisonnable pour usage personnel
- **Lib :** lyricsgenius (Python)

**2. Traduction : deep-translator**
- **Service :** Google Translate (via scraping non-officiel)
- **Coût :** Gratuit
- **Limite :** Rate limiting possible
- **Fallback :** Traductions manuelles
- **Lib :** deep-translator (Python)

**3. Phonétique (Japonais) : pykakasi**
- **Service :** Local, offline
- **Coût :** Gratuit
- **Fonction :** Kanji → Hiragana → Romaji
- **Lib :** pykakasi (Python)

**4. Phonétique (Coréen) : hangul-romanize**
- **Service :** Local, offline
- **Coût :** Gratuit
- **Fonction :** Hangul → Romanization
- **Lib :** hangul-romanize (Python)

**5. Phonétique (FR/EN) : epitran**
- **Service :** Local, offline
- **Coût :** Gratuit
- **Fonction :** Texte → IPA (International Phonetic Alphabet)
- **Lib :** epitran (Python)

**6. Reconnaissance Vocale : Web Speech API**
- **Service :** Intégré navigateur (Chrome/Edge)
- **Coût :** Gratuit
- **Limite :** Nécessite connexion internet
- **Support :** Chrome > Edge > Safari limité, Firefox non
- **Fallback :** Mode oral optionnel

---

### Hébergement Gratuit

**Frontend : Vercel**
- **Coût :** Gratuit (hobby plan)
- **Features :**
  - Déploiement auto depuis GitHub
  - HTTPS inclus
  - CDN global
  - Domaine personnalisé gratuit (.vercel.app)
- **Limites :** 100GB bande passante/mois (largement suffisant)

**Backend : Railway**
- **Coût :** Gratuit (500h/mois)
- **Features :**
  - PostgreSQL inclus (si besoin plus tard)
  - Déploiement auto depuis GitHub
  - HTTPS inclus
- **Limites :** Sleep après 5min inactivité (acceptable pour usage perso)
- **Alternative :** Render (même concept)

**OU Backend auto-hébergé :**
- Raspberry Pi à la maison
- VPS gratuit (Oracle Cloud Always Free, Google Cloud free tier)

---

## Architecture Détaillée

```
┌─────────────────────────────────────────────────────────────────┐
│                      CLIENT (PWA)                                │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    React App                              │  │
│  │  ┌─────────┬─────────┬─────────┬─────────┬──────────┐   │  │
│  │  │ Login   │ Songs   │ Practice│ Stats   │ Settings │   │  │
│  │  │ Page    │ List    │ Modes   │ Page    │ Page     │   │  │
│  │  └─────────┴─────────┴─────────┴─────────┴──────────┘   │  │
│  │                                                           │  │
│  │  Components:                                              │  │
│  │  - SongCard (affiche chanson avec VO/phonétique)         │  │
│  │  - KaraokeMode (défilement phrase par phrase)            │  │
│  │  - FillBlankMode (phrases à trous)                       │  │
│  │  - MCQMode (propositions multiples)                      │  │
│  │  - VoiceMode (reconnaissance vocale)                     │  │
│  │  - PhoneticDisplay (affichage phonétique)                │  │
│  └──────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           │ HTTP/REST (axios)                    │
│                           ▼                                      │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                   SERVER (FastAPI)                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                  API Routes                               │  │
│  │  /api/auth/register                                       │  │
│  │  /api/auth/login                                          │  │
│  │  /api/songs (GET, POST, PUT, DELETE)                      │  │
│  │  /api/songs/{id}/phonetic (génère phonétique)            │  │
│  │  /api/songs/{id}/translate (traduit en EN)               │  │
│  │  /api/genius/search (cherche sur Genius)                 │  │
│  │  /api/genius/import (importe depuis Genius)              │  │
│  │  /api/practice/session (enregistre session)              │  │
│  │  /api/stats (récupère stats)                             │  │
│  └──────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Business Logic                               │  │
│  │  - UserManager (auth, profil)                            │  │
│  │  - SongManager (CRUD, import Genius)                     │  │
│  │  - PhoneticEngine (pykakasi, hangul-romanize, epitran)  │  │
│  │  - TranslationEngine (deep-translator)                   │  │
│  │  - PracticeEngine (modes, scoring)                       │  │
│  │  - ProgressTracker (stats, recommandations)              │  │
│  └──────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Database (SQLite)                            │  │
│  │  Tables:                                                  │  │
│  │  - users (id, username, email, password_hash, ...)       │  │
│  │  - songs (id, title, artist, language, lyrics, ...)      │  │
│  │  - user_songs (user_id, song_id) -- répertoire          │  │
│  │  - sessions (id, user_id, song_id, mode, score, ...)    │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                   EXTERNAL SERVICES                              │
│  - Genius API (import paroles)                                  │
│  - Web Speech API (reconnaissance vocale, côté client)          │
│  - Google Translate (via deep-translator, traduction)           │
└─────────────────────────────────────────────────────────────────┘
```

---

## Structure Projet

```
lyremember/
├── backend/                    # API Python FastAPI
│   ├── app/
│   │   ├── __init__.py
│   │   ├── main.py            # Point d'entrée FastAPI
│   │   ├── models.py          # SQLAlchemy models
│   │   ├── schemas.py         # Pydantic schemas
│   │   ├── database.py        # DB connection
│   │   ├── auth.py            # JWT auth
│   │   ├── routers/
│   │   │   ├── auth.py        # Auth routes
│   │   │   ├── songs.py       # Songs routes
│   │   │   ├── practice.py    # Practice routes
│   │   │   └── genius.py      # Genius API routes
│   │   ├── services/
│   │   │   ├── user_service.py
│   │   │   ├── song_service.py
│   │   │   ├── phonetic_service.py
│   │   │   ├── translation_service.py
│   │   │   └── genius_service.py
│   │   └── utils/
│   ├── tests/
│   ├── requirements.txt
│   └── .env
│
├── frontend/                   # React PWA
│   ├── public/
│   │   ├── manifest.json      # PWA manifest
│   │   └── icons/             # App icons
│   ├── src/
│   │   ├── main.tsx           # Point d'entrée
│   │   ├── App.tsx
│   │   ├── pages/
│   │   │   ├── LoginPage.tsx
│   │   │   ├── RegisterPage.tsx
│   │   │   ├── DashboardPage.tsx
│   │   │   ├── SongsPage.tsx
│   │   │   ├── SongDetailPage.tsx
│   │   │   ├── PracticePage.tsx
│   │   │   └── StatsPage.tsx
│   │   ├── components/
│   │   │   ├── SongCard.tsx
│   │   │   ├── PhoneticDisplay.tsx
│   │   │   ├── KaraokeMode.tsx
│   │   │   ├── FillBlankMode.tsx
│   │   │   ├── MCQMode.tsx
│   │   │   ├── VoiceMode.tsx
│   │   │   └── Layout.tsx
│   │   ├── services/
│   │   │   └── api.ts         # API client
│   │   ├── stores/
│   │   │   └── authStore.ts   # Zustand store
│   │   ├── types/
│   │   │   └── index.ts
│   │   └── utils/
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   └── package.json
│
├── docs/                       # Documentation
├── .github/
│   └── workflows/
│       └── deploy.yml         # CI/CD
└── README.md
```

---

## Roadmap MVP (2-3 semaines)

### Semaine 1 : Backend + Auth
- [ ] Setup FastAPI + SQLite
- [ ] Modèles User, Song, Session
- [ ] Auth JWT (register, login)
- [ ] API CRUD songs
- [ ] Tests backend

### Semaine 2 : Frontend Base
- [ ] Setup React + Vite + Tailwind
- [ ] Pages Login/Register
- [ ] Dashboard + Song List
- [ ] Connexion API
- [ ] Responsive mobile

### Semaine 3 : Fonctionnalités
- [ ] Import Genius
- [ ] Affichage phonétique (JP, KR)
- [ ] Mode karaoke (défilement)
- [ ] Mode phrases à trous
- [ ] Sauvegarde progression

### Semaine 4 : Polish + PWA
- [ ] Mode QCM
- [ ] Traduction auto EN
- [ ] Service Worker (offline)
- [ ] Manifest (installable)
- [ ] Déploiement Vercel + Railway

---

## Évolution Future (Si Intérêt)

### Phase 2 (Apps Natives)
Si besoin de vraies apps natives plus tard :
- **Desktop :** Tauri (Rust + Web, très léger)
- **Mobile :** Capacitor (compile PWA en app native)

### Phase 3 (Fonctionnalités Premium)
- Mode collaboratif (pratiquer à plusieurs)
- Synchronisation cloud
- Export/import données
- Thèmes personnalisés
- Classements / challenges

---

## Validation

✅ **Desktop :** Fonctionne dans navigateur, installable via Chrome/Edge  
✅ **Mobile :** Fonctionne dans navigateur, installable comme app  
✅ **Gratuit :** Toutes technos et hébergement gratuits  
✅ **FR/EN/KR/JP :** Support phonétique pour les 4 langues  
✅ **Flexible :** Peut évoluer sans réécriture complète  

**Prêt à commencer l'implémentation !** 🚀
