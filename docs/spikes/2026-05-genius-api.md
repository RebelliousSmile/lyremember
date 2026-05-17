# Spike : API métadonnées Genius

> **Statut** : R&D — pas d'implémentation production en cette itération.
> **Date** : mai 2026.
> **Issue** : [#26](https://github.com/RebelliousSmile/lyremember/issues/26).
> **Décision** : **NO-GO court terme** — voir conclusion.

## Contexte

USER_STORIES_V2 (US-1.3, US-2.1) évoquait l'« import de paroles depuis
Genius ». Cette story a été refondue (#14) car :
- **Scraper genius.com viole les ToS** (cf. réponses de la lib
  `lyricsgenius` et `Apify`, qui contournent l'API officielle pour
  extraire les lyrics via Beautiful Soup).
- **L'API officielle ne fournit pas les paroles** : `api.genius.com`
  expose des métadonnées (search, song details, annotations) mais
  pas le champ `lyrics`.

Le présent spike évalue ce qu'on peut faire **légalement** avec
l'API métadonnées seulement.

## Périmètre exploré

API officielle : <https://docs.genius.com/>. Authentification par token
client (gratuit, généré sur <https://genius.com/api-clients>).

### Endpoints potentiellement utiles

| Endpoint | Renvoie | Intérêt produit |
|---|---|---|
| `GET /search?q=...` | Liste de songs (id, title, artist, URL Genius) | Auto-complétion à la création d'une chanson |
| `GET /songs/:id` | Métadonnées (album, release_date, producer, featured artists, song_art_url) | Enrichir la fiche après création |
| `GET /artists/:id/songs` | Catalogue d'un artiste | Discover, hors scope MVP |
| `GET /referents` | Annotations utilisateur | Pas pertinent pour LyRemember |

### Ce qui **n'est pas** exposé

- `lyrics`, `body.dom` ou équivalent — confirmé sur la doc et par tous
  les wrappers (lyricsgenius, scf4/lyricist, etc.) qui scrapent en
  HTTP pour pallier le manque.

## Valeur produit estimée

| Use case | Couvert par l'API ? | Valeur |
|---|---|---|
| Importer les paroles | ❌ Non (illégal en scraping) | ★★★★ |
| Auto-compléter titre/artiste à la création | ✅ Oui (`/search`) | ★★ |
| Récupérer URL canonique pour le bouton "Open on Genius" | ✅ Oui | ★ (déjà couvert par #17 : l'utilisateur colle son URL) |
| Cover art / image | ✅ Oui (`song_art_image_url`) | ★★ |
| Date / album / producer | ✅ Oui | ★ (pas demandé par user stories) |

## Coûts

- Auth : token client (gratuit, sans quota documenté en dev).
- Implémentation : nouveau module Rust (`rust-backend/src/services/genius.rs`)
  + reqwest call + JSON deserialization → ~150 lignes.
- UI : modal d'auto-complétion sur `AddSongView`.
- Maintenance : ToS Genius peuvent changer (déjà vu pour les wrappers).

## Recommandation

**NO-GO court terme.** Trois raisons :

1. Le bénéfice marginal est faible : `/search` fait juste gagner
   quelques frappes au clavier. Les utilisateurs collent déjà les
   paroles manuellement (US-2.1 cas par défaut).
2. Le champ `genius_url` (#17) suffit pour le lien sortant — qui est
   le vrai cas d'usage légal.
3. Implémenter un appel API externe ajoute une dépendance réseau et
   un token client à provisionner par utilisateur.

**Re-évaluer** si :
- On veut un onboarding plus rapide pour les utilisateurs casuals.
- On veut afficher `song_art_image_url` (cover) sur les fiches.
- Une mécanique communautaire d'annotation devient pertinente.

## Hors périmètre

- Toute extraction de lyrics (illégal, ToS).
- Implémentation production.
