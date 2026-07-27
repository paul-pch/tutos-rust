# QUIZ — ex13 : Modules & visibilité

> 8 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Ce code compile-t-il ?

```rust
mod task {
    pub struct Task { pub title: String, id: u32 }
    impl Task {
        pub fn new(id: u32, title: &str) -> Self { Task { id, title: title.into() } }
    }
}

fn main() {
    let t = task::Task::new(1, "Fix bug");
    println!("{}", t.title);
    println!("{}", t.id);      // ← ici
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `field id of struct Task is private`
- [ ] **C.** Non : `Task` n'est pas accessible
- [ ] **D.** Non : il faut `use task::Task;`

<details><summary>Réponse</summary>

**B.** La visibilité est **par item**, y compris champ par champ. `title` est `pub`, `id` ne l'est pas.

C'est tout l'intérêt de l'exercice : `id` est un détail d'implémentation, exposé en lecture seule via l'accesseur `pub fn id(&self) -> u32`. Tu peux changer sa représentation (passer à un `u64`, un `Uuid`) sans casser un seul appelant.

Note sur (D) : `use` n'est **jamais** obligatoire, c'est un raccourci. `task::Task::new(...)` en chemin complet est parfaitement valide.

</details>

---

### Q2. 🔥 Ce code compile-t-il ?

```rust
mod task {
    struct Task { pub title: String }          // struct privée, champ pub

    impl Task {
        pub fn new(t: &str) -> Self { Task { title: t.into() } }
    }
}

fn main() {
    let t = task::Task::new("x");
}
```

- [ ] **A.** Oui, `title` est `pub`
- [ ] **B.** Non : `struct Task is private` — un champ ne peut pas être plus visible que son conteneur
- [ ] **C.** Oui, mais `t.title` sera inaccessible
- [ ] **D.** Non : `new` doit être `pub` aussi (elle l'est)

<details><summary>Réponse</summary>

**B.** La visibilité est **plafonnée** par le conteneur. `pub` sur un champ d'une struct privée ne sert à rien : on n'a de toute façon aucun moyen de nommer le type depuis l'extérieur.

Le README le dit en une phrase qu'il faut retenir :

> Si la struct est privée, ses champs `pub` le deviennent aussi.

Même logique pour une fonction `pub` dans un `mod` privé : elle est inatteignable. Le compilateur émet d'ailleurs parfois le lint `unreachable_pub` pour signaler ces `pub` décoratifs.

</details>

---

### Q3. 🔥 Ce code compile-t-il ?

```rust
mod config {
    pub struct Config { pub name: String }
}

mod printer {
    use super::config::Config;
    pub fn print(c: &Config) { println!("{}", c.name); }
}

mod formatter {
    pub fn format(c: &Config) -> String { c.name.clone() }   // ← ici
}
```

- [ ] **A.** Oui, `printer` a déjà importé `Config`
- [ ] **B.** Non : `cannot find type Config in this scope` — un `use` est local à son module
- [ ] **C.** Non : `config` doit être `pub`
- [ ] **D.** Oui, mais avec un warning

<details><summary>Réponse</summary>

**B.** Un `use` **ne déborde jamais**. Chaque module a son propre espace de noms et gère ses propres imports.

Le fix : `use super::config::Config;` dans `formatter` aussi.

Contre-intuitif pour qui vient de Python ou de JS, où un import au niveau du fichier vaut pour tout le fichier. En Rust, **un fichier n'est pas une unité de portée — un module l'est.** Et plusieurs modules peuvent cohabiter dans un fichier, comme ici.

Note : `mod config` n'a pas besoin d'être `pub` — il est visible depuis ses frères via `super::` puisqu'ils partagent le même parent (la racine du crate). `pub` ne serait requis que pour l'exposer **hors** du crate.

</details>

---

### Q4. Que signifient ces trois chemins ?

```rust
super::LABEL
crate::a::LABEL
self::helper()
```

- [ ] **A.** parent / racine du crate / module courant
- [ ] **B.** racine / parent / fichier courant
- [ ] **C.** module courant / parent / racine
- [ ] **D.** crate parent / crate courant / fonction courante

<details><summary>Réponse</summary>

**A.**

| Chemin | Cible |
|---|---|
| `super::` | le module **parent direct** (chaînable : `super::super::`) |
| `crate::` | la **racine du crate** — chemin absolu |
| `self::` | le **module courant** — utile pour lever une ambiguïté avec un item importé |

Quand utiliser quoi :
- `super::` pour les références **locales** entre modules frères → survit à un déplacement de tout le sous-arbre
- `crate::` quand le chemin relatif serait absurdement long, ou depuis un module profondément imbriqué
- `self::` rarement, principalement quand un `use` a introduit un nom qui masque un item local

Attention à ne pas confondre `crate::` (ce crate-ci) avec `::nom_de_crate` (une dépendance externe).

</details>

---

### Q5. 🔥 Ce code compile-t-il ?

```rust
mod outer {
    const SECRET: u32 = 42;        // privé

    pub mod inner {
        pub fn peek() -> u32 { super::SECRET }   // (1)
    }

    pub fn spy() -> u32 { inner::hidden() }      // (2)
}

mod inner_impl {}

// dans inner :
// fn hidden() -> u32 { 7 }                      // privée
```

Les lignes (1) et (2) compilent-elles ?

- [ ] **A.** Les deux
- [ ] **B.** (1) oui, (2) non
- [ ] **C.** (2) oui, (1) non
- [ ] **D.** Aucune

<details><summary>Réponse</summary>

**B**, et c'est **l'asymétrie fondamentale** de la visibilité Rust :

> Un item est visible depuis le module où il est défini **et tous ses descendants**.

- (1) ✅ `inner` est un descendant de `outer`, il voit donc `SECRET` même privé
- (2) ❌ `outer` est le **parent** de `inner` : il ne voit pas les items privés de son enfant → `function hidden is private`

Autrement dit : **la privauté est un mur vers le haut et l'extérieur, jamais vers le bas.** Un module enfant a un accès total à ses ancêtres.

C'est ce qui rend le pattern `mod tests` si naturel :

```rust
#[cfg(test)]
mod tests {
    use super::*;              // le module de test voit tout, y compris le privé
    #[test] fn t() { assert_eq!(super::hidden(), 7); }
}
```

</details>

---

### Q6. 🔥 Ce code compile-t-il ?

```rust
mod task {
    pub enum Priority { Low, Medium, High }
}

fn main() {
    let p = task::Priority::High;
}
```

- [ ] **A.** Non : il faut `pub` sur chaque variante
- [ ] **B.** Oui — les variantes d'un enum `pub` sont **automatiquement** publiques
- [ ] **C.** Non : il faut `use task::Priority;`
- [ ] **D.** Oui, mais les variantes sont en lecture seule

<details><summary>Réponse</summary>

**B.** Les variantes d'un enum (et leurs champs) héritent **toujours** de la visibilité de l'enum. `pub` sur une variante est une erreur de syntaxe :

```
error[E0449]: visibility qualifiers are not permitted here
```

La logique : un enum privé partiellement public n'aurait aucun sens — `match` doit être exhaustif, donc si tu peux voir l'enum tu dois voir toutes ses variantes.

C'est l'inverse d'une struct, où chaque champ décide. Cette différence explique un choix de conception fréquent en bibliothèque : `#[non_exhaustive]` sur un enum public, qui force les appelants externes à écrire un bras `_ =>` — te laissant la liberté d'ajouter des variantes sans casser leur code.

</details>

---

### Q7. 🔥 Que fait `pub use` ?

```rust
mod task { pub struct Task; pub enum Priority { Low } }
mod queue { pub struct Queue; }

pub use task::{Task, Priority};
pub use queue::Queue;
```

- [ ] **A.** Rien de plus qu'un `use` normal
- [ ] **B.** Il **ré-exporte** : `Task` devient accessible depuis la racine, sans exposer l'organisation interne en modules
- [ ] **C.** Il rend les modules publics
- [ ] **D.** Il duplique le code

<details><summary>Réponse</summary>

**B.** C'est le pattern *façade*, omniprésent dans l'écosystème.

Tes utilisateurs écrivent `use mon_crate::Task;` alors que le type vit dans `mon_crate::task::Task`. Tu gardes donc la **liberté de réorganiser** tes modules internes sans casser leur code : seul le `pub use` bouge.

La stdlib en est truffée : `std::io::Result` est un ré-export, `HashMap` vit dans `std::collections::hash::map` mais est exposé comme `std::collections::HashMap`.

Le pattern complet dans un `lib.rs` :

```rust
mod task;          // privé — l'arborescence interne ne regarde personne
mod queue;
pub use task::{Task, Priority};    // seule l'API publique est exposée
pub use queue::Queue;
```

Ça ne s'applique qu'aux **bibliothèques** : dans un `main.rs`, rien n'est visible de l'extérieur de toute façon.

</details>

---

### Q8. Dans quel ordre le compilateur voit-il ces modules ?

```rust
fn main() {
    let q = Queue::new("DevOps Queue");
}

mod task { ... }
mod queue { use super::task::Task; ... }
use queue::Queue;
```

- [ ] **A.** Erreur : `main` utilise `Queue` avant sa déclaration
- [ ] **B.** Ça compile — l'ordre de déclaration au niveau d'un module est sans importance
- [ ] **C.** Erreur : `use` doit être en première ligne
- [ ] **D.** Erreur : `mod queue` référence `task` déclaré avant lui, ce qui crée un cycle

<details><summary>Réponse</summary>

**B.** Contrairement à C ou C++, Rust fait une **passe de résolution de noms** avant la vérification de types. À l'intérieur d'un module (y compris la racine), l'ordre de déclaration des `fn`, `struct`, `mod` et `use` est totalement libre.

Les références circulaires entre modules sont d'ailleurs autorisées : `a` peut utiliser `b` qui utilise `a`. Ce n'est un problème qu'entre **crates**, où le cycle est interdit.

La seule chose qui reste ordonnée, c'est le corps d'une fonction : là, une variable doit être déclarée avant usage.

Convention de mise en page malgré tout : `use` en haut, puis les `mod`, puis les types, puis les fonctions. Rustfmt ne le fait pas pour toi, mais tout le monde l'applique.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | Tu peux organiser un vrai crate — Q5 et Q7 sont les deux clés |
| 6-7/8 | Bien. Ancre Q5 : la privauté descend, elle ne remonte pas |
| < 6 | Reprends Q2, Q3, Q5. Ces trois règles couvrent la quasi-totalité des `E0603 / private` que tu rencontreras |
