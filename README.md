# tutos-rust

Exercices progressifs pour apprendre Rust. Chaque exercice couvre un concept isolé et définit un output attendu à reproduire.


## Prérequis

```sh
# Installe rustup si absent
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Installe les composants et les hooks git
make setup
```

## Travailler sur un exercice

Lis d'abord le README de l'exercice, puis édite son `src/main.rs` :

```sh
# Lancer l'exercice
cargo run -p ex01-variables

# Lancer les tests (vérifie que l'output correspond au README)
cargo test -p ex01-variables
```

## Commandes utiles

```sh
make            # setup + lint + tests (tout en une fois)
make test       # lance tous les tests
make lint       # fmt + clippy sur le workspace
make new name=ex05-foo   # crée un nouvel exercice
```
