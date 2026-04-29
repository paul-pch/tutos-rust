# Exercice 01 — Variables & Immutabilité

## Concept

En Rust, **toute variable est immutable par défaut**. C'est l'inverse de la plupart des langages.

```rust
let x = 5;
x = 6; // ❌ erreur de compilation
```

Pour la rendre mutable :

```rust
let mut x = 5;
x = 6; // ✅
```

Ce n'est pas qu'une convention : c'est **enforced par le compilateur**. C'est une des fondations du modèle de sécurité de Rust.

---

## Types de base

Rust est **statiquement typé** mais infère le type la plupart du temps :

```rust
let x = 5;        // i32 inféré
let y: u32 = 5;   // u32 explicite
let z = 5.0;      // f64 inféré
let ok = true;    // bool
let c = 'a';      // char
```

Les entiers ont une taille explicite :

| Type | Taille | Signé |
|------|--------|-------|
| `i8` … `i128` | 8 à 128 bits | oui |
| `u8` … `u128` | 8 à 128 bits | non |
| `i32` | 32 bits | oui (défaut) |
| `usize` | dépend de l'archi | non |

---

## Shadowing

Rust permet de **re-déclarer** une variable avec le même nom :

```rust
let x = 5;
let x = x + 1; // nouveau x, pas de mut
let x = "hello"; // peut même changer de type !
```

Ce n'est pas une mutation — c'est une nouvelle variable qui **masque** l'ancienne.  
Différence clé avec `mut` : le shadowing permet de changer le type, `mut` non.

---

## Exercice

Crée `src/main.rs` et fais compiler ce programme :

1. Déclare une variable immutable `score` de type `i32` valant `10`
2. Déclare une variable mutable `lives` de type `u8` valant `3`
3. Modifie `lives` pour lui soustraire `1`
4. Utilise le shadowing pour transformer `score` en `String` avec le format `"Score: 10"`
5. Affiche les deux variables

**Output attendu :**
```
Score: 10
Lives: 2
```

---

## Lancer l'exercice

```bash
cargo run -p ex01-variables
```

## Vérifier avec clippy

```bash
cargo clippy -p ex01-variables
```

---

## Pistes

- Pour afficher : `println!("{}", variable)`
- Pour convertir un i32 en String : `format!("Score: {}", score)`
- Si tu bloques sur un type, le compilateur te dit exactement quoi faire — lis ses messages, ils sont excellents en Rust
