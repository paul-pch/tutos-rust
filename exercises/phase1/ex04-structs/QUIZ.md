# QUIZ — ex04 : Structs

> 7 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Ce code compile-t-il ?

```rust
struct Item { name: String, value: u32 }

impl Item {
    fn boost(&self) { self.value += 10; }
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : il faut `&mut self`
- [ ] **C.** Non : il faut déclarer `value: mut u32`
- [ ] **D.** Non : `impl` ne peut pas modifier les champs

<details><summary>Réponse</summary>

**B.** `&self` est une référence **immutable** vers l'instance : aucun champ n'est modifiable à travers elle. Il faut `fn boost(&mut self)`.

Les trois formes de receveur :

| Signature | Sens | Après l'appel |
|---|---|---|
| `&self` | lecture seule | l'instance reste utilisable |
| `&mut self` | lecture/écriture | l'instance reste utilisable |
| `self` | **consomme** l'instance | l'instance est déplacée, inutilisable |

Il n'y a **pas** de mutabilité par champ en Rust : c'est tout ou rien sur la variable qui possède la struct.

</details>

---

### Q2. Ce code compile-t-il ?

```rust
let sword = Item::new("Sword", 5.5, 120);
sword.take_damage(10);   // fn take_damage(&mut self, d: u32)
```

- [ ] **A.** Oui
- [ ] **B.** Non : `cannot borrow sword as mutable, as it is not declared as mutable`
- [ ] **C.** Non : `take_damage` doit être appelée avec `Item::take_damage(&mut sword, 10)`
- [ ] **D.** Oui, avec un warning

<details><summary>Réponse</summary>

**B.** Appeler une méthode `&mut self` crée implicitement un `&mut sword` — même contrainte qu'en ex03 : il faut `let mut sword`.

La règle qui surprend : **la mutabilité appartient à la liaison, pas au type.** Une même `Item` peut être immutable chez un propriétaire et mutable chez un autre, selon comment elle est liée.

</details>

---

### Q3. 🔥 Ce code compile-t-il ?

```rust
struct Item { name: String, value: u32 }

fn main() {
    let i = Item { name: "Rope".to_string(), value: 10 };
    println!("{:?}", i);
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `Item doesn't implement Debug` — il faut `#[derive(Debug)]`
- [ ] **C.** Non : il faut `#[derive(Display)]`
- [ ] **D.** Oui, `{:?}` marche sur toute struct

<details><summary>Réponse</summary>

**B.** Rien n'est implicite : `{:?}` exige le trait `Debug`, `{}` exige `Display`.

Le piège classique : **`#[derive(Display)]` n'existe pas.** `Debug` est dérivable car sa sortie est mécanique (le nom des champs) ; `Display` est destiné aux humains, Rust refuse de deviner le format. Il faut l'écrire à la main — c'est exactement ce que tu feras en ex11, ex12, ex13, ex14.

Et `{:#?}` (pretty-print) donne la version multi-lignes indentée, très utile pour déboguer une struct imbriquée.

</details>

---

### Q4. 🔥 Quelles lignes compilent après ce bloc ?

```rust
#[derive(Debug)]
struct Item { name: String, weight: f32, value: u32 }

let a = Item { name: "Sword".into(), weight: 5.5, value: 120 };
let b = Item { value: 50, ..a };

println!("{}", a.weight);  // (1)
println!("{}", a.name);    // (2)
println!("{:?}", a);       // (3)
```

- [ ] **A.** Les 3
- [ ] **B.** (1) seulement
- [ ] **C.** Aucune
- [ ] **D.** (1) et (3)

<details><summary>Réponse</summary>

**B — (1) seulement.** C'est le **partial move**, et c'est fin.

La syntaxe `..a` **déplace** les champs restants dans `b`, champ par champ :
- `name: String` → **déplacé** (pas `Copy`) → `a.name` est mort → (2) ❌
- `weight: f32` → **copié** (`Copy`) → `a.weight` reste lisible → (1) ✅
- `a` en tant que valeur entière est partiellement déplacé → on ne peut plus l'utiliser d'un bloc → (3) ❌

Rust suit l'ownership **par champ**, pas seulement par variable. Si tous les champs étaient `Copy`, les 3 lignes passeraient.

</details>

---

### Q5. 🔥 Ce code compile-t-il ?

```rust
impl Item {
    fn consume(self) -> u32 { self.value }
}

let i = Item::new("Potion", 0.5, 30);
println!("{}", i.consume());
println!("{}", i.consume());
```

- [ ] **A.** Oui, affiche `30` deux fois
- [ ] **B.** Non : `use of moved value: i`
- [ ] **C.** Non : `consume` doit prendre `&self`
- [ ] **D.** Oui, mais avec un warning

<details><summary>Réponse</summary>

**B.** `fn consume(self)` prend l'ownership de l'instance. Le premier appel déplace `i` dans la méthode ; à la fin de `consume`, l'`Item` est libéré. Le second appel emprunte un cadavre.

Quand utiliser `self` volontairement ? Quand la méthode **transforme** ou **détruit** logiquement la valeur : `into_iter()`, `into_bytes()`, `unwrap()`, ou le `swap(self)` de ex12. La convention de nommage `into_*` signale précisément ce transfert.

</details>

---

### Q6. Comment appelle-t-on `Item::new` ?

```rust
impl Item {
    fn new(name: &str, weight: f32, value: u32) -> Item { ... }
}
```

- [ ] **A.** `item.new("Sword", 5.5, 120)`
- [ ] **B.** `Item::new("Sword", 5.5, 120)`
- [ ] **C.** `Item.new("Sword", 5.5, 120)`
- [ ] **D.** `new::Item("Sword", 5.5, 120)`

<details><summary>Réponse</summary>

**B.** Sans paramètre `self`, c'est une **fonction associée** : elle appartient au type, pas à une instance → appel avec `::`.

Deux détails idiomatiques :
- `new` n'a **rien de spécial** en Rust (pas de mot-clé, pas de trait) — c'est une pure convention
- `-> Item` peut s'écrire `-> Self` : c'est plus court et ça survit à un renommage du type. Tu verras `Self` partout à partir de ex12

</details>

---

### Q7. 🔥 `describe(&self)` affiche l'item mais ne retourne rien. Laquelle de ces signatures est **la meilleure** pour un vrai projet ?

- [ ] **A.** `fn describe(&self) { println!("..."); }`
- [ ] **B.** `fn describe(&self) -> String { format!("...") }`
- [ ] **C.** `impl Display for Item { fn fmt(...) }`
- [ ] **D.** `fn describe(self) -> String`

<details><summary>Réponse</summary>

**C**, et c'est la trajectoire de ce cursus.

- **A** (l'énoncé de ex04) : simple, mais **intestable** — tu ne peux pas asserter sur du `stdout`, et l'appelant ne choisit pas la destination (fichier ? log ? `stderr` ?)
- **B** (ex06/ex09) : testable, mais alloue une `String` à chaque appel et reste hors de l'écosystème standard
- **C** (ex11+) : `write!` directement dans le `Formatter` — zéro allocation, et tu gagnes gratuitement `to_string()`, `format!("{item}")`, `{:>20}`, et l'usage dans n'importe quelle API générique sur `Display`
- **D** : consomme l'item pour l'afficher — absurde

À retenir : **implémenter le trait standard bat toujours la méthode maison**. C'est la leçon de ex09.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 7/7 | Structs maîtrisées, ex05 t'attend |
| 5-6/7 | Bien. Le point à revoir : Q4, le partial move |
| < 5 | Reprends le README, et écris volontairement les erreurs de Q1/Q2/Q5 pour lire les messages du compilateur |
