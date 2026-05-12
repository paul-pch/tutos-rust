# Exercice 03 — Ownership

## Concept

L'ownership est **la** notion centrale de Rust. Pas de garbage collector, pas de free manuel — le compilateur gère la mémoire via des règles strictes.

### Les 3 règles

1. Chaque valeur a **un seul propriétaire**
2. Il ne peut y avoir **qu'un seul propriétaire** à la fois
3. Quand le propriétaire sort du scope, la valeur est **libérée**

---

## Move

Quand tu assignes une valeur à une autre variable, la propriété est **transférée** (move). L'ancienne variable n'est plus utilisable.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 est "moved" dans s2

println!("{}", s1); // ❌ erreur : s1 n'existe plus
println!("{}", s2); // ✅
```

Cela ne s'applique **pas** aux types qui implémentent `Copy` (les scalaires : `i32`, `bool`, `f64`...). Ces types sont copiés automatiquement.

```rust
let x = 5;
let y = x; // copie, pas move

println!("{}", x); // ✅ x est toujours valide
```

---

## Borrowing

Pour utiliser une valeur sans en prendre la propriété, on **emprunte** avec `&` :

```rust
let s1 = String::from("hello");
let len = calculate_len(&s1); // on passe une référence

println!("{}", s1); // ✅ s1 est toujours valide
```

```rust
fn calculate_len(s: &String) -> usize {
    s.len()
} // s sort du scope mais ne libère rien — ce n'est pas son owner
```

Une référence est **immutable par défaut**. Pour emprunter de façon mutable :

```rust
fn add_world(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
add_world(&mut s);
```

### Règle des références

À tout moment, tu peux avoir **soit** :
- Autant de références immutables `&T` que tu veux
- **Une seule** référence mutable `&mut T`

Jamais les deux en même temps.

---

## Exercice

1. Crée une `String` `name` avec ton prénom
2. Passe `name` à une fonction `greet` qui affiche `"Hello, <name>!"` — sans prendre l'ownership (utilise `&String`)
3. Après l'appel, affiche `name` dans le main pour prouver qu'il est toujours valide
4. Crée une fonction `shout` qui prend un `&mut String` et met le contenu en majuscules
5. Appelle `shout` et affiche le résultat

**Output attendu :**
```
Hello, Alice!
Alice
ALICE
```

---

## Pistes

- Pour mettre en majuscules en place : la méthode n'existe pas directement sur `String`. Tu peux faire `*s = s.to_uppercase()` dans `shout`
- `*s` c'est le **déréférencement** — on accède à la valeur derrière la référence pour l'écraser

---

## Lancer l'exercice

```bash
cargo run -p ex03-ownership
```
