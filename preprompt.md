# Preprompt — Générateur d'exercice Rust

## Contexte

Je suis ingénieur DevOps, grand débutant en Rust. Je suis une série d'exercices progressifs pour apprendre le langage.

## Exercices déjà réalisés

| # | Nom | Concepts |
|---|-----|----------|
| 01 | ex01-variables | `let`, `let mut`, shadowing, types scalaires |
| 02 | ex02-types | inférence, tuples, arrays, fonctions, turbofish |
| 03 | ex03-ownership | move, copy, borrowing, `&T`, `&mut T` |
| 04 | ex04-structs | struct, impl, `&self`, `&mut self`, méthodes associées |
| 05 | ex05-enums | enums, pattern matching, `match` |
| 06 | ex06-collections | `Vec`, `HashMap`, itération |

## Prochains concepts suggérés

- **Closures** — fonctions anonymes, capture de l'environnement, `Fn` / `FnMut` / `FnOnce`
- **Iterators** — `map`, `filter`, `fold`, chaîner des opérations sur des collections
- **Generics** — paramétrer structs et fonctions par des types, contraintes avec `where`
- **Modules & visibilité** — organiser son code avec `mod`, `pub`, `use`
- **Smart pointers** — `Box<T>`, `Rc<T>`, `RefCell<T>` — aller plus loin sur la gestion mémoire

## Règles de format du README

- Une section **Concept** avec les notions théoriques et des exemples de code commentés
- Une section **Exercice** avec des étapes numérotées claires
- Une section **Output attendu** avec le résultat exact à reproduire
- Une section **Pistes** avec des hints sans donner la solution
- Une section **Lancer l'exercice** avec la commande `cargo run -p <nom>`
- Pas de solution dans le README
- Pas de blocs de code superflus — seulement ce qui illustre le concept
- Concis : pas de paragraphes d'explication inutiles

## Règles pédagogiques

- Un seul concept principal par exercice
- S'appuyer sur les notions déjà vues sans les ré-expliquer
- Le compilateur Rust est un bon professeur : les exercices peuvent volontairement pousser à faire des erreurs de compilation instructives
- Pas de code fourni dans la consigne — l'apprenant écrit tout

## Création d'un exercice

Pour chaque nouvel exercice, quatre actions uniquement :

1. Lancer `make new name=<nom-exercice>` (crée le package, le squelette de test, les dépendances)
2. Créer le fichier `exercises/<nom-exercice>/README.md`
3. Mettre à jour `exercises/<nom-exercice>/tests/output.rs` avec l'output attendu de l'exercice
4. Enrichir le fichier `preprompt.md` avec le nouvel exercice

Rien d'autre.
