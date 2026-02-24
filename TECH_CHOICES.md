# Choix Technologiques - LyRemember

## Contexte
Nous devons choisir la stack technique AVANT de créer les user stories détaillées et de coder.

---

## 1. Type d'Application

### Option A : Application Web (Recommandé)
**Avantages :**
- ✅ Accessible depuis n'importe quel appareil (PC, tablette, smartphone)
- ✅ Pas d'installation nécessaire
- ✅ Mise à jour centralisée
- ✅ Meilleure pour affichage multi-colonnes (VO + traduction + phonétique)
- ✅ Meilleur support audio/micro pour mode oral
- ✅ Interface riche pour mode karaoke/défilement

**Inconvénients :**
- ❌ Nécessite hébergement
- ❌ Plus complexe à développer qu'une CLI
- ❌ Nécessite connexion internet (sauf si PWA)

**Stack possible :**
- Frontend : React / Vue.js / Svelte
- Backend : Node.js (Express/Fastify) ou Python (Flask/Django) ou Go
- Base de données : PostgreSQL / MongoDB / SQLite
- Hébergement : Vercel / Netlify / Heroku / Railway

---

### Option B : Application Desktop
**Avantages :**
- ✅ Fonctionne offline
- ✅ Bonnes performances
- ✅ Accès complet au système (micro, fichiers)

**Inconvénients :**
- ❌ Installation nécessaire
- ❌ Maintenance multi-OS (Windows, Mac, Linux)
- ❌ Pas accessible sur mobile

**Stack possible :**
- Electron (JavaScript/TypeScript)
- Tauri (Rust + Web)
- Python + PyQt/Tkinter

---

### Option C : Application Mobile
**Avantages :**
- ✅ Toujours avec soi
- ✅ Bon pour micro/audio
- ✅ Notifications possibles

**Inconvénients :**
- ❌ Écran petit pour affichage multi-colonnes
- ❌ Développement iOS + Android
- ❌ App stores (délais, coûts)

**Stack possible :**
- React Native
- Flutter
- Swift (iOS) + Kotlin (Android)

---

### Option D : CLI (Ligne de Commande)
**Avantages :**
- ✅ Développement rapide
- ✅ Léger
- ✅ Parfait pour développeurs

**Inconvénients :**
- ❌ Interface limitée (pas idéal pour karaoke/défilement)
- ❌ Difficile pour affichage phonétique/multi-langues
- ❌ Pas adapté pour mode oral (micro compliqué en CLI)
- ❌ Pas user-friendly pour utilisateurs non-techniques

**Stack possible :**
- Python (déjà commencé)
- Node.js
- Go

---

### Option E : Hybride (Web + Mobile PWA)
**Avantages :**
- ✅ Un seul code pour web + mobile
- ✅ Installable comme une app
- ✅ Fonctionne offline (si configuré)
- ✅ Notifications

**Inconvénients :**
- ❌ Limites PWA sur iOS
- ❌ Performances légèrement inférieures aux apps natives

**Stack possible :**
- Frontend : React/Vue/Svelte
- Backend : Idem Option A
- Service Worker pour offline

---

## 2. Langage Backend

### Option A : Python
**Pour :**
- ✅ Déjà commencé
- ✅ Excellentes bibliothèques NLP/traduction
- ✅ Facile pour API Genius, translittération
- ✅ SpeechRecognition disponible

**Contre :**
- ❌ Moins performant que Go/Rust
- ❌ Déploiement un peu plus lourd

**Frameworks :**
- FastAPI (moderne, rapide, async)
- Flask (simple, léger)
- Django (tout inclus, mais lourd pour ce projet)

---

### Option B : Node.js (JavaScript/TypeScript)
**Pour :**
- ✅ Même langage frontend/backend
- ✅ Performant (async)
- ✅ NPM riche en packages
- ✅ Bon écosystème

**Contre :**
- ❌ Moins de libs pour translittération/NLP que Python
- ❌ Reconnaissance vocale limitée

**Frameworks :**
- Express (classique)
- Fastify (rapide)
- NestJS (structuré)

---

### Option C : Go
**Pour :**
- ✅ Très performant
- ✅ Compilé (déploiement simple)
- ✅ Bon pour APIs

**Contre :**
- ❌ Écosystème moins riche pour NLP/traduction
- ❌ Courbe d'apprentissage

---

## 3. Base de Données

### Option A : PostgreSQL
**Pour :**
- ✅ Robuste, relationnel
- ✅ Bon pour relations User-Songs
- ✅ Support JSON pour métadonnées
- ✅ Gratuit, open-source

**Contre :**
- ❌ Nécessite serveur séparé
- ❌ Plus complexe que SQLite

---

### Option B : MongoDB
**Pour :**
- ✅ Flexible (schema-less)
- ✅ Bon pour données non structurées
- ✅ Facile à débuter

**Contre :**
- ❌ Pas idéal pour relations complexes
- ❌ Peut devenir désordonné

---

### Option C : SQLite
**Pour :**
- ✅ Simple, fichier unique
- ✅ Pas de serveur nécessaire
- ✅ Parfait pour MVP

**Contre :**
- ❌ Pas adapté pour multi-utilisateurs simultanés
- ❌ Moins de fonctionnalités

---

### Option D : JSON Files (actuel)
**Pour :**
- ✅ Ultra simple
- ✅ Humainement lisible
- ✅ Bon pour prototype

**Contre :**
- ❌ Pas scalable
- ❌ Pas de requêtes complexes
- ❌ Pas de transactions
- ❌ Risque de corruption

---

## 4. Frontend (si Web)

### Option A : React
**Pour :**
- ✅ Écosystème énorme
- ✅ Beaucoup de composants disponibles
- ✅ Bon pour applications complexes

**Contre :**
- ❌ Courbe d'apprentissage
- ❌ Boilerplate

---

### Option B : Vue.js
**Pour :**
- ✅ Plus simple que React
- ✅ Documentation excellente
- ✅ Bon compromis

**Contre :**
- ❌ Écosystème plus petit que React

---

### Option C : Svelte
**Pour :**
- ✅ Très performant
- ✅ Moins de code
- ✅ Compilation, pas de runtime

**Contre :**
- ❌ Écosystème jeune
- ❌ Moins de ressources

---

## 5. Reconnaissance Vocale (pour mode oral)

### Option A : Web Speech API (Browser)
**Pour :**
- ✅ Gratuit
- ✅ Intégré au navigateur
- ✅ Facile à utiliser

**Contre :**
- ❌ Fonctionne uniquement en ligne
- ❌ Support limité (Chrome > Firefox > Safari)
- ❌ Envoie audio à Google

---

### Option B : Google Cloud Speech-to-Text
**Pour :**
- ✅ Très précis
- ✅ Multi-langues
- ✅ API complète

**Contre :**
- ❌ Payant (après quota gratuit)
- ❌ Nécessite connexion

**Prix :** 0.006$/15s (gratuit : 60min/mois)

---

### Option C : Whisper (OpenAI) - Offline possible
**Pour :**
- ✅ Très précis
- ✅ Multi-langues
- ✅ Peut tourner en local

**Contre :**
- ❌ Nécessite GPU pour temps réel
- ❌ Lourd

---

### Option D : SpeechRecognition (Python)
**Pour :**
- ✅ Facile (déjà dans requirements)
- ✅ Plusieurs backends possibles

**Contre :**
- ❌ Principalement pour CLI/desktop
- ❌ Qualité variable

---

## 6. Traduction Automatique

### Option A : Google Translate API
**Pour :**
- ✅ Excellente qualité
- ✅ Beaucoup de langues

**Contre :**
- ❌ Payant (après quota)

**Prix :** 20$/million caractères

---

### Option B : DeepL API
**Pour :**
- ✅ Meilleure qualité que Google
- ✅ Spécialisé traduction

**Contre :**
- ❌ Payant
- ❌ Moins de langues

**Prix :** 5€/mois (500k caractères)

---

### Option C : LibreTranslate (Open Source)
**Pour :**
- ✅ Gratuit
- ✅ Auto-hébergeable
- ✅ Open-source

**Contre :**
- ❌ Qualité inférieure
- ❌ Moins de langues

---

### Option D : deep-translator (Python lib)
**Pour :**
- ✅ Gratuit (utilise Google Translate non-officiel)
- ✅ Facile

**Contre :**
- ❌ Pas officiel, peut casser
- ❌ Rate limiting

---

## 7. Translittération/Phonétique

### Option A : epitran
**Pour :**
- ✅ Multi-langues
- ✅ Basé sur IPA (International Phonetic Alphabet)

**Contre :**
- ❌ Limité pour idéogrammes

---

### Option B : pykakasi (Japonais)
**Pour :**
- ✅ Spécialisé japonais
- ✅ Kanji → Romaji très bon

**Contre :**
- ❌ Japonais uniquement

---

### Option C : pinyin (Chinois)
**Pour :**
- ✅ Chinois → Pinyin

---

### Option D : hangul-romanize (Coréen)
**Pour :**
- ✅ Coréen → Romanisation

---

## 8. Hébergement (si Web)

### Option A : Vercel
**Pour :**
- ✅ Gratuit pour hobby
- ✅ Déploiement facile
- ✅ Excellent pour Next.js/React

**Contre :**
- ❌ Serverless (limites pour certaines fonctionnalités)

---

### Option B : Railway / Render
**Pour :**
- ✅ Gratuit avec limites
- ✅ Bon pour full-stack
- ✅ Base de données incluse

**Contre :**
- ❌ Sleep après inactivité (version gratuite)

---

### Option C : Heroku
**Pour :**
- ✅ Classique
- ✅ Add-ons faciles

**Contre :**
- ❌ Plus de plan gratuit depuis 2022

---

### Option D : VPS (DigitalOcean, Linode, etc.)
**Pour :**
- ✅ Contrôle total
- ✅ Prévisible

**Contre :**
- ❌ Nécessite administration
- ❌ ~5$/mois minimum

---

## Recommandations par Scénario

### Scénario 1 : MVP Rapide (1-2 semaines)
**Type :** Application Web simple
**Stack :**
- Frontend : Vue.js ou React (create-react-app)
- Backend : Python FastAPI (continuer ce qui existe)
- BDD : SQLite
- Hébergement : Vercel (front) + Railway (back)
- Traduction : deep-translator (gratuit)
- Phonétique : epitran + pykakasi
- Vocal : Web Speech API (gratuit)

**Pourquoi :** Balance rapidité/fonctionnalités, gratuit

---

### Scénario 2 : MVP Premium (4-6 semaines)
**Type :** Application Web PWA
**Stack :**
- Frontend : React + TypeScript
- Backend : Python FastAPI
- BDD : PostgreSQL
- Hébergement : Railway
- Traduction : DeepL API
- Phonétique : epitran + libs spécialisées
- Vocal : Google Cloud Speech-to-Text

**Pourquoi :** Qualité professionnelle, scalable

---

### Scénario 3 : Application Complète (3+ mois)
**Type :** Web + Mobile (React Native)
**Stack :**
- Frontend Web : Next.js + TypeScript
- Mobile : React Native
- Backend : Node.js (TypeScript) ou Go
- BDD : PostgreSQL
- Hébergement : VPS ou Cloud Platform
- Toutes les fonctionnalités premium

**Pourquoi :** Solution complète, toutes plateformes

---

## Questions à Répondre

1. **Quel est le public cible ?**
   - [ ] Développeurs/tech-savvy → CLI acceptable
   - [ ] Grand public → Web/Mobile nécessaire

2. **Quel est le budget ?**
   - [ ] 0€ → Stack gratuite (deep-translator, Web Speech API, SQLite)
   - [ ] ~20€/mois → Services premium (DeepL, Google Speech, PostgreSQL hébergé)
   - [ ] ~100€/mois → Infrastructure scalable

3. **Quelle est la timeline ?**
   - [ ] 1-2 semaines → MVP minimal
   - [ ] 1-2 mois → MVP solide
   - [ ] 3+ mois → Produit complet

4. **Quelles langues sont prioritaires ?**
   - [ ] Japonais/Coréen/Chinois (idéogrammes) → Phonétique crucial
   - [ ] Langues européennes → Phonétique moins important
   - [ ] Toutes → Solution universelle

5. **Mode d'utilisation principal ?**
   - [ ] Seul à la maison → Desktop/Web OK
   - [ ] En déplacement → Mobile nécessaire
   - [ ] Les deux → PWA ou Multi-plateforme

6. **Priorité sur quels modes de pratique ?**
   - [ ] Karaoke/défilement → Web meilleur
   - [ ] Oral → Bon micro nécessaire
   - [ ] Trous/QCM → CLI peut suffire

---

## Ma Recommandation

**Pour commencer :** 

**🎯 Application Web Progressive (PWA)**

**Stack :**
- **Frontend :** React + TypeScript + Tailwind CSS
- **Backend :** Python FastAPI (continuer existant)
- **BDD :** PostgreSQL (SQLite pour dev)
- **Auth :** JWT tokens
- **Traduction :** deep-translator (gratuit, upgrade DeepL plus tard)
- **Phonétique :** epitran + pykakasi (JP) + pinyin (CN)
- **Vocal :** Web Speech API (gratuit)
- **Hébergement :** Vercel (front) + Railway (back + BDD)

**Pourquoi :**
- ✅ Gratuit pour commencer
- ✅ Fonctionne partout (PC, mobile, tablette)
- ✅ Bonne UX pour karaoke/défilement
- ✅ Peut évoluer en mobile app plus tard
- ✅ Réutilise le code Python déjà écrit
- ✅ Stack moderne et demandée

**Phase 1 (MVP - 2-3 semaines) :**
1. Comptes utilisateurs
2. Import Genius
3. Affichage VO + phonétique
4. Mode défilement basique
5. Mode trous

**Phase 2 (1-2 semaines) :**
6. Traduction EN
7. Mode QCM
8. Amélioration UI

**Phase 3 (2-3 semaines) :**
9. Mode vocal
10. Stats avancées
11. PWA (offline, install)

---

## Action Requise

**Merci de répondre aux questions et de valider :**

1. Type d'application préféré ? (Web / Desktop / Mobile / CLI / Autre)
2. Budget disponible ? (0€ / ~20€/mois / ~100€/mois / Autre)
3. Timeline souhaitée ? (MVP en combien de temps ?)
4. Langues prioritaires ? (JP/KR/CN / Européennes / Toutes / Autres)
5. D'accord avec ma recommandation ou préférez autre chose ?

**Une fois validé, je pourrai :**
- Ajuster les user stories selon la stack choisie
- Créer l'architecture technique détaillée
- Commencer l'implémentation
