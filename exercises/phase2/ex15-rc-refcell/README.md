# ex15-rc-refcell — `Rc<T>`, `RefCell<T>` et partage de données

## Concept

Jusqu'ici, une valeur avait **un seul propriétaire** et la mutabilité était vérifiée
**à la compilation**. Deux besoins cassent ces règles :

- **Plusieurs propriétaires** pour la même donnée → `Rc<T>` (Reference Counted)
- **Muter une donnée derrière une référence partagée** → `RefCell<T>` (interior mutability)

### `Rc<T>` — propriété partagée

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);          // ne copie pas la String, incrémente le compteur
println!("{}", Rc::strong_count(&a)); // 2
```

- `Rc::clone` est **bon marché** : il ne duplique pas la donnée, seulement le compteur.
- La donnée est libérée quand le compteur retombe à `0`.
- `Rc<T>` ne donne qu'un accès **immuable** (`&T`). Impossible d'obtenir `&mut T`.
- `Rc<T>` n'est **pas** thread-safe — pour du multi-thread, c'est `Arc<T>`.

### `RefCell<T>` — mutabilité intérieure

`RefCell<T>` déplace la vérification d'emprunt de la **compilation** vers l'**exécution** :

```rust
use std::cell::RefCell;

let cell = RefCell::new(0);
*cell.borrow_mut() += 1;        // emprunt mutable, même via &self
println!("{}", cell.borrow());  // emprunt immuable
```

- `borrow()` → `Ref<T>` (partagé), `borrow_mut()` → `RefMut<T>` (exclusif).
- Les règles d'emprunt sont vérifiées **à l'exécution** : deux `borrow_mut()` simultanés
  (ou un `borrow` + un `borrow_mut`) provoquent un **panic**, pas une erreur de compilation.

### La combinaison `Rc<RefCell<T>>`

Le motif classique : **partager** une donnée (`Rc`) tout en la rendant **mutable** (`RefCell`).

```rust
let counter = Rc::new(RefCell::new(0));
let view = Rc::clone(&counter);
*counter.borrow_mut() += 10;
println!("{}", view.borrow()); // 10 — la mutation est visible par tous les Rc
```

**Pièges courants :**

- Garder un `borrow()` vivant trop longtemps puis appeler `borrow_mut()` → panic.
- Les **cycles** de `Rc` ne sont jamais libérés (fuite mémoire) → `Weak<T>` casse le cycle.

---

## Exercice

Tu vas modéliser un **graphe de dépendances de services d'infrastructure**. Une même
dépendance (ex. un disque) est partagée par plusieurs services → `Rc`. Le statut d'un
service change à l'exécution alors qu'il est partagé → `RefCell`.

```
   api ──┐
         ├──> database ──> disk
   web ──┘
```

### Étape 1 — `Status`

Définis un enum `Status` avec deux variants : `Up` et `Down`.
Implémente `Display` : `Up` → `up`, `Down` → `down`.

### Étape 2 — `Service`

Définis une struct `Service` avec :
- `name: String`
- `status: RefCell<Status>` — muté à l'exécution derrière une référence partagée
- `deps: Vec<Rc<Service>>` — les dépendances, potentiellement partagées

Implémente `Service::new(name: &str, deps: Vec<Rc<Service>>) -> Rc<Service>`
(le constructeur renvoie directement un `Rc`, tous les services démarrent `Up`).

### Étape 3 — Méthodes

- `set_status(&self, status: Status)` — remplace le statut. Note bien : `&self`, **pas**
  `&mut self` — c'est tout l'intérêt de `RefCell`.
- `print_status(&self)` — affiche `<name>: <status>`.
- `is_healthy(&self) -> bool` — le service est `Up` **et** toutes ses dépendances
  (récursivement) sont saines.

### Étape 4 — `main`

1. Construis le graphe ci-dessus. `disk` est partagé par `database` ; `database` est
   partagé par `api` et `web` (utilise `Rc::clone`).
2. Affiche l'en-tête `=== Infra Status ===` puis le statut de `disk`, `database`, `api`, `web`.
3. Affiche si `api` et `web` sont sains.
4. Passe `disk` à `Down` (via une seule référence).
5. Affiche à nouveau la santé de `api` et `web` — la panne se propage par la dépendance partagée.
6. Affiche le nombre de références (`Rc::strong_count`) vers `disk` et vers `database`.

---

## Output attendu

```
=== Infra Status ===
disk: up
database: up
api: up
web: up

api healthy? true
web healthy? true

-- disk goes down --

api healthy? false
web healthy? false

references to disk: 2
references to database: 3
```

---

## Pistes

- `Rc::clone(&x)` (forme explicite) est préféré à `x.clone()` — il signale clairement
  qu'on incrémente un compteur, pas qu'on copie la donnée.
- Pour lire le statut dans un `match` : `match *self.status.borrow() { ... }`, ou
  `matches!(*self.status.borrow(), Status::Up)`.
- `is_healthy` est récursif : `self.deps.iter().all(|d| d.is_healthy())`.
- Les comptes de `strong_count` incluent la variable locale (`disk`) **en plus** des
  clones stockés dans les `deps`. Compte les liaisons vivantes pour retrouver `2` et `3`.
- Si tu obtiens un panic `already borrowed`, c'est qu'un `borrow` est encore vivant quand
  tu appelles `borrow_mut` — relâche-le (fin de scope) avant.

---

## Lancer l'exercice

```sh
cargo run -p ex15-rc-refcell
cargo test -p ex15-rc-refcell
```
