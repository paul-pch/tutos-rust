# Phase 3 — le nouveau format d'exercice

À partir d'ici, un exercice n'est plus un simple binaire : c'est une **bibliothèque testable**
accompagnée d'un **binaire mince**. Ce document explique ce qui change et pourquoi.

## Pourquoi

Le format des phases 1 et 2 tenait en deux fichiers : `src/main.rs` et un `tests/output.rs` qui
comparait le stdout au byte près. C'est parfait pour un concept déterministe — mais ça bloque
dès qu'on veut :

- **mesurer** quelque chose : un benchmark n'a pas de « stdout attendu » ;
- **paralléliser** : l'ordre d'exécution des threads n'est pas reproductible ;
- **tester une règle métier** sans lancer un processus complet et parser sa sortie ;
- **réutiliser** du code : rien n'est importable depuis un `main.rs`.

Le découpage lib/binaire lève ces quatre limites d'un coup.

## Package, crate, target

Un **package** (un `Cargo.toml`) peut contenir plusieurs **crates** — les unités de compilation
de Rust. Cargo les détecte par convention :

| Fichier | Crate produite | Rôle |
|---|---|---|
| `src/lib.rs` | crate *library* | la logique, importable |
| `src/main.rs` | crate *binary* | l'exécutable |
| `tests/*.rs` | une crate de test **par fichier** | tests d'intégration |
| `benches/*.rs` | une crate de bench par fichier | benchmarks |

Point important : ce sont des crates **séparées**. Le binaire ne « voit » pas le contenu de
`lib.rs` par magie — il en dépend, exactement comme d'une crate téléchargée depuis crates.io.

```rust
// src/main.rs
use ex17_threads::Probe;   // la lib est une dépendance comme une autre
```

### Le nom de la lib

Le nom du package (`ex17-threads`) contient un tiret, ce qui est interdit dans un identifiant
Rust. Cargo le convertit en underscore : la lib s'importe sous `ex17_threads`. C'est déclaré
explicitement dans le `Cargo.toml` pour que ce soit visible :

```toml
[lib]
name = "ex17_threads"
path = "src/lib.rs"
```

### Conséquence directe : `pub` devient obligatoire

En phase 2, `pub` servait à traverser des `mod` à l'intérieur d'un même fichier. Ici, il faut le
`pub` pour franchir la frontière entre deux crates. Tout ce que `main.rs` utilise doit être
public dans `lib.rs` — struct **et** champs **et** méthodes.

```rust
// src/lib.rs
pub struct Probe {          // sans `pub` : invisible depuis main.rs
    pub name: String,       // une struct pub ne rend pas ses champs pub
    latency_ms: u64,        // volontairement privé → seule la lib le manipule
}

impl Probe {
    pub fn new(name: &str, latency_ms: u64) -> Self { /* ... */ }
    pub fn run(&self) -> ProbeResult { /* ... */ }
}
```

C'est le vrai bénéfice pédagogique du découpage : il t'oblige à **dessiner une API**. Ce que tu
exposes, tu t'engages à le maintenir ; ce que tu gardes privé, tu peux le refactorer librement.

## `main.rs` reste mince

Règle de la phase 3 : `main.rs` lit l'entrée, appelle la lib, affiche. Rien d'autre.

```rust
fn main() {
    let probes = ex17_threads::default_probes();
    let results = ex17_threads::check_parallel(&probes);
    for result in &results {
        println!("{result}");
    }
}
```

Le test à t'appliquer : *si une règle métier n'est atteignable qu'en lançant le binaire et en
parsant sa sortie, elle est au mauvais endroit.* Elle doit descendre dans `lib.rs`.

## Les quatre niveaux de test

C'est la partie qui change le plus par rapport aux phases 1 et 2.

### 1. Tests unitaires — dans `src/lib.rs`

Ils vivent **dans** le fichier qu'ils testent, donc ils voient les éléments **privés**. C'est là
que vont les cas limites et la logique fine.

```rust
#[cfg(test)]              // compilé uniquement par `cargo test`
mod tests {
    use super::*;         // importe tout le contenu du module parent

    #[test]
    fn up_count_on_empty_list() {
        assert_eq!(up_count(&[]), 0);
    }
}
```

### 2. Tests d'intégration — `tests/api.rs`

Une crate séparée : elle ne voit que l'**API publique**, exactement comme un utilisateur de ta
lib. Si un test d'intégration ne compile pas, c'est souvent qu'il manque un `pub`.

```rust
use ex17_threads::{Probe, check_parallel};

#[test]
fn results_keep_input_order() { /* ... */ }
```

### 3. Test de l'exécutable — `tests/output.rs`

Il lance le binaire compilé et assert sur sa sortie. Il vérifie le **câblage** (main appelle
bien la lib et formate correctement), pas la logique.

### 4. Doctests — les exemples dans les commentaires `///`

Les blocs de code des commentaires de doc sont compilés et exécutés par `cargo test`. Une doc
fausse devient donc un test rouge. Ils ne fonctionnent que sur la crate lib.

### Quoi lancer

```sh
cargo test -p ex17-threads --lib          # seulement les tests unitaires
cargo test -p ex17-threads --test api     # seulement tests/api.rs
cargo test -p ex17-threads --doc          # seulement les doctests
cargo test -p ex17-threads                # les quatre niveaux
```

En cas d'échec, `cargo test -p ex17-threads --lib -- --nocapture` laisse passer les `println!`.

## Assertions : exactes ou par prédicat

Un exercice qui mesure du temps ne peut pas avoir de sortie figée. `tests/output.rs` bascule
alors sur des prédicats, qui vérifient le **format** et les **invariants** plutôt que la valeur :

```rust
use predicates::prelude::*;

.stdout(
    predicate::str::contains("=== Parallel ===")
        .and(predicate::str::is_match(r"(?m)^  total: \d+ms$").unwrap()),
)
```

Le réflexe à prendre : **l'ordre d'arrivée des threads n'est pas un invariant**, l'ordre de
collecte des résultats en est un. On assert le second, jamais le premier.

## Benchmarks

Les exercices de perf sont créés avec `make new … bench=1`, ce qui ajoute `benches/bench.rs` et
la dépendance `criterion`. Criterion remplace le harness de test standard — d'où le
`harness = false` dans le `Cargo.toml` — et produit un rapport avec écart-type et détection de
régression.

```sh
cargo bench -p ex22-bench
```

Un benchmark ne se lance jamais en `--debug` : `cargo bench` compile en profil optimisé,
sinon les chiffres ne veulent rien dire.

## Créer un exercice

```sh
make new name=ex18-channels phase=phase3            # squelette lib + bin + tests
make new name=ex22-bench    phase=phase3 bench=1    # ... avec un squelette criterion
```

Le squelette généré ne contient **aucune logique** : `lib.rs` n'a que son `mod tests` vide,
`main.rs` affiche `Hello, world!`, et `tests/output.rs` attend une sortie vide. C'est à toi
d'écrire le reste, tests compris — le README de l'exercice dit *quels* cas couvrir, pas
*comment*.

## Ce qui ne change pas

- Le README de l'exercice reste la spec : Concept, Exercice, Output attendu, Pistes.
- `make lint` (fmt + clippy `-D warnings`) et `make test` restent les garde-fous.
- Les phases 1 et 2 gardent leur ancien format : un binaire, un `tests/output.rs`. Elles ne
  seront pas reprises.

## Récapitulatif

| | Phases 1-2 | Phase 3+ |
|---|---|---|
| Fichiers source | `src/main.rs` | `src/lib.rs` + `src/main.rs` |
| Visibilité | `pub` entre modules | `pub` entre crates — API explicite |
| Tests | `tests/output.rs` seul | unitaires + intégration + output + doctests |
| Assertions | stdout byte-exact | exactes **ou** par prédicat |
| Dépendances | std uniquement | std en phase 3, puis l'écosystème |
| Mesure | aucune | benchmarks criterion |
