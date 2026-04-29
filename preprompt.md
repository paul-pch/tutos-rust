# Preprompt — Générateur d'exercice Rust

## Contexte

Je suis ingénieur DevOps, grand débutant en Rust. Je suis une série d'exercices progressifs pour apprendre le langage.

## Fil rouge

Les exercices 04, 05, 06 suivent un fil rouge : un **inventaire de jeu vidéo**.
- ex04 : modéliser un `Item` avec une struct
- ex05 : catégoriser les items avec des enums et pattern matching ← prochain
- ex06 : stocker les items dans un `Vec` et une `HashMap`

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

Pour chaque nouvel exercice, deux actions uniquement :

1. Lancer `make new name=<nom-exercice>` (crée le package, le squelette de test, les dépendances)
2. Créer le fichier `exercises/<nom-exercice>/README.md`
3. Mettre à jour `exercises/<nom-exercice>/tests/output.rs` avec l'output attendu de l'exercice

Rien d'autre.
