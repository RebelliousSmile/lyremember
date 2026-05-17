# User Stories - Nouvelles Exigences LyRemember

## Contexte
Application pour mémoriser et pratiquer des paroles de chansons en plusieurs langues, avec support phonétique et modes de pratique variés.

---

## Epic 1 : Gestion des Comptes Utilisateur

### US-1.1 : Créer un compte
**En tant qu'** utilisateur  
**Je veux** pouvoir créer un compte avec mes informations  
**Afin de** sauvegarder mon répertoire personnel de chansons

**Critères d'acceptation :**
- [ ] L'utilisateur peut s'inscrire avec : nom d'utilisateur, email, mot de passe
- [ ] Le système vérifie que le nom d'utilisateur est unique
- [ ] Le mot de passe est stocké de manière sécurisée (hashé)
- [ ] Un message de confirmation est affiché après inscription
- [ ] Les données sont persistantes entre les sessions

**Scénarios de test :**
```
GIVEN je suis un nouvel utilisateur
WHEN je lance "lyremember register"
AND je saisis mon nom d'utilisateur, email et mot de passe
THEN un compte est créé
AND je reçois une confirmation
AND je peux me connecter avec ces identifiants
```

**Questions ouvertes :**
- Faut-il une validation d'email ?
- Politique de mot de passe (longueur minimale, complexité) ?

---

### US-1.2 : Se connecter
**En tant qu'** utilisateur inscrit  
**Je veux** me connecter à mon compte  
**Afin d'** accéder à mon répertoire personnel

**Critères d'acceptation :**
- [ ] L'utilisateur peut se connecter avec nom d'utilisateur + mot de passe
- [ ] Message d'erreur si identifiants incorrects
- [ ] La session reste active pendant l'utilisation de l'application
- [ ] L'utilisateur peut se déconnecter

**Scénarios de test :**
```
GIVEN j'ai un compte existant
WHEN je lance "lyremember login"
AND je saisis mes identifiants corrects
THEN je suis connecté
AND je peux accéder à mes chansons

GIVEN je suis connecté
WHEN je lance "lyremember logout"
THEN je suis déconnecté
AND je dois me reconnecter pour accéder à mes données
```

---

### US-1.3 : Lier un compte Genius
**En tant qu'** utilisateur  
**Je veux** lier mon compte Genius  
**Afin de** pouvoir importer facilement des paroles depuis Genius

**Critères d'acceptation :**
- [ ] L'utilisateur peut saisir son token API Genius
- [ ] Le token est stocké de manière sécurisée dans son profil
- [ ] Le système vérifie la validité du token
- [ ] L'utilisateur peut modifier ou supprimer son token

**Scénarios de test :**
```
GIVEN je suis connecté
WHEN je lance "lyremember link-genius <token>"
THEN mon token Genius est enregistré
AND je peux maintenant importer des chansons depuis Genius
```

**Notes techniques :**
- Token Genius : obtenu depuis https://genius.com/api-clients

---

## Epic 2 : Gestion du Répertoire Personnel

### US-2.1 : Ajouter une chanson au répertoire
**En tant qu'** utilisateur connecté  
**Je veux** ajouter des chansons à mon répertoire  
**Afin de** constituer ma collection personnelle

**Critères d'acceptation :**
- [ ] L'utilisateur peut ajouter une chanson manuellement (titre, artiste, paroles)
- [ ] L'utilisateur peut importer une chanson depuis Genius
- [ ] La chanson est ajoutée au répertoire de l'utilisateur uniquement
- [ ] L'utilisateur voit une confirmation

**Scénarios de test :**
```
GIVEN je suis connecté
WHEN je lance "lyremember add-to-repertoire <song-id>"
THEN la chanson est ajoutée à mon répertoire
AND elle apparaît dans ma liste "lyremember my-songs"

GIVEN je suis connecté
AND j'ai lié mon compte Genius
WHEN je cherche "lyremember search-genius 'Bohemian Rhapsody'"
AND je sélectionne un résultat
THEN la chanson est importée et ajoutée à mon répertoire
```

---

### US-2.2 : Voir mon répertoire
**En tant qu'** utilisateur connecté  
**Je veux** voir la liste de mes chansons  
**Afin de** choisir laquelle pratiquer

**Critères d'acceptation :**
- [ ] Commande "my-songs" affiche uniquement les chansons de l'utilisateur
- [ ] Liste affiche : titre, artiste, langue, statut de progression
- [ ] Possibilité de filtrer par langue
- [ ] Possibilité de rechercher dans mon répertoire

**Scénarios de test :**
```
GIVEN j'ai 5 chansons dans mon répertoire
WHEN je lance "lyremember my-songs"
THEN je vois mes 5 chansons avec leurs détails

GIVEN j'ai des chansons en français et anglais
WHEN je lance "lyremember my-songs --language fr"
THEN je vois uniquement mes chansons en français
```

---

### US-2.3 : Retirer une chanson du répertoire
**En tant qu'** utilisateur  
**Je veux** retirer des chansons de mon répertoire  
**Afin de** garder seulement celles qui m'intéressent

**Critères d'acceptation :**
- [ ] L'utilisateur peut retirer une chanson de son répertoire
- [ ] La chanson n'est plus visible dans "my-songs"
- [ ] La progression sur cette chanson est conservée (au cas où)
- [ ] Demande de confirmation avant retrait

---

## Epic 3 : Affichage Multi-Langues et Phonétique

### US-3.1 : Voir les paroles en version originale
**En tant qu'** utilisateur  
**Je veux** voir les paroles dans la langue originale  
**Afin de** apprendre la chanson telle qu'elle est chantée

**Critères d'acceptation :**
- [ ] Les paroles s'affichent dans la langue originale (VO)
- [ ] La langue est clairement indiquée
- [ ] Format lisible et claire (une ligne par ligne de chanson)

**Scénarios de test :**
```
GIVEN j'ai une chanson en japonais dans mon répertoire
WHEN je lance "lyremember view <song-id>"
THEN les paroles s'affichent en japonais (caractères originaux)
AND la langue "ja" est indiquée
```

---

### US-3.2 : Voir la traduction anglaise
**En tant qu'** utilisateur  
**Je veux** voir la traduction en anglais des paroles non-anglaises  
**Afin de** comprendre le sens de ce que je chante

**Critères d'acceptation :**
- [ ] Si la VO n'est pas en anglais, une traduction EN est affichée
- [ ] Option "--show-translation" ou affichage côte à côte
- [ ] Si pas de traduction dispo, possibilité de traduction automatique
- [ ] Traduction alignée ligne par ligne avec VO

**Scénarios de test :**
```
GIVEN j'ai une chanson en espagnol
WHEN je lance "lyremember view <song-id> --show-translation"
THEN je vois :
  Ligne 1 (ES): "Estas son las mañanitas"
  Ligne 1 (EN): "These are the morning songs"
  Ligne 2 (ES): "Que cantaba el Rey David"
  Ligne 2 (EN): "That King David used to sing"
  ...
```

**Options :**
- Format côte à côte vs ligne par ligne
- Traduction manuelle vs automatique (Google Translate API)

---

### US-3.3 : Voir la translittération phonétique
**En tant qu'** utilisateur  
**Je veux** voir la translittération phonétique des paroles  
**Afin de** savoir comment prononcer les mots, surtout pour les langues avec idéogrammes

**Critères d'acceptation :**
- [ ] Option "--show-phonetic" affiche la version phonétique
- [ ] Pour le japonais : affichage en romaji
- [ ] Pour le coréen : affichage en romanisation
- [ ] Pour le chinois : affichage en pinyin
- [ ] Pour l'arabe, russe, etc. : translittération latine
- [ ] Aligné avec les paroles originales

**Scénarios de test :**
```
GIVEN j'ai une chanson en japonais "千本桜"
WHEN je lance "lyremember view <song-id> --show-phonetic"
THEN je vois :
  Original: 千本桜
  Phonetic: Senbonzakura
  
  Original: 夜ニ紛レ君ノ声モ届カナイヨ
  Phonetic: Yoru ni magire kimi no koe mo todoka nai yo
```

**Notes techniques :**
- Utiliser epitran pour la translittération
- Utiliser pykakasi pour le japonais (kanji → romaji)
- Pour langues européennes : moins critique mais utile pour accentuation

---

### US-3.4 : Affichage combiné VO + Traduction + Phonétique
**En tant qu'** utilisateur avancé  
**Je veux** voir simultanément VO, traduction et phonétique  
**Afin d'** avoir toutes les informations en un coup d'œil

**Critères d'acceptation :**
- [ ] Option "--show-all" affiche les 3 versions
- [ ] Format clair et lisible (peut-être tableau)
- [ ] Possibilité de masquer/afficher chaque colonne

**Exemple d'affichage :**
```
╔═══════════════════════════════════════════════════════════════════════╗
║ Chanson: Sukiyaki (上を向いて歩こう) - Kyu Sakamoto                    ║
╠═══════════════════════════════════════════════════════════════════════╣
║ Original (JA)         │ Phonétique           │ Traduction (EN)        ║
╠═══════════════════════╪══════════════════════╪════════════════════════╣
║ 上を向いて歩こう      │ Ue wo muite arukou   │ Let's walk looking up  ║
║ 涙がこぼれないように  │ Namida ga koborenai  │ So the tears won't     ║
║                       │ you ni               │ fall                   ║
╚═══════════════════════╧══════════════════════╧════════════════════════╝
```

---

## Epic 4 : Mode Défilement Phrase par Phrase

### US-4.1 : Défilement automatique des paroles
**En tant qu'** utilisateur  
**Je veux** que les paroles défilent automatiquement phrase par phrase  
**Afin de** suivre le rythme de la chanson pendant que je chante

**Critères d'acceptation :**
- [ ] Mode "scroll" ou "karaoke" affiche une phrase à la fois
- [ ] Délai configurable entre chaque phrase (en secondes)
- [ ] Indicateur visuel de la phrase courante (surbrillance, couleur)
- [ ] Progression automatique jusqu'à la fin

**Scénarios de test :**
```
GIVEN j'ai une chanson avec 20 lignes
WHEN je lance "lyremember scroll <song-id> --speed 3"
THEN la ligne 1 s'affiche pendant 3 secondes
THEN la ligne 2 s'affiche pendant 3 secondes
...
THEN la chanson se termine après toutes les lignes
```

---

### US-4.2 : Contrôles de lecture
**En tant qu'** utilisateur  
**Je veux** contrôler le défilement (pause, play, vitesse)  
**Afin de** m'adapter à mon rythme d'apprentissage

**Critères d'acceptation :**
- [ ] Touche Espace : pause/play
- [ ] Touches flèches : ligne précédente/suivante
- [ ] Touches +/- : augmenter/diminuer vitesse
- [ ] Touche Q ou Esc : quitter
- [ ] Affichage des contrôles à l'écran

**Interface proposée :**
```
┌─────────────────────────────────────────────────┐
│         🎵 Mode Défilement Karaoke 🎵          │
├─────────────────────────────────────────────────┤
│                                                 │
│  [Phrase courante en grand, centrée]           │
│                                                 │
│  "Twinkle, twinkle, little star"                │
│                                                 │
├─────────────────────────────────────────────────┤
│ Ligne 3/20 │ Vitesse: 3s │ ⏸️ Pause            │
│ [Espace]:⏯️  [↑↓]:Nav  [±]:Vitesse  [Q]:Quitter │
└─────────────────────────────────────────────────┘
```

---

## Epic 5 : Mode Vérification Orale

### US-5.1 : Pratiquer en mode oral
**En tant qu'** utilisateur  
**Je veux** dire les paroles à voix haute et que l'application vérifie  
**Afin de** m'entraîner à chanter correctement

**Critères d'acceptation :**
- [ ] Mode "oral" active le microphone
- [ ] Une phrase s'affiche
- [ ] L'utilisateur lit la phrase à voix haute
- [ ] Le système capte l'audio et vérifie la correspondance
- [ ] Feedback immédiat (correct/incorrect)
- [ ] Possibilité de réécouter ce qu'on a dit

**Scénarios de test :**
```
GIVEN je lance "lyremember practice <song-id> --mode oral"
WHEN la phrase "Happy birthday to you" s'affiche
AND je dis "Happy birthday to you" dans le micro
THEN le système indique ✓ Correct!
AND passe à la phrase suivante

GIVEN la phrase "Twinkle twinkle little star" s'affiche
AND je dis "Twinkle little star" (incomplet)
THEN le système indique ✗ Incomplet
AND me montre ce qu'il a compris
AND me permet de réessayer
```

**Défis techniques :**
- Reconnaissance vocale (SpeechRecognition + Google Speech API)
- Gestion du bruit de fond
- Tolérance aux accents/prononciations

---

### US-5.2 : Réglages de sensibilité orale
**En tant qu'** utilisateur  
**Je veux** ajuster la sensibilité de la vérification orale  
**Afin de** avoir un niveau de difficulté adapté

**Critères d'acceptation :**
- [ ] Paramètre de tolérance : strict / moyen / permissif
- [ ] En mode permissif : 70% de similarité acceptée
- [ ] En mode strict : 90%+ requis
- [ ] Feedback sur le pourcentage de correspondance

---

## Epic 6 : Mode Phrases à Trous ("N'oubliez pas les paroles")

### US-6.1 : Pratiquer en mode "trous de fin de phrase"
**En tant qu'** utilisateur  
**Je veux** compléter la fin des phrases comme dans "N'oubliez pas les paroles"  
**Afin de** tester ma mémoire de façon ludique

**Critères d'acceptation :**
- [ ] Mode "noplp" (N'Oubliez Pas Les Paroles) disponible
- [ ] La phrase s'affiche complète sauf les derniers mots
- [ ] L'utilisateur doit taper les mots manquants
- [ ] Nombre de mots cachés configurable
- [ ] Vérification avec tolérance aux fautes de frappe

**Exemple :**
```
Phrase complète: "Twinkle twinkle little star, how I wonder what you are"

Affichage en mode NOPLP (3 mots cachés):
"Twinkle twinkle little star, how I wonder _____ _____ _____"

Utilisateur tape: "what you are"
→ ✓ Correct!

OU

Utilisateur tape: "what you're"
→ ✗ Presque! Bonne idée mais ce n'est pas "you're", c'est "you are"
```

**Options :**
- Nombre de mots à cacher (1-5)
- Position : fin de phrase (défaut) vs milieu vs aléatoire

---

### US-6.2 : Difficulté progressive en mode trous
**En tant qu'** utilisateur  
**Je veux** que la difficulté augmente progressivement  
**Afin de** être challengé au fur et à mesure

**Critères d'acceptation :**
- [ ] Commence avec 1 mot caché
- [ ] Augmente à 2, puis 3, etc. selon réussite
- [ ] Si échec, revient au niveau précédent
- [ ] Score basé sur niveau atteint

---

## Epic 7 : Mode Propositions Multiples (QCM)

### US-7.1 : Pratiquer en mode QCM
**En tant qu'** utilisateur  
**Je veux** choisir la bonne phrase parmi plusieurs propositions  
**Afin de** tester ma reconnaissance des paroles

**Critères d'acceptation :**
- [ ] Mode "quiz" ou "mcq" disponible
- [ ] Pour chaque ligne, 4 propositions affichées (A, B, C, D)
- [ ] Une seule est correcte
- [ ] Les 3 autres sont des variantes plausibles
- [ ] L'utilisateur sélectionne avec 1, 2, 3, 4 ou A, B, C, D
- [ ] Feedback immédiat avec explication

**Exemple :**
```
Quelle est la ligne suivante dans "Twinkle Twinkle" ?

A) How I wonder what you do
B) How I wonder what you are ✓
C) How I wonder where you are
D) How we wonder what you are

Votre réponse: B
→ ✓ Correct! "How I wonder what you are" est la bonne réponse.

Points: +10 | Combo: x2 🔥
```

---

### US-7.2 : Génération intelligente de fausses propositions
**En tant qu'** utilisateur  
**Je veux** que les fausses propositions soient crédibles  
**Afin que** le quiz soit réellement challengeant

**Critères d'acceptation :**
- [ ] Fausses réponses basées sur :
  - Mots similaires phonétiquement
  - Mots de la même chanson (autres lignes)
  - Variantes grammaticales
  - Erreurs communes
- [ ] Pas de propositions évidement fausses
- [ ] Difficulté ajustable

---

## Epic 8 : Fonctionnalités Transverses

### US-8.1 : Historique et statistiques par mode
**En tant qu'** utilisateur  
**Je veux** voir mes stats pour chaque mode de pratique  
**Afin de** identifier où je dois m'améliorer

**Critères d'acceptation :**
- [ ] Stats séparées pour : oral, trous, QCM, scroll
- [ ] Pourcentage de réussite par mode
- [ ] Temps passé par mode
- [ ] Chanson favorite / plus pratiquée

---

### US-8.2 : Recommandations personnalisées
**En tant qu'** utilisateur  
**Je veux** recevoir des suggestions de pratique  
**Afin d'** optimiser mon apprentissage

**Critères d'acceptation :**
- [ ] "lyremember recommend" suggère une chanson + mode
- [ ] Basé sur maîtrise actuelle
- [ ] Basé sur temps depuis dernière pratique
- [ ] Basé sur difficultés identifiées

---

## Priorités pour le MVP

### Must Have (P0)
- US-1.1, 1.2 : Comptes utilisateur (register/login)
- US-2.1, 2.2 : Répertoire personnel
- US-3.1, 3.3 : VO + Phonétique (au moins pour japonais/coréen)
- US-4.1, 4.2 : Défilement karaoke basique
- US-6.1 : Mode trous style NOPLP

### Should Have (P1)
- US-1.3 : Lien Genius
- US-3.2 : Traduction EN
- US-7.1 : Mode QCM
- US-5.1 : Vérification orale basique

### Could Have (P2)
- US-3.4 : Affichage combiné 3 vues
- US-5.2 : Réglages sensibilité orale
- US-6.2 : Difficulté progressive trous
- US-7.2 : Génération intelligente QCM
- US-8.1, 8.2 : Stats avancées et recommandations

---

## Questions pour validation

1. **Comptes utilisateur** : Faut-il un système de récupération de mot de passe ?
2. **Genius** : Token obligatoire ou optionnel ? Fallback si pas de token ?
3. **Phonétique** : Quelles langues sont prioritaires ? (JP, KR, ZH, AR, RU ?)
4. **Mode oral** : Service de reconnaissance vocale (Google gratuit vs payant vs offline) ?
5. **Données** : Les chansons sont partagées entre users ou chaque user a ses propres chansons ?
6. **Interface** : CLI uniquement ou prévoir interface web ?

---

## Estimations (story points)

- Epic 1 (Comptes): 13 pts
- Epic 2 (Répertoire): 8 pts
- Epic 3 (Multi-langues): 21 pts
- Epic 4 (Défilement): 13 pts
- Epic 5 (Oral): 21 pts
- Epic 6 (Trous): 8 pts
- Epic 7 (QCM): 13 pts
- Epic 8 (Transverse): 8 pts

**Total : ~105 story points**

MVP (P0) : ~55 pts
MVP + P1 : ~80 pts
