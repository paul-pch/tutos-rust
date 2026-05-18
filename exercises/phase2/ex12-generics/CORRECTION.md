# Correction — ex12-generics

---

## Bugs (bloquants)

### 1. `None` du `min_value` affiche "Max" (L102)
```rust
// ❌ actuel
None => println!("Max: N/A"),

// ✅ attendu
None => println!("Min: N/A"),
```
Copier-coller fautif depuis le bloc `max_value`.

---

## Types (sémantique)

### 2. `Bucket<i64>` pour de la mémoire (L118)
```rust
// ❌ actuel
let mut mem: Bucket<i64> = Bucket::new("Memory (bytes)");

// ✅ attendu
let mut mem: Bucket<u64> = Bucket::new("Memory (bytes)");
```
La mémoire ne peut pas être négative. `u64` est le type sémantiquement correct.

---

## Nommage (typos)

### 3. `Measurment` → `Measurement` (L3, partout)
Il manque le `e` : `Measur**e**ment`.

### 4. `thresold` → `threshold` (L62, L91, L104)
Il manque le `h` : `thres**h**old`.

---

## Style idiomatique

### 5. Import inutile : `vec` (L1)
```rust
// ❌ actuel
use std::{fmt::{self, Display}, vec};

// ✅ attendu
use std::fmt::{self, Display};
```
`vec![]` est dans le prelude Rust — pas besoin de l'importer.

### 6. `iter().count()` sur un `Vec` (L47)
```rust
// ❌ actuel
self.items.iter().count()

// ✅ attendu
self.items.len()
```
`Vec` expose `.len()` en O(1). Itérer pour compter est inutile.

### 7. `{:.2}` sans effet sur un type `Display` custom (L97, L101)
```rust
// ❌ actuel
println!("Max: {:.2}", item)

// ✅ attendu
println!("Max: {}", item)
```
La précision `:.2` est passée au `Formatter`, mais l'impl `Display` de `Measurment` utilise `write!(f, "{} = {}", ...)` qui ne la relit pas. Le `.2` est sans effet et trompeur.

### 8. Nombres magiques sans underscores (L119-122)
```rust
// ❌ actuel
8589934592, 524288000, 268435456, 134217728

// ✅ attendu
8_589_934_592, 524_288_000, 268_435_456, 134_217_728
```
Les underscores rendent les grands nombres lisibles en Rust.
