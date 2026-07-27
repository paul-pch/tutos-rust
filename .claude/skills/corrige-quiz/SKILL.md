---
name: corrige-quiz
description: Corrige un QUIZ.md d'exercice dont l'apprenant a coché les réponses (`- [x]`) — score, tableau récapitulatif, et explication enseignante de chaque erreur. À utiliser quand l'apprenant dit « corrige mon quiz », « j'ai répondu au QCM », « relis le quiz de exXX », « note-moi », ou colle un QCM rempli.
---

# Corrige-quiz — relecture d'un QCM répondu

Les `QUIZ.md` du dépôt sont des QCM à cases à cocher. L'apprenant coche `- [x]` une option par
question, sans déplier les `<details>`. Ce skill relit le fichier tel quel, corrige, et **explique**.

L'objectif n'est pas la note : c'est de rendre lisible **le modèle mental erroné** qui a produit
chaque mauvaise réponse. Une erreur de QCM est un symptôme, pas une faute.

## Localiser le quiz

1. Argument explicite (`/corrige-quiz ex14`) → `exercises/<phase>/<exXX-*>/QUIZ.md`.
2. Sinon, le `QUIZ.md` modifié le plus récemment contenant au moins un `- [x]`.
3. Aucun `- [x]` nulle part → le dire, ne rien inventer.
4. QCM collé dans la conversation plutôt que lu sur disque : corriger le texte collé, mais lire
   quand même le `QUIZ.md` d'origine pour disposer du corrigé de référence.

## Lire les réponses

Le format est stable dans tout le dépôt :

```markdown
### Q3. 🔥 <énoncé>

- [ ] **A.** …
- [x] **B.** …          ← la réponse de l'apprenant

<details><summary>Réponse</summary>

**B.** <corrigé de référence>
```

- La réponse attendue est la **première lettre en gras** ouvrant le bloc `<details>`.
- `🔥` dans le titre = question tricky, à traiter avec plus d'égards dans les deux sens :
  ratée, elle est normale ; réussie, elle mérite une ligne de consolidation.
- **Zéro case cochée** sur une question → *non répondue*. Ne compte ni juste ni faux, la
  signaler à part : c'est souvent l'aveu le plus utile du lot.
- **Plusieurs cases cochées** → hésitation. Compter faux, mais traiter le vrai sujet :
  quelle distinction manque pour trancher entre ces deux options précises.

## Structure de la correction

### 1. Le score
`N/M`, suivi du verdict correspondant lu dans la table **Auto-évaluation** en bas du `QUIZ.md`
— elle est écrite pour ce quiz, l'utiliser plutôt que d'improviser un commentaire.

### 2. Le tableau
Une ligne par question, dans l'ordre. Compact :

| Q | Ta réponse | Attendue | |
|---|---|---|---|
| Q1 | B | B | ✅ |
| Q3 🔥 | A | C | ❌ |
| Q7 | — | B | ⚠️ non répondue |

### 3. Les erreurs, une par une
C'est le corps de la correction. Pour chaque ❌, trois temps :

**a. Pourquoi ton choix était séduisant.** Commencer par là, toujours. Un distracteur de ce
dépôt est rarement absurde : il encode une croyance plausible (« `Vec<Box<T>>` est nécessaire à
la récursion », « `mut` autorise le changement de type »). Nommer cette croyance explicitement —
c'est elle qu'on corrige, pas la lettre cochée.

**b. Le mécanisme réel, montré en code.** Ce que fait vraiment Rust, et **pourquoi** — quel bug
la règle empêche, quel invariant elle protège. Le bloc `<details>` donne la réponse ; il ne
suffit pas de la recopier. La correction doit aller là où le corrigé s'arrête : le corrigé
répond à la question, la correction répond à *l'erreur*.

**Chaque erreur reçoit son exemple de code commenté** — voir *Les exemples de code* ci-dessous.
C'est le format qui fait comprendre l'apprenant : si une erreur est expliquée en prose seule,
l'explication est incomplète. Idéalement l'exemple met en regard ce que ton option cochée
décrivait et ce que Rust fait réellement.

**c. L'ancrage.** Rattacher à son code réel quand c'est possible — le `src/main.rs` de
l'exercice, un exercice antérieur du tableau de `preprompt.md`, ou l'erreur du compilateur qu'il
verrait s'il écrivait ça. Une ligne, pas un paragraphe.

Si une erreur mérite un développement complet — plusieurs exemples, un déroulé ligne à ligne du
raisonnement du compilateur — le dire et basculer sur le skill **`explique`**, plutôt que
d'étouffer le reste de la correction.

### 4. Le fil rouge
2-4 lignes de synthèse, seulement si les erreurs dessinent un motif : toutes sur les emprunts,
toutes sur des questions 🔥, toutes sur du typage d'`Option`/`Result`. Un motif vaut plus que
la somme des erreurs. Terminer par la relecture concrète à faire (« relis la section Concept
d'ex14 sur les fat pointers ») — pas par un encouragement générique.

### 5. Les 🔥 réussies
Optionnel, une ligne chacune, seulement si le quiz est globalement bon. « Q3 juste : tu as évité
le piège du fat pointer. » Rien de plus.

## Les exemples de code

**C'est par là que l'apprenant comprend.** Une correction sans code est une correction à moitié
faite. Chaque erreur du QCM doit repartir avec un bout de code court et **commenté ligne à
ligne** — priorité au code montré sur le paragraphe qui le décrit.

- **Commenter la ligne, pas le bloc** : ce que le compilateur *voit* à cette ligne précise.
- **Le type en commentaire dès qu'il n'est pas évident** : `// → Option<&Rc<Node>>`.
- **Marquer où ça casse** : `// ← ici : ...`, `// ERREUR E0502 : ...`.
- **Les deux versions côte à côte** quand la question portait sur une frontière — la variante
  que ton option décrivait, et celle que Rust applique vraiment.
- **Types neutres** (`Stack<T>`, `Point`, `Container<K, V>`) : jamais les types de l'exercice.
  C'est ce qui permet de commenter généreusement sans livrer la solution.
- 5 à 15 lignes. Au-delà, l'exemple illustre deux choses — le couper.

```rust
let mut stack = vec![1, 2, 3];
let first = &stack[0];      // emprunt PARTAGÉ, vivant tant que `first` est lu plus bas
stack.push(4);              // ← ici : push veut &mut self, mais l'emprunt partagé court encore
println!("{first}");        // ERREUR E0502 — c'est cette ligne qui prolonge la vie de `first`
```

Les tableaux comparatifs des `QUIZ.md` (coûts, allocations, layouts) restent bienvenus **en plus**
du code — jamais à sa place.

## Règles

- **Vérifier avant d'affirmer.** Toute affirmation testable — une taille, un type, « ça compile /
  ça panique » — se vérifie au compilateur (`cargo check`, un scratch `rustc`, `size_of`) avant
  d'être écrite. Le dire quand ça a été fait.
- **Le corrigé de référence n'est pas sacré.** Si la réponse de l'apprenant est défendable, ou si
  l'énoncé est ambigu, le vérifier au compilateur et l'écrire noir sur blanc : « tu as raison, la
  question est mal posée ». Compter le point. Corriger l'énoncé du `QUIZ.md` dans la foulée.
- **Pas de code solution de l'exercice.** La règle du dépôt tient aussi ici : illustrer sur des
  types neutres (`Stack<T>`, `Point`, `Container<K, V>`), jamais sur la struct que l'apprenant
  doit écrire.
- **Français, tutoiement**, dense, ton d'instructeur — le registre des `QUIZ.md` eux-mêmes.
- Pas de flatterie, pas de « bravo », pas de « bonne question ». Un score bas se commente
  factuellement.
- Ne jamais modifier les cases cochées en corrigeant.

## Après la correction

1. Écrire l'intégralité dans `exercises/<phase>/<exXX-nom>/QUIZ-CORRECTION.md` (écraser s'il
   existe : c'est la dernière tentative qui compte). Ne pas toucher au `CORRECTION.md`, réservé
   aux relectures de code.
2. Proposer — sans le faire d'office — de **remettre les cases à zéro** (`- [x]` → `- [ ]`) pour
   une seconde tentative, une fois les explications lues.
