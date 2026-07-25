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

Lis d'abord le README de l'exercice, puis écris le code :

```sh
# Lancer l'exercice
cargo run -p ex01-variables

# Lancer les tests (vérifie que l'output correspond au README)
cargo test -p ex01-variables
```

**Phases 1 et 2** — tout tient dans `src/main.rs`, et `tests/output.rs` vérifie le stdout
au byte près.

**Phase 3 et suivantes** — l'exercice est découpé en une lib et un binaire :

```
exXX-nom/
├── src/lib.rs      # la logique, avec ses tests unitaires
├── src/main.rs     # mince : appelle la lib et affiche
├── tests/api.rs    # tests d'intégration sur l'API publique de la lib
├── tests/output.rs # assertion sur le binaire
└── benches/        # benchmarks criterion (exercices de perf)
```

La lib s'importe sous le nom du package avec des underscores : `use ex17_threads::...`.
Le détail de ce format est expliqué dans [`exercises/phase3/README.md`](exercises/phase3/README.md).

```sh
cargo test -p ex17-threads --lib    # seulement les tests unitaires de lib.rs
cargo test -p ex17-threads          # tout : lib, api, output, doctests
cargo bench -p ex17-threads         # les benchmarks
```

## Commandes utiles

```sh
make            # setup + lint + tests (tout en une fois)
make test       # lance tous les tests
make bench      # lance les benchmarks
make lint       # fmt + clippy sur le workspace
make doc        # génère et ouvre la doc
make new name=ex17-foo phase=phase3          # crée un nouvel exercice
make new name=ex22-bench phase=phase3 bench=1  # ... avec un squelette de benchmark
```
