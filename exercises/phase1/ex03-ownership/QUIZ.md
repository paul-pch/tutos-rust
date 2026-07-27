# QUIZ — ex03 : Ownership

> 8 questions — l'exercice le plus important de la phase 1.
> 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Quelle est l'erreur ?

```rust
let name = String::from("Alice");
greet(name);
println!("{name}");

fn greet(n: String) { println!("Hello, {n}!"); }
```

- [ ] **A.** `cannot borrow name as mutable`
- [ ] **B.** `borrow of moved value: name`
- [ ] **C.** Aucune, ça compile
- [ ] **D.** `name does not live long enough`

<details><summary>Réponse</summary>

**B.** `greet(name)` prend l'ownership : la `String` est **déplacée** dans le paramètre `n`, qui est libéré à la fin de `greet`. Le `println!` emprunte une valeur qui n'existe plus.

Les 3 fixes possibles, par ordre de préférence :
1. `fn greet(n: &String)` + `greet(&name)` — ou mieux `&str` (voir Q7)
2. `greet(name.clone())` — coûteux, à éviter par réflexe
3. `fn greet(n: String) -> String` qui rend la valeur — verbeux

</details>

---

### Q2. 🔥 Ce code compile-t-il ?

```rust
let mut s = String::from("hi");
let r1 = &s;
println!("{r1}");
let r2 = &mut s;
r2.push('!');
println!("{s}");
```

- [ ] **A.** Non : `cannot borrow s as mutable because it is also borrowed as immutable`
- [ ] **B.** Oui
- [ ] **C.** Non : `s` doit être `mut` — elle l'est, donc oui mais avec un warning
- [ ] **D.** Non : on ne peut pas afficher `s` après un emprunt mutable

<details><summary>Réponse</summary>

**B — oui, ça compile.** C'est le **NLL** (*Non-Lexical Lifetimes*), actif depuis Rust 2018.

La durée de vie d'un emprunt s'arrête à sa **dernière utilisation**, pas à la fin du bloc. `r1` meurt après le `println!`, donc `&mut s` ne chevauche rien.

Déplace `println!("{r1}")` **après** `let r2 = &mut s;` et là ça casse : les deux emprunts se chevaucheraient réellement.

</details>

---

### Q3. 🔥 Pourquoi ce code échoue-t-il ?

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);
println!("{first}");
```

- [ ] **A.** `push` prend `&mut self`, ce qui entre en conflit avec l'emprunt immutable `first` toujours vivant
- [ ] **B.** On ne peut pas emprunter un élément d'un `Vec`
- [ ] **C.** `v` n'est pas `mut`
- [ ] **D.** Ça compile, NLL s'en occupe

<details><summary>Réponse</summary>

**A** — erreur `E0502`. Contrairement à Q2, `first` est **encore utilisé après** le `push`, donc son emprunt chevauche bien l'emprunt mutable.

Et ce n'est pas du pédantisme : `push` peut déclencher une **réallocation** du buffer. `first` pointerait alors sur de la mémoire libérée. C'est exactement le *use-after-free* que le borrow checker élimine, gratuitement, à la compilation.

Ce cas reviendra à l'identique en ex06 (modifier une collection pendant qu'on la parcourt) et en ex11 (`sell` sur l'inventaire).

</details>

---

### Q4. Combien de ces lignes compilent ?

```rust
let mut s = String::from("hi");
let a = &s;        // (1)
let b = &s;        // (2)
let c = &mut s;    // (3)
println!("{a}{b}{c}");
```

- [ ] **A.** Les 4
- [ ] **B.** (1) et (2) seulement — (3) échoue
- [ ] **C.** Aucune
- [ ] **D.** (3) seulement

<details><summary>Réponse</summary>

**B.** La règle : **soit** N emprunts immutables, **soit** 1 emprunt mutable — jamais les deux.

Ici le `println!` utilise `a`, `b` **et** `c`, donc les trois emprunts sont vivants simultanément → `E0502` sur la ligne (3).

Le raisonnement à ancrer : un `&mut` est une garantie d'**exclusivité**. Tant qu'il existe, personne d'autre ne peut ni lire ni écrire. C'est ce qui rend le *data race* impossible par construction — on le retrouvera tel quel en ex17 avec les threads.

</details>

---

### Q5. 🔥 Ce code compile-t-il ?

```rust
fn shout(s: &mut String) {
    *s = s.to_uppercase();
}

fn main() {
    let name = String::from("alice");
    shout(&mut name);
    println!("{name}");
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `cannot borrow name as mutable, as it is not declared as mutable`
- [ ] **C.** Non : `*s = ...` est invalide
- [ ] **D.** Non : `to_uppercase` prend `self` par valeur

<details><summary>Réponse</summary>

**B.** Pour créer un `&mut x`, il faut que `x` soit déclaré `let mut x`. La mutabilité de la variable et celle de la référence sont deux choses distinctes, mais la seconde exige la première.

Le corps de `shout` est correct : `s.to_uppercase()` produit une **nouvelle** `String` (elle prend `&self` et alloue), et `*s = ...` écrase la valeur pointée. L'ancienne `String` est libérée au passage.

Note : `let mut s: &String` (référence *mutable de nom*) et `let s: &mut String` (référence *vers* du mutable) sont différents. `let mut s: &mut String` combine les deux.

</details>

---

### Q6. 🔥 Quelle est la sortie ?

```rust
let x = 5;
let y = x;
let s1 = String::from("hi");
let s2 = s1.clone();
println!("{x} {y} {s1} {s2}");
```

- [ ] **A.** Erreur : `x` déplacé
- [ ] **B.** `5 5 hi hi`
- [ ] **C.** Erreur : `s1` déplacé
- [ ] **D.** `5 5 hi hi` avec un warning clippy

<details><summary>Réponse</summary>

**B.** `i32` implémente `Copy` → `let y = x` copie les 4 octets sur la pile, `x` reste valide. `.clone()` fait explicitement la même chose pour `String` : nouvelle allocation heap + copie des octets.

Règle mnémotechnique : **un type est `Copy` s'il n'a rien à libérer**. Dès qu'un type possède une ressource heap (`String`, `Vec`, `Box`, `HashMap`), il implémente `Drop` et ne peut **pas** être `Copy` — les deux traits sont mutuellement exclusifs.

</details>

---

### Q7. 🔥 Lequel de ces appels échoue ?

```rust
fn greet(s: &str) { println!("{s}"); }
fn shout(s: &String) { println!("{s}"); }

let owned = String::from("hi");
let literal = "hi";

greet(&owned);    // (1)
greet(literal);   // (2)
shout(&owned);    // (3)
shout(literal);   // (4)
```

- [ ] **A.** (2)
- [ ] **B.** (4)
- [ ] **C.** (1) et (4)
- [ ] **D.** Aucun

<details><summary>Réponse</summary>

**B — (4) échoue.** La **deref coercion** est à sens unique : `&String` se convertit automatiquement en `&str` (car `String: Deref<Target = str>`), mais jamais l'inverse.

Conséquence pratique, et c'est la vraie leçon : **prends toujours `&str` en paramètre, jamais `&String`**. Tu acceptes alors les deux mondes gratuitement. Clippy te le dira d'ailleurs (`ptr_arg`).

L'énoncé de l'exercice demande `&String` volontairement, pour te faire manipuler l'emprunt explicitement — mais en vrai code, c'est `&str`.

</details>

---

### Q8. 🔥 Quelle est la sortie ?

```rust
let s = String::from("hello");
let len = s.len();
let slice = &s[0..2];
println!("{len} {slice}");
```

Et si on remplace `"hello"` par `"héllo"` avec `&s[0..2]` ?

- [ ] **A.** `5 he` — et `5 hé` dans le second cas
- [ ] **B.** `5 he` — et le second **panique** à l'exécution
- [ ] **C.** `5 he` — et le second affiche `h?`
- [ ] **D.** Erreur de compilation dans le second cas

<details><summary>Réponse</summary>

**B.** Une `String` est un **`Vec<u8>` d'UTF-8 valide**. `.len()` compte les **octets**, pas les caractères, et le slicing indexe des octets.

`"héllo"` fait **6** octets (`é` en occupe 2). `&s[0..2]` couperait `é` en deux → panique à l'exécution : *`byte index 2 is not a char boundary`*.

Le compilateur ne peut pas l'attraper (l'index est runtime). Les alternatives sûres : `.chars()`, `.char_indices()`, ou `s.get(0..2)` qui rend un `Option<&str>`.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | L'ownership est acquis — c'est 70 % de Rust |
| 6-7/8 | Solide. Revérifie Q2/Q3 : c'est la nuance NLL |
| < 6 | **Ne passe pas à la suite.** Reprends le README et fais échouer chaque cas volontairement dans ton `main.rs` |
