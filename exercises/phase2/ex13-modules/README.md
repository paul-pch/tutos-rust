# ex13-modules

## Concept

### `mod` et espace de noms

Un module (`mod`) crée un espace de noms isolé. Tout ce qui y est déclaré est **privé par défaut** — inaccessible depuis l'extérieur sans `pub` explicite :

```rust
mod engine {
    pub struct Config {      // pub : accessible de l'extérieur
        pub name: String,
        timeout: u32,        // privé : détail d'implémentation
    }

    impl Config {
        pub fn new(name: &str, timeout: u32) -> Self {
            Config { name: name.to_string(), timeout }
        }
        pub fn timeout(&self) -> u32 { self.timeout } // accesseur : lecture seule
    }

    fn internal() -> bool { true }  // invisible hors du module
}
```

### Privé par défaut — et c'est voulu

La règle Rust : **expose le moins possible**. Les champs et fonctions internes restent privés ; tu peux les restructurer librement sans casser les appelants.

Trois niveaux de visibilité :

| Annotation | Visible depuis |
|---|---|
| *(rien)* | le module lui-même uniquement |
| `pub(crate)` | n'importe où dans ce crate |
| `pub` | n'importe où, y compris les crates externes |

```rust
mod cache {
    pub struct Store {
        pub capacity: usize,          // le caller en a besoin
        hits: u64,                    // détail interne
    }
    impl Store {
        pub fn hits(&self) -> u64 { self.hits }           // lecture via accesseur
        pub(crate) fn reset(&mut self) { self.hits = 0 }  // partagé dans le crate
    }
}
```

> Si la struct est privée, ses champs `pub` le deviennent aussi — la visibilité d'un champ ne peut pas dépasser celle de la struct qui le contient.

### Portée de `use`

Un `use` est valide uniquement dans la portée où il est déclaré. Chaque module gère ses propres imports — un `use` dans un module ne déborde pas dans un autre :

```rust
mod printer {
    use super::config::Config; // import local à printer

    pub fn print(c: &Config) { println!("{}", c.name); }
}

mod formatter {
    use super::config::Config; // printer n'a pas "partagé" son import

    pub fn format(c: &Config) -> String { c.name.clone() }
}
```

### Chemins : `super::`, `crate::`, `self::`

| Chemin | Signifie |
|---|---|
| `super::` | module parent direct |
| `crate::` | racine du crate (chemin absolu) |
| `self::` | module courant (pour lever une ambiguïté) |

```rust
mod a {
    pub const LABEL: &str = "a";

    mod b {
        pub fn local() -> &'static str { super::LABEL }      // remonte d'un niveau
        pub fn absolute() -> &'static str { crate::a::LABEL } // depuis la racine
    }
}
```

Préfère `super::` pour les références locales ; `crate::` est utile quand le chemin relatif serait trop long ou ambigu.

### `use` raccourcit un chemin

```rust
use engine::Config;

fn main() {
    let c = Config::new("prod", 30); // sans use : engine::Config::new(...)
    println!("{} — {}s", c.name, c.timeout());
}
```

---

## Exercice

Construis un gestionnaire de tâches DevOps organisé en trois modules. Tout dans `src/main.rs`.

### Étape 1 — `mod task`

Déclare l'enum `pub Priority` avec les variantes `Low`, `Medium`, `High`.

Implémente `Display` pour `Priority` avec un format paddé sur 6 caractères :
- `Low` → `"LOW   "`, `Medium` → `"MEDIUM"`, `High` → `"HIGH  "`

Déclare la struct `pub Task` avec :
- `id: u32` — privé
- `pub title: String`
- `pub priority: Priority`
- `done: bool` — privé

Implémente sur `Task` :
- `pub fn new(id: u32, title: &str, priority: Priority) -> Self`
- `pub fn complete(&mut self)` — marque la tâche comme faite
- `pub fn is_done(&self) -> bool`
- `pub fn id(&self) -> u32`

### Étape 2 — `mod queue`

Dans ce module, importe `Task` depuis le module parent avec `use super::task::Task`.

Déclare la struct `pub Queue` avec :
- `name: String` — privé
- `tasks: Vec<Task>` — privé

Implémente sur `Queue` :
- `pub fn new(name: &str) -> Self`
- `pub fn push(&mut self, task: Task)`
- `pub fn complete_by_id(&mut self, id: u32) -> bool` — complète la tâche d'id donné ; renvoie `false` si introuvable
- `pub fn name(&self) -> &str`
- `pub fn pending(&self) -> Vec<&Task>` — tâches non terminées
- `pub fn done(&self) -> Vec<&Task>` — tâches terminées

### Étape 3 — `mod report`

Dans ce module, importe `Queue` avec `use super::queue::Queue`.

Écris `pub fn print_status(queue: &Queue)` qui affiche le rapport selon le format de l'output attendu.

### Étape 4 — `main`

Importe avec `use` : `task::{Priority, Task}`, `queue::Queue`, `report::print_status`.

1. Crée une `Queue::new("DevOps Queue")`.
2. Ajoute ces 5 tâches (dans cet ordre) :

   | id | title                   | priority |
   |----|-------------------------|----------|
   | 1  | Fix critical bug        | High     |
   | 2  | Run integration tests   | Medium   |
   | 3  | Deploy to production    | High     |
   | 4  | Update load balancer    | Medium   |
   | 5  | Archive old logs        | Low      |

3. Complète les tâches 1 et 2 via `complete_by_id`.
4. Appelle `print_status(&q)`.

---

## Output attendu

```
=== DevOps Queue ===

Pending tasks:
  [HIGH  ] #3 Deploy to production
  [MEDIUM] #4 Update load balancer
  [LOW   ] #5 Archive old logs

Completed tasks:
  [HIGH  ] #1 Fix critical bug
  [MEDIUM] #2 Run integration tests

3 pending, 2 done
```

---

## Pistes

- La visibilité `pub` sur un champ ne suffit pas si la struct elle-même n'est pas `pub` — les deux doivent l'être.
- `report` accède à `task.priority`, `task.id()`, `task.title` sans avoir besoin d'importer `Task` explicitement : les types retournés par les méthodes de `Queue` sont déjà résolus.
- `complete_by_id` : utilise `.iter_mut().find(|t| t.id() == id)` puis appelle `.complete()` sur le résultat.
- `pending` et `done` : `.iter().filter(...).collect()`.
- Affichage du numéro de tâche : `println!("  [{}] #{} {}", task.priority, task.id(), task.title)`.

---

## Lancer l'exercice

```sh
cargo run -p ex13-modules
cargo test -p ex13-modules
```
