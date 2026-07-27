# QUIZ — ex09 : Traits

> 8 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Ce code compile-t-il ?

```rust
trait Summarize {
    fn summary(&self) -> String;
    fn print_summary(&self) { println!("{}", self.summary()); }
}

struct Item;
impl Summarize for Item {
    fn summary(&self) -> String { "un item".into() }
}
```

- [ ] **A.** Non : `print_summary` n'est pas implémentée
- [ ] **B.** Oui — `print_summary` a une implémentation par défaut
- [ ] **C.** Non : une méthode par défaut ne peut pas appeler une méthode requise
- [ ] **D.** Oui, mais `print_summary` ne fera rien

<details><summary>Réponse</summary>

**B.** Une méthode par défaut peut parfaitement appeler les méthodes **requises** du trait : au moment de l'appel, le compilateur sait que `Self: Summarize`, donc `self.summary()` existe forcément.

C'est le pattern **template method** : le trait fournit le squelette, l'implémenteur remplit le minimum. `Iterator` est l'exemple ultime — tu implémentes `next()`, et tu récupères `map`, `filter`, `sum`, `zip`… gratuitement.

Un implémenteur peut **surcharger** une méthode par défaut. Mais attention (voir Q7) : cette surcharge n'est visible que via dispatch dynamique ou générique, jamais depuis une autre méthode par défaut du trait qui appellerait `Self::print_summary` en dur.

</details>

---

### Q2. 🔥 Ce code compile-t-il ?

```rust
impl std::fmt::Display for Vec<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} éléments", self.len())
    }
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `only traits defined in the current crate can be implemented for types defined outside of the crate`
- [ ] **C.** Non : `Vec` est déjà `Display`
- [ ] **D.** Oui, mais ça écrase l'impl de la stdlib

<details><summary>Réponse</summary>

**B** — la **règle de l'orphelin** (`E0117`). Pour écrire `impl Trait for Type`, il faut que **le trait ou le type** soit défini dans ton crate. Ici les deux sont dans `std`.

Pourquoi cette règle ? Sans elle, deux crates de ton arbre de dépendances pourraient fournir deux `impl Display for Vec<String>` incompatibles, et le linker n'aurait aucun moyen de trancher. Rust garantit qu'il existe **au plus une** impl d'un trait pour un type — c'est ce qu'on appelle la cohérence.

Le contournement standard, le **newtype** :

```rust
struct Inventory(Vec<String>);          // ← type local, l'impl devient légale
impl std::fmt::Display for Inventory { ... }
```

Coût à l'exécution : zéro (un tuple struct à un champ disparaît à la compilation).

Note au passage : `Vec<T>` n'implémente **pas** `Display`, seulement `Debug`. La stdlib refuse de choisir un séparateur à ta place.

</details>

---

### Q3. Ce code compile-t-il ?

```rust
mod traits {
    pub trait Summarize { fn summary(&self) -> String; }
    pub struct Item;
    impl Summarize for Item { fn summary(&self) -> String { "x".into() } }
}

fn main() {
    let i = traits::Item;
    println!("{}", i.summary());
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `items from traits can only be used if the trait is in scope` — il manque `use traits::Summarize;`
- [ ] **C.** Non : `Item` n'est pas public
- [ ] **D.** Non : `summary` doit être `pub`

<details><summary>Réponse</summary>

**B.** Une méthode de trait n'est appelable que si **le trait lui-même est importé**, même si le type l'est déjà. Le compilateur le dit explicitement :

```
help: trait `Summarize` which provides `summary` is implemented but not in scope
```

C'est la raison des `use std::io::Write;` ou `use std::fmt::Write;` que tu vois partout : on n'utilise jamais le nom `Write`, seulement ses méthodes.

Note : les méthodes du trait n'ont pas besoin d'être marquées `pub` — elles héritent de la visibilité du trait.

</details>

---

### Q4. 🔥 Ce code compile-t-il ?

```rust
fn print_all(items: &[impl Summarize]) {
    for i in items { i.print_summary(); }
}

let items = vec![sword, potion];      // deux Item
let player = Player::new("Alice");

print_all(&items);
print_all(&[sword, player]);          // ← ici
```

- [ ] **A.** Oui, `impl Summarize` accepte tout type implémentant le trait
- [ ] **B.** Non : une slice exige **un seul** type concret ; `Item` et `Player` sont différents
- [ ] **C.** Non : `print_all` ne peut être appelée qu'une fois
- [ ] **D.** Oui, avec une conversion automatique

<details><summary>Réponse</summary>

**B.** `&[impl Summarize]` signifie « une slice de **un** type `T`, quel qu'il soit, tel que `T: Summarize` ». Le `T` est fixé **par appel**, pas par élément.

Physiquement, une slice est un bloc contigu d'éléments de taille identique — `Item` et `Player` n'ont pas la même taille, c'est impossible.

Pour une collection **hétérogène**, il faut passer au dispatch dynamique :

```rust
let mixed: Vec<Box<dyn Summarize>> = vec![Box::new(sword), Box::new(player)];
for x in &mixed { x.print_summary(); }
```

Chaque `Box<dyn Summarize>` fait 16 octets (pointeur données + pointeur vtable), taille uniforme. C'est exactement ce que tu feras en ex14 (`Box<dyn Trait>`) et ex16 (`Box<dyn SchedulingPolicy>`).

</details>

---

### Q5. 🔥 Quelle est la vraie différence entre ces deux versions ?

```rust
fn print_all(items: &[impl Summarize]) { ... }        // (1) — statique
fn print_all(items: &[&dyn Summarize]) { ... }        // (2) — dynamique
```

- [ ] **A.** Aucune, c'est du sucre syntaxique
- [ ] **B.** (1) est monomorphisée (une copie par type, appels inlinables) ; (2) passe par une vtable
- [ ] **C.** (2) est plus rapide
- [ ] **D.** (1) ne marche pas avec les méthodes par défaut

<details><summary>Réponse</summary>

**B.**

| | `impl Trait` / générique | `dyn Trait` |
|---|---|---|
| Résolution | compilation | exécution (vtable) |
| Code généré | **une copie par type** utilisé | une seule |
| Appel | direct, inlinable | indirect |
| Types mélangés | ❌ | ✅ |
| Taille du binaire | gonfle | stable |

La **monomorphisation** : si tu appelles `print_all` avec `Item` puis `Player`, le compilateur génère physiquement deux fonctions. D'où la vitesse (zéro coût d'abstraction) et la taille des binaires Rust.

Choisis `dyn` quand tu as besoin d'hétérogénéité ou quand tu veux limiter la taille du binaire ; `impl`/générique par défaut.

Détail : `fn f(x: impl Trait)` et `fn f<T: Trait>(x: T)` sont identiques à **une** exception près — avec `impl Trait` tu ne peux pas forcer le type avec le turbofish `f::<Item>(x)`.

</details>

---

### Q6. 🔥 Peut-on faire un `Box<dyn Summarize>` de ce trait ?

```rust
trait Summarize {
    fn summary(&self) -> String;
    fn duplicate(&self) -> Self;
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : le trait n'est pas *dyn-compatible* (object safe) à cause de `-> Self`
- [ ] **C.** Non : il manque `: Sized`
- [ ] **D.** Oui, mais `duplicate` paniquera

<details><summary>Réponse</summary>

**B.** Pour construire une vtable, chaque méthode doit avoir une signature **connue et de taille fixe** indépendamment du type concret. `-> Self` viole ça : derrière un `dyn Summarize`, `Self` est inconnu et sa taille aussi.

Les principales causes de non-compatibilité `dyn` :
- une méthode qui rend `Self` (sauf en `Box<Self>`)
- une méthode **générique** : `fn compare<T>(&self, other: T)` — il faudrait une entrée de vtable par `T` possible, c'est infini
- une méthode statique (sans `self`)
- `Self: Sized` comme borne du trait

L'échappatoire : marquer la méthode gênante `where Self: Sized`. Elle disparaît alors de la vtable (inappelable via `dyn`) et le reste du trait redevient utilisable en trait object.

</details>

---

### Q7. 🔥 Pourquoi ce code compile-t-il, alors que tu n'as jamais implémenté `ToString` ?

```rust
struct Item;
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "item") }
}

let s: String = Item.to_string();
```

- [ ] **A.** `to_string` est une méthode inhérente à tout type
- [ ] **B.** À cause de l'**impl générale** `impl<T: Display + ?Sized> ToString for T` dans la stdlib
- [ ] **C.** Le compilateur dérive `ToString` automatiquement
- [ ] **D.** `to_string` vient de `Debug`

<details><summary>Réponse</summary>

**B.** C'est une *blanket impl* : une implémentation couvrant **tous** les types satisfaisant une borne.

```rust
impl<T: fmt::Display + ?Sized> ToString for T {
    fn to_string(&self) -> String { ... }
}
```

Implémente `Display` une fois → tu gagnes `to_string()`, `format!("{x}")`, `println!("{x}")`, et l'usage dans toute API générique sur `Display`. C'est le meilleur retour sur investissement de la stdlib, et la justification de ex04/Q7.

Autre blanket impl célèbre : `impl<T, U> Into<U> for T where U: From<T>` — implémente `From`, tu obtiens `Into` gratuitement. D'où la règle : **implémente toujours `From`, jamais `Into`**.

</details>

---

### Q8. Quelle syntaxe est **invalide** ?

```rust
fn a<T: Summarize + Clone>(x: T) {}                    // (1)
fn b<T>(x: T) where T: Summarize + Clone {}            // (2)
fn c(x: impl Summarize + Clone) {}                     // (3)
fn d<T: Summarize>(x: T) where T: Clone {}             // (4)
```

- [ ] **A.** (3)
- [ ] **B.** (4) — on ne peut pas mélanger les deux syntaxes
- [ ] **C.** Aucune
- [ ] **D.** (2) et (4)

<details><summary>Réponse</summary>

**C — les quatre sont valides.** `where` **complète** les bornes en ligne, il ne les remplace pas.

Quand `where` devient obligatoire :
- borne sur un type **associé** : `where T::Item: Display`
- borne sur un type qui n'est pas un paramètre : `where Vec<T>: Clone`
- lifetimes complexes : `where for<'a> F: Fn(&'a str) -> &'a str`

Convention : bornes en ligne quand il y en a une ou deux, `where` au-delà. C'est ce que le README de ex12 recommande.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | Excellent — Q5 et Q6 sont des questions d'entretien Rust senior |
| 6-7/8 | Solide. Fixe le tableau `impl Trait` vs `dyn Trait` de Q5, il gouverne ex14 et ex16 |
| < 6 | Reprends Q4 et Q5 : la distinction statique/dynamique est le cœur de la phase 2 |
