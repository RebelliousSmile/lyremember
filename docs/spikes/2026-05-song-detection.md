# Spike : détection de chanson en cours d'écoute (Shazam-like)

> **Statut** : R&D — pas d'implémentation production en cette itération.
> **Date** : mai 2026.
> **Issue** : [#27](https://github.com/RebelliousSmile/lyremember/issues/27).
> **Décision** : **NO-GO MVP, à reconsidérer post-MVP** — voir conclusion.

## Contexte

Idée bonus issue de la session d'audit : permettre à LyRemember de
détecter automatiquement la chanson en cours d'écoute pour proposer de
l'ajouter à la bibliothèque (UX à la Shazam ou à l'app mobile Genius).

## Technologies évaluées

| Service | Modèle | Tarification (mai 2026) | Latence | Couverture |
|---|---|---|---|---|
| **AudD** | API REST | Plan gratuit 25 req/jour ; à partir de 9 USD/mois pour 10k req | 1–3 s | Bon mainstream, plus faible sur underground |
| **ACRCloud** | API REST + SDK natifs | Plan gratuit limité ; commercial à partir de ~99 USD/mois | < 1 s | Catalogue large, très bonne langues asiatiques |
| **Shazam (RapidAPI)** | Reverse-engineered API | Tier gratuit limité, commercial à partir de quelques USD pour 1k req | 1–2 s | Excellent catalogue mainstream |
| **Implémentation propre (Chromaprint/AcoustID + MusicBrainz)** | Open-source | Gratuit | 3–5 s | Limité à MusicBrainz (catalogue OSS) |

Recommandation techno : **AudD** pour démarrer (free tier suffisant
pour spike → bêta) ; **ACRCloud** si on cible JP/KR sérieusement.

## Faisabilité capture audio Tauri v2

### Desktop (Windows / macOS / Linux)
- `tauri-plugin-mic` n'existe pas officiellement.
- Possibilités :
  - Capturer via une crate Rust : `cpal` (cross-platform, mature) →
    fournit un buffer PCM, à encoder en MP3/OGG/WAV pour envoi API.
  - Côté webview : `navigator.mediaDevices.getUserMedia` est dispo
    sur la WebView Tauri (Webkit/WebView2/WRY), mais limité par les
    permissions OS (entitlement micro côté macOS, capability
    `permissionMicrophone` côté Windows).
- Coût d'implémentation : ~2 jours pour un POC fonctionnel.

### Android
- `getUserMedia` fonctionne dans la WebView Android Tauri.
- Permission manifeste : `RECORD_AUDIO` (à ajouter dans
  `lyremember-app/src-tauri/gen/android/.../AndroidManifest.xml`).
- Encoding côté JS via `MediaRecorder` → blob → envoi API.

### iOS
- Idem, mais nécessite l'entitlement `NSMicrophoneUsageDescription`
  dans `Info.plist`. Pas de build iOS prouvé pour LyRemember
  aujourd'hui (cf. release.yml qui ne cible que Windows + Android).

## Aspects légaux

- **Enregistrement audio environnement** : implique l'enregistrement
  d'éventuelles conversations en arrière-plan → RGPD (consentement
  explicite, durée minimale, suppression immédiate après identification).
- **ToS des services de reconnaissance** : tous interdisent la
  redistribution des résultats d'identification (donc on ne peut pas
  enrichir une base publique avec les détections — c'est OK pour notre
  usage personnel).
- **Pas d'enregistrement persistant** : capturer 5 s, envoyer, jeter.

## Coût d'usage (estimation pour 100 / 1000 utilisateurs)

Hypothèse : 3 détections/jour/utilisateur actif.

| Échelle | Requêtes/mois | AudD (15 c/req ≈ 9 USD/mois plafonné par tiers, ou 99 USD pro) | ACRCloud (~9 USD/1k req) |
|---|---|---|---|
| 100 utilisateurs | 9 000 | 9 USD (free) → 0.5k req gratuits puis paid | ~80 USD |
| 1 000 utilisateurs | 90 000 | 99 USD (pro 10k req) → multi-plan | ~810 USD |

→ Modèle économique à ajuster (limite de quotas freemium, abonnement
premium, ou self-hosting Chromaprint pour gros volume).

## Recommandation

**NO-GO pour le MVP** :

1. Effort d'implémentation : 1–2 semaines (Rust audio capture +
   encoding + API client + UI + permissions Android/iOS + tests).
2. Coût récurrent significatif au-delà du free tier — non aligné
   avec le scope « gratuit, offline-first ».
3. UX n'est pas critique pour la promesse principale (mémoriser des
   paroles), juste un confort d'ajout.

**Conditions pour ré-évaluer post-MVP** :
- Le nombre d'utilisateurs justifie l'investissement (>500 actifs).
- Un partenariat couvre les coûts API.
- Un alternative open-source (Chromaprint + MusicBrainz) est jugée
  suffisante en couverture pour le public cible.

## Hors périmètre

- POC code (autorisé par l'issue uniquement si nécessaire pour valider
  la captation audio — pas effectué ici, le détail technique
  ci-dessus suffit).
- Implémentation production.
