# Exercice 10 — Lifetimes

## Concept

Une lifetime est une annotation qui dit au compilateur **combien de temps une référence est valide**. Le compilateur les infère la plupart du temps — tu n'as à les écrire que quand il ne peut pas.

Le cas classique : une fonction qui retourne une référence vers une de ses entrées.

```rust
// Le compilateur ne sait pas si le retour vit aussi longtemps que `a` ou `b`
fn longest(a: &str, b: &str) -> &str { // ❌ erreur
    if a.len() > b.len() { a } else { b }
}
```

Avec annotation :
```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { // ✅
    if a.len() > b.len() { a } else { b }
}
```

`'a` est un nom de lifetime (convention : lettres courtes). Ici ça dit : "le retour vit au moins aussi longtemps que le plus court de `a` et `b`".

---

## Lifetimes dans les structs

Si une struct contient une référence, elle doit annoter sa lifetime :

```rust
struct Excerpt<'a> {
    text: &'a str, // cette référence doit vivre au moins aussi longtemps que la struct
}
```

```rust
let text = String::from("Hello world");
let excerpt = Excerpt { text: &text };
// excerpt ne peut pas outliver text
```

---

## Ce que les lifetimes ne font PAS

Elles ne changent pas combien de temps une valeur vit. Elles **décrivent** les relations entre durées de vie existantes pour que le compilateur puisse vérifier la cohérence.

---

## Elision

Dans beaucoup de cas, Rust infère les lifetimes automatiquement (elision rules). Tu n'as à les écrire que quand l'ambiguïté est réelle.

```rust
fn first_word(s: &str) -> &str { // lifetimes inférées, pas besoin d'annoter
    &s[..s.find(' ').unwrap_or(s.len())]
}
```

---

## Exercice

1. Écris une fonction `longest_name<'a>(a: &'a str, b: &'a str) -> &'a str` qui retourne le nom le plus long
2. Définis une struct `InventoryRef<'a>` qui contient une référence vers un nom de catégorie (`&'a str`) et un `Vec<Item>` owned
3. Implémente une méthode `describe(&self)` sur `InventoryRef` qui affiche la catégorie et le nombre d'items
4. Dans le `main`, crée une `String` pour la catégorie, puis une `InventoryRef` qui pointe dessus, et vérifie que le compilateur t'empêche de dropper la catégorie avant l'inventaire

**Output attendu :**
```
Longest: Greatsword
Category: Weapons | 2 items
```

---

## Pistes

- Pour l'étape 4, essaie de `drop(category)` avant d'utiliser `inventory_ref` — observe l'erreur du compilateur, c'est instructif
- `drop(x)` force la libération immédiate d'une valeur
- La struct `InventoryRef` possède le `Vec<Item>` mais emprunte le nom de catégorie — deux natures différentes dans la même struct

---

## Lancer l'exercice

```bash
cargo run -p ex10-lifetimes
```
