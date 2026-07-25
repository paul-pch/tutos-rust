# ex16-integration — Orchestrateur de conteneurs : exercice d'intégration

## Concept

> Exercice d'intégration de la phase 2. Le **fil directeur** est un mini-orchestrateur
> qui place des pods sur des nœuds. Tu vas mobiliser **tous** les patterns de la phase 2 —
> generics, modules, `Box<dyn Trait>`, `Rc<T>`, `RefCell<T>` — en plus des acquis de la
> phase 1 (traits, `Display`, erreurs custom, `Result`, itérateurs, closures).

Les briques réutilisées :

- **modules** (ex13) : le code est découpé en `mod` (`util`, `error`, `node`, `pod`, `scheduler`).
- **generics** (ex12) : une fonction générique d'agrégation avec contrainte `where`.
- **Box + trait objects** (ex14/ex09) : la stratégie de placement est un `Box<dyn ...>`
  interchangeable.
- **Rc** (ex15) : un même nœud est partagé entre l'orchestrateur et les pods qui y tournent.
- **RefCell** (ex15) : la capacité consommée d'un nœud change à l'exécution, derrière `&self`.
- **erreurs** (ex07/ex08) : un enum d'erreurs custom avec `Display` + `std::error::Error`.

Rappel du motif d'agrégation générique :

```rust
fn fold_by<T, F>(items: &[T], f: F) -> u32
where
    F: Fn(&T) -> u32,
{
    items.iter().map(f).sum()
}
```

---

## Exercice

Tu écris tout dans `src/main.rs`, organisé en modules. Pas de code fourni.

### Module `util`

Écris une fonction **générique** `sum_by<T, F>(items: &[T], f: F) -> u32` avec la
contrainte `where F: Fn(&T) -> u32`. Elle mappe chaque élément vers un `u32` et somme.
Elle servira à calculer la capacité et l'usage total du cluster.

### Module `error`

Définis `ClusterError` avec trois variants :
- `NodeNotFound(String)`
- `NotEnoughResources { node: String, requested: u32, available: u32 }`
- `NoNodeAvailable`

Implémente `Display` :
- `node not found: <name>`
- `not enough resources on <node> (requested <r>, available <a>)`
- `no node available`

Implémente aussi `std::error::Error for ClusterError`.

### Module `node`

Struct `Node` :
- `name: String`
- `cpu_total: u32`
- `cpu_used: RefCell<u32>` — muté à l'exécution

Méthodes :
- `Node::new(name: &str, cpu_total: u32) -> Rc<Node>`
- `available(&self) -> u32` — `cpu_total - cpu_used`
- `allocate(&self, cpu: u32) -> Result<(), ClusterError>` — refuse avec
  `NotEnoughResources` si `cpu > available()`, sinon incrémente `cpu_used`
- `Display` → `<name>: <used>/<total> cpu`

### Module `pod`

Struct `Pod` :
- `name: String`
- `cpu: u32`
- `node: RefCell<Option<Rc<Node>>>` — le nœud assigné, inconnu à la création

Méthodes :
- `Pod::new(name: &str, cpu: u32) -> Pod`
- `assign(&self, node: Rc<Node>)` — stocke le nœud (partagé via `Rc`)
- `assigned_node(&self) -> Option<Rc<Node>>` — renvoie un clone du `Rc` assigné

### Module `scheduler`

Un **trait** `SchedulingPolicy` :
- `choose(&self, nodes: &[Rc<Node>], cpu: u32) -> Option<Rc<Node>>`
- `name(&self) -> &str`

Un implémenteur `FirstFit` : `choose` renvoie le **premier** nœud dont `available() >= cpu`
(sinon `None`) ; `name` renvoie `"first-fit"`.

Struct `Scheduler` :
- `nodes: Vec<Rc<Node>>`
- `policy: Box<dyn SchedulingPolicy>`

Méthodes :
- `Scheduler::new(nodes: Vec<Rc<Node>>, policy: Box<dyn SchedulingPolicy>) -> Scheduler`
- `node(&self, name: &str) -> Result<Rc<Node>, ClusterError>` — recherche par nom
  (`.iter().find(...)`), `NodeNotFound` sinon
- `schedule(&self, pod: &Pod) -> Result<(), ClusterError>` : demande un nœud à la politique
  (`NoNodeAvailable` si `None`), l'`allocate`, puis `assign` le pod sur ce nœud

### `main`

1. Crée deux nœuds : `node-a` (4 cpu), `node-b` (8 cpu).
2. Crée un `Scheduler` avec la politique `FirstFit` (via `Box`).
3. Affiche l'en-tête `=== Cluster Scheduler (<policy name>) ===`.
4. Crée les pods dans cet ordre : `p1` (3 cpu), `p2` (5), `p3` (2), `p4` (4).
   Pour chacun, appelle `schedule` et affiche :
   - succès : `scheduled <pod> (<cpu> cpu) -> <node>` (lis le nœud via `assigned_node`)
   - échec : `failed to schedule <pod>: <erreur>`
5. Affiche l'état du cluster (`Display` de chaque nœud).
6. Affiche la capacité totale et l'usage total du cluster via `sum_by`.
7. Démontre la gestion d'erreur :
   - recherche un nœud inexistant `ghost`
   - tente une sur-allocation de `5` cpu sur `node-a`

---

## Output attendu

```
=== Cluster Scheduler (first-fit) ===

scheduled p1 (3 cpu) -> node-a
scheduled p2 (5 cpu) -> node-b
scheduled p3 (2 cpu) -> node-b
failed to schedule p4: no node available

--- Cluster state ---
node-a: 3/4 cpu
node-b: 7/8 cpu

total capacity: 12 cpu
total used: 10 cpu

--- Error handling ---
lookup ghost: node not found: ghost
over-allocate node-a by 5: not enough resources on node-a (requested 5, available 1)
```

---

## Pistes

- Trace le placement à la main : `p1(3)` tient sur `node-a` (reste 1) ; `p2(5)` ne tient
  plus sur `node-a`, va sur `node-b` (reste 3) ; `p3(2)` tient sur `node-b` (reste 1) ;
  `p4(4)` ne tient nulle part → `no node available`.
- `available()` fait un `borrow()` ; `allocate()` fait un `borrow_mut()`. Ne garde jamais
  les deux emprunts vivants en même temps, sinon panic à l'exécution.
- Pour propager les erreurs, `?` fonctionne dès que `schedule`/`node` renvoient
  `Result<_, ClusterError>`.
- `sum_by(&nodes, |n| n.cpu_total)` pour la capacité ; `sum_by(&nodes, |n| *n.cpu_used.borrow())`
  pour l'usage — la même fonction générique, deux closures différentes.
- Le pod stocke un `Rc<Node>` **cloné** : le nœud est donc partagé entre `scheduler.nodes`
  et le pod. C'est exactement le rôle de `Rc`.
- Depuis un module, les items d'un module frère se référencent via `crate::` ou `super::`.
- **Extension** (facultative, hors output) : ajoute une politique `BestFit` (le nœud au
  `available()` le plus serré qui tient encore) et échange-la dans le `Scheduler` — c'est
  tout l'intérêt du `Box<dyn SchedulingPolicy>`.

---

## Lancer l'exercice

```sh
cargo run -p ex16-integration
cargo test -p ex16-integration
```
