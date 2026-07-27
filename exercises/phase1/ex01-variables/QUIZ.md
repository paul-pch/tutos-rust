# QUIZ — ex01 : Variables & Immutabilité

> 7 questions. Réponds **sans compiler**, puis déplie la correction.
> 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Que se passe-t-il ?

```rust
let mut x = 5;
x = "hello";
```

- [ ] **A.** Compile : `mut` autorise le changement de valeur *et* de type
- [ ] **B.** Erreur de compilation : `expected integer, found &str`
- [ ] **C.** Compile avec un warning
- [ ] **D.** Compile : `x` devient un `&str` par inférence

<details><summary>Réponse</summary>

**B.** `mut` autorise la **mutation**, pas le **changement de type**. Le type de `x` a été figé à `i32` par l'inférence au moment du `let`. Seul le **shadowing** (`let x = "hello";`) permet de changer de type.

</details>

---

### Q2. Quelle est la sortie ?

```rust
let x = 5;
let x = x + 1;
{
    let x = x * 2;
    println!("{x}");
}
println!("{x}");
```

- [ ] **A.** `12` puis `12`
- [ ] **B.** `12` puis `6`
- [ ] **C.** `12` puis `5`
- [ ] **D.** Erreur : `x` est immutable

<details><summary>Réponse</summary>

**B.** Le shadowing est **scopé**. Le `let x = x * 2` du bloc interne crée une 3ᵉ variable qui masque la précédente **uniquement jusqu'à la fin du bloc**. À la sortie, on retrouve le `x` valant `6`.

</details>

---

### Q3. 🔥 Ce code compile-t-il ?

```rust
let x: i32;
if std::env::args().count() > 1 {
    x = 1;
} else {
    x = 2;
}
println!("{x}");
```

- [ ] **A.** Non : `x` n'est pas `mut`, on ne peut pas l'assigner
- [ ] **B.** Non : `x` doit être initialisé au `let`
- [ ] **C.** Oui : l'initialisation différée est légale si `x` est assigné **exactement une fois** sur chaque chemin
- [ ] **D.** Oui, mais seulement avec `let mut x`

<details><summary>Réponse</summary>

**C.** C'est le *deferred initialization*. Rust suit les chemins de contrôle : chaque branche assigne `x` une et une seule fois, donc l'immutabilité est respectée — `x` n'est jamais *ré*-assigné. Ajouter `x = 3;` après le `if` casserait tout (`cannot assign twice to immutable variable`).

Utile pour éviter un `mut` inutile ou une valeur bidon type `let mut x = 0;`.

</details>

---

### Q4. 🔥 Quelle affirmation est **fausse** ?

- [ ] **A.** `const MAX: u32 = 100;` exige l'annotation de type explicite
- [ ] **B.** `const` peut être déclaré à n'importe quelle portée, y compris globale
- [ ] **C.** `let` peut être déclaré au niveau global (hors fonction)
- [ ] **D.** `const` est inliné à chaque usage, il n'a pas d'adresse mémoire stable

<details><summary>Réponse</summary>

**C** est fausse. `let` n'existe qu'à l'intérieur d'une fonction. Au niveau global, tu n'as que `const` (inliné) et `static` (une seule adresse mémoire, durée de vie `'static`).

Note : `const` ne peut jamais être `mut`, et sa valeur doit être calculable à la compilation.

</details>

---

### Q5. 🔥 Que fait ce programme ?

```rust
fn lose_life(lives: u8) -> u8 {
    lives - 1
}

fn main() {
    println!("{}", lose_life(0));
}
```

- [ ] **A.** Affiche `-1`
- [ ] **B.** Affiche `255`
- [ ] **C.** Panique en debug, affiche `255` en release
- [ ] **D.** Erreur de compilation

<details><summary>Réponse</summary>

**C.** C'est **le** piège des entiers non signés en Rust.

- `cargo run` (profil **debug**) : les checks d'overflow sont actifs → `panicked at 'attempt to subtract with overflow'`
- `cargo run --release` : les checks sont désactivés par défaut → arithmétique **wrapping** → `255`

Même programme, deux comportements. C'est pour ça qu'existent `checked_sub` (→ `Option`), `saturating_sub` (→ plancher à 0) et `wrapping_sub` (→ explicite). Dans l'exercice, `lives: u8` valant `3` puis `-1` est sûr, mais le réflexe doit être là.

</details>

---

### Q6. Dans l'énoncé, l'étape 4 demande de transformer `score` (`i32`) en `String`. Pourquoi le shadowing est-il **obligatoire** ici et pas un simple `mut` ?

- [ ] **A.** Parce que `String` est alloué sur le tas
- [ ] **B.** Parce que `mut` ne permet pas de changer le type d'une variable
- [ ] **C.** Parce que `score` était déclaré `const`
- [ ] **D.** Parce que `format!` retourne une valeur immutable

<details><summary>Réponse</summary>

**B.** `let mut score: i32` reste un `i32` pour toujours. `let score = format!("Score: {score}");` crée une **nouvelle variable** de type `String` qui masque l'ancienne. C'est précisément le cas d'usage canonique du shadowing : convertir une valeur en gardant le même nom.

</details>

---

### Q7. 🔥 Combien de warnings clippy/rustc sur ce code ?

```rust
let mut score = 10;
let lives = 3;
println!("{}", score);
```

- [ ] **A.** 0
- [ ] **B.** 1 — `variable does not need to be mutable`
- [ ] **C.** 2 — le `mut` inutile **et** `lives` jamais utilisé
- [ ] **D.** 1 — `lives` jamais utilisé

<details><summary>Réponse</summary>

**C.** Deux lints distincts :
- `unused_mut` : `score` n'est jamais réassigné
- `unused_variables` : `lives` n'est jamais lu → suggestion de le renommer `_lives`

Les deux sont des **warnings**, pas des erreurs : le code compile. Le préfixe `_` est la façon idiomatique de dire « je sais, c'est volontaire ».

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 7/7 | Tu peux passer à ex02 les yeux fermés |
| 5-6/7 | Solide. Relis la section ratée |
| < 5 | Relis le README et rejoue avec le compilateur : modifie ton `main.rs` pour reproduire chaque piège |
