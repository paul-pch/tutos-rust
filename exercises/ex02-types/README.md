# Exercice 02 — Types & Inférence

## Concept

Rust est statiquement typé mais le compilateur **infère** le type dans la majorité des cas. Tu n'as pas besoin de tout annoter.

Il y a cependant des situations où tu **dois** annoter :
- Ambiguïté entre plusieurs types possibles
- Signature de fonction (toujours obligatoire)

```rust
// Inféré
let x = 5;

// Annoté (obligatoire ici, parse() peut retourner plusieurs types)
let x: i32 = "5".parse().unwrap();

// Signature de fonction : toujours explicite
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## Tuples

Un tuple regroupe plusieurs valeurs de types différents. Taille fixe.

```rust
let point = (10, 20);
let (x, y) = point; // destructuring

println!("{}", point.0); // accès par index
```

---

## Arrays

Taille fixe, même type pour tous les éléments.

```rust
let arr = [1, 2, 3, 4, 5];
println!("{}", arr[0]);
println!("{}", arr.len());
```

Différent de `Vec` qui est dynamique — on verra ça plus tard.

---

## Fonctions

```rust
fn multiply(a: i32, b: i32) -> i32 {
    a * b // pas de return ni de ; — la dernière expression est la valeur retournée
}
```

Le `return` existe mais est idiomatique uniquement pour les retours anticipés.

---

## Exercice

1. Déclare un tuple `player` contenant un nom (`&str`), un score (`i32`), et un niveau (`u8`)
2. Destructure ce tuple en 3 variables distinctes
3. Déclare un array `bonuses` de 3 entiers `i32`
4. Écris une fonction `total_score` qui prend un `i32` et un array `[i32; 3]` et retourne la somme du score et de tous les bonus
5. Affiche le résultat

**Output attendu :**
```
Player: Alice
Total score: 115
```
*(avec score=100 et bonuses=[5, 7, 3] par exemple)*

---

## Pistes

- Pour sommer un array : `.iter().sum()` ou une boucle `for`
- La signature de `total_score` avec un array de taille fixe : `fn total_score(score: i32, bonuses: [i32; 3]) -> i32`
- `[i32; 3]` signifie : array de 3 éléments de type `i32`. La taille fait partie du type.

---

## Lancer l'exercice

```bash
cargo run -p ex02-types
```
