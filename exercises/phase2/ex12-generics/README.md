# ex12-generics — Monitoring générique

## Concept

### Generics & contraintes `where`

Un type générique est paramétré par un ou plusieurs types inconnus à l'écriture :

```rust
struct Bucket<T> {
    name: String,
    items: Vec<T>,
}
```

Les **contraintes** limitent ce que `T` doit implémenter. On peut les écrire en ligne ou via `where` (préférable quand elles sont nombreuses) :

```rust
// en ligne
impl<T: Display + PartialOrd> Bucket<T> { ... }

// via where — plus lisible
impl<T> Bucket<T>
where
    T: Display + PartialOrd,
{ ... }
```

Les fonctions génériques suivent la même logique :

```rust
fn highest<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a >= b { a } else { b }
}
```

Avec **plusieurs paramètres** de type :

```rust
struct Pair<A, B> { first: A, second: B }

impl<A, B> Pair<A, B> {
    fn swap(self) -> Pair<B, A> {
        Pair { first: self.second, second: self.first }
    }
}
```

---

## Exercice

Construis un petit système de monitoring générique. Tout dans `src/main.rs`.

### Étape 1 — `Measurement<T>`

Définis une struct `Measurement<T>` avec les champs :
- `label: String`
- `value: T`

Implémente `Measurement::new(label: &str, value: T) -> Self`.

Implémente `Display` pour `Measurement<T>` où `T: Display`, au format :
```
web-server = 87.3
```

### Étape 2 — `Bucket<T>`

Définis une struct `Bucket<T>` avec les champs :
- `name: String`
- `items: Vec<Measurement<T>>`

Implémente :
- `Bucket::new(name: &str) -> Self`
- `add(&mut self, m: Measurement<T>)`
- `len(&self) -> usize`

Puis, dans un bloc `impl` séparé avec contrainte `where T: PartialOrd` :
- `max_value(&self) -> Option<&Measurement<T>>` — measurement avec la valeur la plus haute
- `min_value(&self) -> Option<&Measurement<T>>` — measurement avec la valeur la plus basse
- `count_above(&self, threshold: &T) -> usize` — nombre de measurements strictement au-dessus du seuil

### Étape 3 — `Pair<A, B>`

Définis une struct `Pair<A, B>` avec les champs `first: A` et `second: B`.

Implémente :
- `Pair::new(first: A, second: B) -> Self`
- `swap(self) -> Pair<B, A>` — consomme la paire et renvoie une nouvelle avec les types inversés
- `Display` pour `Pair<A, B>` où `A: Display, B: Display`, au format : `(web-server, 87.3)`

### Étape 4 — `fn summary<T>`

Écris une fonction `summary<T>(bucket: &Bucket<T>)` qui affiche :

```
=== CPU Usage (%) ===
5 measurements
Max: web-server = 87.3
Min: cache = 12.1
Above 50.0: 3
```

Contrainte sur `T` : `Display + PartialOrd`. Le seuil est passé en paramètre : `fn summary<T>(bucket: &Bucket<T>, threshold: &T)`.

### Étape 5 — `main`

1. Crée un `Bucket<f64>` nommé `"CPU Usage (%)"` et ajoute ces 5 measurements :

   | label      | value |
   |------------|-------|
   | web-server | 87.3  |
   | database   | 72.1  |
   | proxy      | 51.5  |
   | cache      | 12.1  |
   | worker     | 34.8  |

2. Crée un `Bucket<u64>` nommé `"Memory (bytes)"` et ajoute ces 4 measurements :

   | label      | value         |
   |------------|---------------|
   | database   | 8_589_934_592 |
   | app-server | 524_288_000   |
   | logs       | 268_435_456   |
   | cache      | 134_217_728   |

3. Appelle `summary` sur chaque bucket avec pour seuil `50.0` (CPU) et `1_000_000_000` (mémoire).

4. Affiche `--- Pairs ---`, puis :
   - Crée `Pair::new("web-server", 87.3_f64)`, affiche-le, puis affiche sa version swappée
   - Crée `Pair::new("database", 8_589_934_592_u64)`, affiche-le, puis affiche sa version swappée

---

## Output attendu

```
=== CPU Usage (%) ===
5 measurements
Max: web-server = 87.3
Min: cache = 12.1
Above 50.0: 3

=== Memory (bytes) ===
4 measurements
Max: database = 8589934592
Min: cache = 134217728
Above 1000000000: 1

--- Pairs ---
(web-server, 87.3)
Swapped: (87.3, web-server)
(database, 8589934592)
Swapped: (8589934592, database)
```

---

## Pistes

- `PartialOrd` (et non `Ord`) car `f64` n'implémente pas `Ord` (à cause de `NaN`). Utilise `.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)` ou compare avec `>=` directement.
- `max_value` et `min_value` : utilise `.iter().reduce(|acc, m| if m.value >= acc.value { m } else { acc })`.
- `count_above` : `.iter().filter(|m| m.value > *threshold).count()`.
- Pour `swap`, le compilateur infère les types — tu n'as pas besoin d'annoter.
- Deux blocs `impl<T>` séparés sur `Bucket<T>` : l'un sans contrainte (constructeur, add, len), l'autre avec `where T: PartialOrd` (max, min, count_above). C'est valide et idiomatique.

---

## Lancer l'exercice

```sh
cargo run -p ex12-generics
cargo test -p ex12-generics
```
