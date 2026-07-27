# QUIZ — ex02 : Types & Inférence

> 7 questions. Réponds **sans compiler**, puis déplie la correction.
> 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Quels sont les types inférés ?

```rust
let a = 5;
let b = 5.0;
let c = 5u8;
let d = 'a';
```

- [ ] **A.** `i64`, `f32`, `u8`, `char`
- [ ] **B.** `i32`, `f64`, `u8`, `char`
- [ ] **C.** `i32`, `f32`, `u8`, `&str`
- [ ] **D.** `usize`, `f64`, `u8`, `char`

<details><summary>Réponse</summary>

**B.** Les *fallbacks* d'inférence de Rust sont `i32` pour les entiers et `f64` pour les flottants — indépendamment de l'architecture. `'a'` est un `char` (4 octets, un scalaire Unicode), pas un octet : `b'a'` serait un `u8`.

</details>

---

### Q2. Ce code compile-t-il ?

```rust
let n = "42".parse().unwrap();
println!("{n}");
```

- [ ] **A.** Oui, `n` est un `i32` par défaut
- [ ] **B.** Non : `type annotations needed`
- [ ] **C.** Oui, `n` est une `String`
- [ ] **D.** Non : `parse` n'existe pas sur `&str`

<details><summary>Réponse</summary>

**B.** `parse::<T>()` est générique sur le type de sortie. Le `println!("{n}")` ne contraint rien (`Display` est implémenté par des dizaines de types), donc l'inférence n'a aucun point d'ancrage.

Deux façons de lever l'ambiguïté :
```rust
let n: i32 = "42".parse().unwrap();      // par annotation
let n = "42".parse::<i32>().unwrap();    // par turbofish
```

Le *fallback* `i32` ne s'applique **qu'aux littéraux** entiers, pas aux retours de fonctions génériques.

</details>

---

### Q3. 🔥 Ce code compile-t-il ?

```rust
fn total_score(score: i32, bonuses: [i32; 3]) -> i32 {
    score + bonuses.iter().sum::<i32>()
}

fn main() {
    let b = [5, 7, 3, 1];
    println!("{}", total_score(100, b));
}
```

- [ ] **A.** Oui, `total_score` ignore le 4ᵉ élément
- [ ] **B.** Non : `expected [i32; 3], found [i32; 4]`
- [ ] **C.** Oui, les arrays sont convertis automatiquement en slices
- [ ] **D.** Non : il faut passer `&b`

<details><summary>Réponse</summary>

**B.** **La taille fait partie du type.** `[i32; 3]` et `[i32; 4]` sont deux types totalement distincts, aussi différents que `i32` et `String`.

C'est la différence fondamentale avec une **slice** `&[i32]`, dont la taille est connue à l'exécution. Si tu veux accepter n'importe quelle taille :
```rust
fn total_score(score: i32, bonuses: &[i32]) -> i32 { ... }
total_score(100, &b);   // un array coerce en slice
```

</details>

---

### Q4. 🔥 Quelle est la sortie ?

```rust
let a = [1, 2, 3];
let b = a;
println!("{:?} {:?}", a, b);
```

- [ ] **A.** `[1, 2, 3] [1, 2, 3]`
- [ ] **B.** Erreur : `borrow of moved value: a`
- [ ] **C.** `[] [1, 2, 3]`
- [ ] **D.** Erreur : `{:?}` ne marche pas sur un array

<details><summary>Réponse</summary>

**A.** Un array `[T; N]` implémente `Copy` **si et seulement si** `T: Copy`. Ici `i32: Copy`, donc `let b = a` est une copie et `a` reste valide.

Le piège inverse, à garder en tête pour ex03 :
```rust
let a = [String::from("x")];
let b = a;
println!("{:?}", a);  // ❌ E0382 : String n'est pas Copy → l'array ne l'est pas non plus
```

</details>

---

### Q5. 🔥 Ce code compile-t-il ?

```rust
fn multiply(a: i32, b: i32) -> i32 {
    a * b;
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `expected i32, found ()`
- [ ] **C.** Oui, mais avec un warning
- [ ] **D.** Non : il manque le mot-clé `return`

<details><summary>Réponse</summary>

**B.** Le point-virgule transforme l'**expression** `a * b` en **instruction**, dont la valeur est `()` (le type unit). Le bloc de la fonction vaut donc `()`, alors que la signature promet `i32`.

Le compilateur est explicite : *`help: remove this semicolon to return this value`*. C'est l'erreur n°1 du débutant Rust, et elle reviendra dans tous les exercices — notamment dans les `impl Display` de la phase 2.

</details>

---

### Q6. 🔥 Quelle expression **ne compile pas** ?

```rust
let a = 5 / 2;        // (1)
let b = 5.0 / 2.0;    // (2)
let c = 5 / 2.0;      // (3)
let d = 5 as f64 / 2.0; // (4)
```

- [ ] **A.** (1) — division entière interdite
- [ ] **B.** (3) — pas de `Div<f64>` pour un entier
- [ ] **C.** (2) et (3)
- [ ] **D.** Aucune, tout compile

<details><summary>Réponse</summary>

**B.** Rust ne fait **aucune promotion numérique implicite**. `5 / 2.0` échoue : le littéral `5` devrait être un entier, mais aucun type entier n'implémente `Div<f64>`.

Les valeurs : `a == 2` (division entière tronquée, pas d'arrondi), `b == 2.5`, `d == 2.5`.

Attention : `let x = 5; let y: f64 = x;` échoue aussi. Toute conversion numérique est explicite (`as`, ou `From`/`TryFrom` pour les conversions sûres).

</details>

---

### Q7. Que vaut `t.1` et ce code compile-t-il ?

```rust
let t = ("Alice", 100, 3u8);
let (name, score, _) = t;
println!("{} {} {}", name, score, t.1);
```

- [ ] **A.** Erreur : `t` a été déplacé par le destructuring
- [ ] **B.** `Alice 100 100` — compile
- [ ] **C.** Erreur : on ne peut pas destructurer et réutiliser
- [ ] **D.** `Alice 100 3`

<details><summary>Réponse</summary>

**B.** Le destructuring d'un tuple dont **tous** les champs sont `Copy` (`&str`, `i32`, `u8`) est une copie : `t` reste utilisable. `t.1` vaut `100`.

Deux subtilités à retenir :
- l'accès tuple exige un **littéral** : `t.0` ✅, `t.i` avec `let i = 0` ❌ (l'index n'est pas une valeur runtime, c'est un nom de champ)
- si le tuple contenait une `String`, le destructuring en déplacerait le champ concerné et `t` deviendrait partiellement invalide

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 7/7 | Types & inférence maîtrisés, direction ex03 |
| 5-6/7 | Bon. Le point à sécuriser : l'absence totale de conversion implicite |
| < 5 | Rejoue Q3/Q5/Q6 dans ton `main.rs` et lis chaque message d'erreur en entier |
