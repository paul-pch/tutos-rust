# Exercice 08 — Erreurs Custom

## Concept

En ex07 tu as utilisé `.unwrap()` sur les erreurs de parsing — c'est un crash garanti si la donnée est malformée. En production, on définit ses propres types d'erreur.

---

## Définir une erreur custom

Une erreur custom est une enum dont chaque variant représente un cas d'échec :

```rust
#[derive(Debug)]
enum ParseError {
    InvalidFormat(String),
    InvalidNumber(String),
}
```

Pour qu'elle soit utilisable avec `?` et compatible avec la stdlib, elle doit implémenter deux traits : `Display` et `std::error::Error` :

```rust
use std::fmt;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidFormat(s) => write!(f, "Format invalide : {}", s),
            ParseError::InvalidNumber(s) => write!(f, "Nombre invalide : {}", s),
        }
    }
}

impl std::error::Error for ParseError {}
```

---

## Combiner plusieurs types d'erreur

Quand une fonction peut retourner plusieurs types d'erreur (`io::Error`, `ParseError`...), le type de retour le plus simple est `Box<dyn std::error::Error>` :

```rust
fn load_items(path: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    ...
}
```

`Box<dyn Error>` accepte n'importe quel type qui implémente `Error` — pratique, mais tu perds l'information précise sur le type d'erreur à l'appel.

---

## Convertir une erreur avec `map_err`

Pour transformer une erreur d'un type en un autre avant de la propager :

```rust
"abc".parse::<f32>().map_err(|e| ParseError::InvalidNumber(e.to_string()))?;
```

---

## Exercice

Reprends le parser CSV de ex07 et remplace les `.unwrap()` par une gestion d'erreur propre.

1. Définis une enum `ParseError` avec deux variants :
   - `InvalidFormat(String)` — ligne mal formée (pas 3 champs)
   - `InvalidNumber(String)` — champ non parseable en nombre
2. Implémente `Display` et `std::error::Error` pour `ParseError`
3. Modifie `load_items` pour retourner `Result<Vec<Item>, Box<dyn std::error::Error>>`
4. Remplace chaque `.unwrap()` par un `.map_err(...)` suivi de `?`
5. Ajoute une ligne invalide dans `items.csv` et gère l'erreur proprement dans le `main`

**Output attendu (avec une ligne invalide `BadLine`) :**
```
Erreur de parsing : Format invalide : BadLine
```

**Output attendu (sans ligne invalide) :**
```
Loaded 3 items:
- Sword | 5.5kg | 120 gold
- Rope | 1.0kg | 10 gold
- Potion | 0.5kg | 30 gold
```

---

## Pistes

- `impl std::error::Error for ParseError {}` — le corps est vide, les méthodes ont des implémentations par défaut
- `map_err` prend une closure : `.map_err(|_| ParseError::InvalidFormat(line.to_string()))`
- Le `?` sur un `Result<T, ParseError>` dans une fonction qui retourne `Result<_, Box<dyn Error>>` fonctionne automatiquement grâce à une conversion implicite

---

## Lancer l'exercice

```bash
cargo run -p ex08-custom-errors
```
