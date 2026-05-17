# Workflow : nettoyage doc → audit → issues → tri des issues

Prompt agnostique réutilisable pour lancer le même workflow sur n'importe quel repo. À coller dans une nouvelle session Claude Code (ou équivalent agent).

---

## Prompt

Tu interviens sur un repo logiciel. Goal global : **partir d'une documentation potentiellement obsolète, l'aligner sur le code réel, et convertir les écarts en issues GitHub trackables**. Le workflow se décompose en 4 phases. Exécute-les **dans l'ordre**, en validant chaque phase avec l'utilisateur avant de passer à la suivante.

### Phase 1 — Nettoyage de la documentation

Objectif : la doc doit être lisible et déduplicée avant qu'on puisse l'auditer.

1. Liste tous les fichiers Markdown de design / planning / architecture (typiquement `*.md` à la racine + dossier `docs/`).
2. Identifie les **doublons** (deux fichiers couvrent le même sujet) et les **incohérences entre docs** (deux fichiers contredisent un même fait).
3. Propose à l'utilisateur :
   - Réorganisation des fichiers (par exemple : tout sauf le README à la racine vers `docs/`).
   - Suppression des docs obsolètes ou redondants.
   - Création d'un `docs/INDEX.md` thématique.
4. Applique les changements validés, commit + push avec un message du type `docs: reorganize design files into docs/ with INDEX`.

### Phase 2 — Audit doc ↔ code existant

Objectif : pour chaque promesse de la doc, vérifier qu'elle est tenue dans le code.

1. Parcours la doc nettoyée et liste **toutes les promesses vérifiables** : fonctionnalités annoncées, choix techniques (langages, libs, services externes), URLs / commandes / noms de fichiers cités.
2. Pour chaque promesse, **vérifie dans le code** :
   - Le fichier / module / classe existe-t-il vraiment ?
   - La lib citée est-elle dans le `package.json` / `Cargo.toml` / `requirements.txt` ?
   - Le service externe annoncé est-il celui réellement appelé ?
   - Les chiffres (nb de commandes, nb d'endpoints, etc.) correspondent-ils ?
3. Produis un **rapport d'audit structuré** :
   - Section "Décisions" : choix techniques observés vs annoncés.
   - Section "Écarts" : tableau `Promesse | Constat | Sévérité (high/medium/low) | Action recommandée (corriger doc / corriger code / arbitrer)`.
   - Section "Contradictions" : faits documentés qui se contredisent entre eux.
4. Pour les **arbitrages** (cas où ni la doc ni le code ne sont la vérité absolue), pose des questions à l'utilisateur **avant** de créer les issues.

### Phase 3 — Issues GitHub

Objectif : chaque écart non trivial devient une issue trackable.

1. Vérifie le format des issues existantes du repo (`gh issue list` ou MCP `list_issues`) pour reproduire les conventions : labels, sections (Contexte / Livrables / Hors périmètre / Critères d'acceptation / Dépendances), priorité (MVP / post-MVP), stack (frontend / backend / cross / ci).
2. Pour chaque écart d'audit qui nécessite du code :
   - **Une issue par écart**, titre en Conventional Commits (`feat(scope): …`, `fix(scope): …`, `docs(scope): …`).
   - Source de la promesse en référence (fichier `.md` + section).
   - Critères d'acceptation testables (pas "améliorer", mais "test X passe").
   - Découpage suggéré si > 1 jour de dev.
3. Pour les écarts purement documentaires (la doc ment, le code est OK) : corrige la doc dans des commits dédiés `docs: align X with reality (#NN)` sans créer d'issue.
4. Liste à la fin tous les numéros d'issues créés avec une matrice `# | Titre | Stack | Priorité`.

### Phase 4 — Tri / traitement des issues

Objectif : transformer le backlog en travail livré.

1. Trie les nouvelles issues par dépendance + complexité :
   - **Quick wins** (< 1h) en premier : utile pour valider le workflow et libérer du levier.
   - Items qui débloquent d'autres (par ex. infra lint avant audit code).
   - Lourdes en dernier (peuvent dépasser la session).
2. Pour chaque issue traitée :
   - Lis le code existant (`Read` / `Grep`).
   - Implémente le strict nécessaire (pas de refacto ou abstraction au-delà du livrable).
   - Run tests + lint locaux (Vitest, cargo test, eslint, etc. selon le repo).
   - Commit `type(scope): description (#NN)`.
   - Push sur la branche de travail.
   - Ferme l'issue avec un commentaire listant le commit + les livrables couverts + ce qui est explicitement hors scope.
3. **Arrête-toi proactivement** quand les issues restantes deviennent trop lourdes pour la session, et rends compte : "X/N traitées, voici l'état des Y restantes".

## Contraintes générales

- **Demande à l'utilisateur d'arbitrer** chaque décision contradictoire avant d'agir (pas d'arbitrage silencieux).
- **Pas d'over-engineering** : un quick fix n'a pas besoin d'un refactor d'accompagnement.
- **Conventional Commits** systématiquement, avec `(#NN)` quand une issue est liée.
- **CHANGELOG.md** mis à jour en section `[Unreleased]` à chaque livraison.
- **Branche dédiée** pour tout ce workflow (par ex. `chore/doc-audit-and-cleanup-YYYYMM`), jamais directement sur `main`.
- **Pas de PR sauf demande explicite** : laisse l'utilisateur ouvrir la PR finale.
- **Format de réponse** : updates courts sur ce que tu fais, listes / tableaux pour les rapports d'audit, pas de blabla narratif.

## Livrables attendus à chaque phase

| Phase | Livrable principal |
|---|---|
| 1 | Commit `docs:` qui réorganise + crée INDEX |
| 2 | Rapport d'audit (rendu inline OU fichier `docs/audits/YYYY-MM-audit.md`) |
| 3 | N issues GitHub ouvertes, matrice récap |
| 4 | M commits livrés sur la branche + M issues fermées avec commentaire |
