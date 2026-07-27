# QUIZ — ex16 : Orchestrateur (intégration phase 2)

> 9 questions. Quiz d'intégration : chaque question relie deux exercices.
> 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Pourquoi `policy: Box<dyn SchedulingPolicy>` et non `policy: P` générique ?

```rust
struct Scheduler { nodes: Vec<Rc<Node>>, policy: Box<dyn SchedulingPolicy> }
// vs
struct Scheduler<P: SchedulingPolicy> { nodes: Vec<Rc<Node>>, policy: P }
```

- [ ] **A.** Les deux marchent, mais `dyn` permet de **changer de politique à l'exécution** sans changer le type du `Scheduler`
- [ ] **B.** La version générique ne compile pas
- [ ] **C.** `dyn` est plus rapide
- [ ] **D.** Un champ ne peut pas être générique

<details><summary>Réponse</summary>

**A.** Les deux compilent. La différence est de **conception** :

```rust
// générique : le type encode la politique
let s: Scheduler<FirstFit> = ...;
s.policy = BestFit;                   // ❌ mismatched types

// dyn : un seul type de Scheduler
let mut s = Scheduler::new(nodes, Box::new(FirstFit));
s.policy = Box::new(BestFit);         // ✅
```

Avec le générique, `Scheduler<FirstFit>` et `Scheduler<BestFit>` sont **deux types distincts** (cf. ex12/Q6) : impossible de les mettre dans le même `Vec`, ni de choisir la politique depuis un fichier de config à l'exécution.

C'est l'extension facultative de l'énoncé (`BestFit`) qui rend l'intérêt visible. Le coût : un saut indirect par appel à `choose` — négligeable face au travail réel.

**La règle** : générique par défaut ; `dyn` dès que la variante doit être choisie à l'exécution, ou que tu dois stocker des types hétérogènes (ex09/Q4).

</details>

---

### Q2. 🔥 Ce code panique-t-il ?

```rust
fn allocate(&self, cpu: u32) -> Result<(), ClusterError> {
    let mut used = self.cpu_used.borrow_mut();
    if cpu > self.available() {                    // available() fait un borrow()
        return Err(ClusterError::NotEnoughResources { ... });
    }
    *used += cpu;
    Ok(())
}
```

- [ ] **A.** Non, `borrow()` et `borrow_mut()` sur le même `RefCell` sont compatibles
- [ ] **B.** Oui : `already mutably borrowed` — le `RefMut` de la ligne 2 est encore vivant quand `available()` appelle `borrow()`
- [ ] **C.** Erreur de compilation
- [ ] **D.** Non, mais le résultat est faux

<details><summary>Réponse</summary>

**B.** `let mut used = ...borrow_mut()` crée un `RefMut` lié à une **variable**, donc vivant jusqu'à la fin de la fonction. `available()` tente un `borrow()` partagé sur le même `RefCell` → conflit → panic à l'exécution.

Écriture correcte : **ne prends l'emprunt qu'au moment de muter.**

```rust
fn allocate(&self, cpu: u32) -> Result<(), ClusterError> {
    let available = self.available();          // le Ref meurt à la fin de l'appel
    if cpu > available {
        return Err(ClusterError::NotEnoughResources {
            node: self.name.clone(), requested: cpu, available,
        });
    }
    *self.cpu_used.borrow_mut() += cpu;         // emprunt ponctuel, relâché immédiatement
    Ok(())
}
```

C'est la même leçon qu'ex15/Q4, sous une autre forme : **un emprunt lié à une variable vit longtemps, un emprunt temporaire meurt vite.** Préfère toujours le second.

</details>

---

### Q3. 🔥 Quel est le type de `n` dans cette closure ?

```rust
fn sum_by<T, F>(items: &[T], f: F) -> u32 where F: Fn(&T) -> u32 {
    items.iter().map(f).sum()
}

let nodes: Vec<Rc<Node>> = vec![...];
let capacity = sum_by(&nodes, |n| n.cpu_total);
```

- [ ] **A.** `Node`
- [ ] **B.** `Rc<Node>`
- [ ] **C.** `&Rc<Node>`
- [ ] **D.** `&Node`

<details><summary>Réponse</summary>

**C — `&Rc<Node>`.** `T` est inféré à `Rc<Node>` (le type d'élément de la slice), et `F: Fn(&T)` donne `&Rc<Node>`.

`n.cpu_total` fonctionne grâce à **deux** déréférencements automatiques enchaînés : `&Rc<Node>` → `Rc<Node>` → `Node`. Le compilateur les insère silencieusement pour l'accès aux champs et l'appel de méthode.

C'est exactement le mécanisme de ex14/Q4 avec `Box`. Tous les pointeurs intelligents implémentent `Deref`, et l'auto-deref les traverse en cascade.

L'autre appel de l'énoncé est plus subtil :

```rust
sum_by(&nodes, |n| *n.cpu_used.borrow())
```

Ici `n.cpu_used` traverse les deux couches, `.borrow()` rend un `Ref<u32>`, et le `*` extrait le `u32` (qui est `Copy`, donc c'est une copie — le `Ref` peut mourir en fin d'expression sans problème).

</details>

---

### Q4. Ce trait est-il utilisable en `Box<dyn SchedulingPolicy>` ?

```rust
trait SchedulingPolicy {
    fn choose(&self, nodes: &[Rc<Node>], cpu: u32) -> Option<Rc<Node>>;
    fn name(&self) -> &str;
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `Option<Rc<Node>>` n'est pas *dyn-compatible*
- [ ] **C.** Non : il manque `: Sized`
- [ ] **D.** Non : `name` doit rendre `String`

<details><summary>Réponse</summary>

**A — oui.** Les deux méthodes prennent `&self`, ne sont pas génériques, et ne mentionnent `Self` ni en paramètre ni en retour. Toutes les conditions de ex09/Q6 sont remplies.

Ce qui l'aurait cassé :

```rust
fn clone_policy(&self) -> Self;                       // ❌ -> Self
fn choose_from<I: Iterator<Item = Rc<Node>>>(&self, i: I);  // ❌ méthode générique
fn default_policy() -> Self;                          // ❌ pas de self
```

Peu importe ce que rendent les méthodes, tant que la signature a une taille connue : `Option<Rc<Node>>` fait 8 octets (niche du pointeur, cf. ex05/Q6), c'est parfaitement représentable dans une vtable.

</details>

---

### Q5. 🔥 Après `schedule(&p1)` où `p1` demande 3 cpu sur `node-a`, que voit le `Scheduler` ?

```rust
let node_a = Node::new("node-a", 4);            // Rc<Node>
let sched = Scheduler::new(vec![Rc::clone(&node_a)], Box::new(FirstFit));
sched.schedule(&p1).unwrap();                    // p1 stocke Rc::clone(node choisi)
println!("{}", node_a);                          // "node-a: ?/4 cpu"
```

- [ ] **A.** `node-a: 0/4 cpu` — chaque `Rc` a sa propre copie
- [ ] **B.** `node-a: 3/4 cpu` — tous les `Rc` pointent vers **le même** `Node`
- [ ] **C.** Erreur de compilation
- [ ] **D.** Panique

<details><summary>Réponse</summary>

**B.** C'est **tout le point** de l'exercice, et la raison pour laquelle `Rc` et `RefCell` sont ici ensemble.

Il n'existe qu'**un seul** `Node` en mémoire. Le `Rc` local, celui du `Scheduler`, et celui stocké dans `p1.node` sont trois pointeurs vers cet unique objet. `allocate()` mute son `cpu_used` via `RefCell`, et **tous** le voient instantanément.

Compare avec ce qu'il aurait fallu sans `Rc` :
- `Vec<Node>` dans le `Scheduler` + index dans le `Pod` → possible, mais l'index peut devenir invalide (indices fantômes)
- `&'a Node` dans le `Pod` → lifetimes virales dans tout le programme (`Pod<'a>`, `Scheduler<'a>`…)
- `Node` clonée dans le `Pod` → deux états divergents, bug garanti

`Rc<T>` achète le partage sans annotation de lifetime ; `RefCell<T>` achète la mutation sans `&mut`. Ensemble, ils modélisent un graphe d'objets mutables — au prix d'une vérification à l'exécution.

</details>

---

### Q6. Pourquoi `Node::new` rend-elle `Rc<Node>` alors que `Pod::new` rend un `Pod` nu ?

- [ ] **A.** Incohérence de l'énoncé
- [ ] **B.** Parce qu'un `Node` est **partagé** (scheduler + pods) alors qu'un `Pod` a un propriétaire unique
- [ ] **C.** Parce que `Pod` est plus petit
- [ ] **D.** Parce que `Pod` contient déjà un `RefCell`

<details><summary>Réponse</summary>

**B.** Le partage se décide **par la topologie du modèle**, pas par habitude :

- un `Node` est référencé par le `Scheduler` **et** par chaque `Pod` qui y tourne → plusieurs propriétaires → `Rc`
- un `Pod` n'est référencé que par `main` → un propriétaire → valeur nue

Renvoyer `Rc<Node>` **depuis le constructeur** est un choix d'API délibéré : il rend le partage obligatoire et empêche l'appelant de créer un `Node` nu qu'il faudrait ensuite emballer. C'est le pattern qu'on trouve dans les vraies bibliothèques d'arbres et de graphes.

La règle à emporter : **n'ajoute `Rc` que quand un second propriétaire existe réellement.** `Rc` par défaut, c'est du GC manuel — plus lent et plus confus qu'un ownership clair.

</details>

---

### Q7. 🔥 Ce code compile-t-il ?

```rust
mod scheduler {
    use super::node::Node;
    use super::error::ClusterError;

    pub fn total(nodes: &[std::rc::Rc<Node>]) -> Result<u32, ClusterError> {
        let n = crate::scheduler::find(nodes, "node-a")?;
        Ok(n.cpu_total)
    }
    fn find(nodes: &[std::rc::Rc<Node>], name: &str) -> Result<std::rc::Rc<Node>, ClusterError> { ... }
}
```

`find` est **privée** et appelée via `crate::scheduler::find`.

- [ ] **A.** Non : `find` est privée, un chemin absolu ne peut pas y accéder
- [ ] **B.** Oui : la visibilité dépend de **qui appelle**, pas de la forme du chemin — et l'appelant est `scheduler` lui-même
- [ ] **C.** Non : il faut `self::find`
- [ ] **D.** Non : `crate::` est interdit dans un `main.rs`

<details><summary>Réponse</summary>

**B.** La forme du chemin (`self::`, `super::`, `crate::`, ou relatif) est purement **syntaxique**. La règle de visibilité de ex13/Q5 s'applique au **site d'appel** : `find` est privée à `scheduler`, et l'appel se fait depuis `scheduler` → autorisé.

Le style idiomatique ici serait `find(nodes, "node-a")` tout court, ou `self::find(...)`. `crate::scheduler::find` est correct mais verbeux.

Pour les modules **frères** (`scheduler` → `node`), l'énoncé recommande `super::node::Node` ou `crate::node::Node`. `super::` résiste mieux à un déplacement de tout le sous-arbre de modules ; `crate::` est plus lisible quand on est profondément imbriqué.

</details>

---

### Q8. Pourquoi `schedule` prend-elle `&self` et non `&mut self` ?

```rust
fn schedule(&self, pod: &Pod) -> Result<(), ClusterError>
```

Elle modifie pourtant le `cpu_used` d'un nœud et le `node` du pod.

- [ ] **A.** Erreur de l'énoncé
- [ ] **B.** Parce que **toutes** les mutations passent par des `RefCell` — le `Scheduler` lui-même ne change pas
- [ ] **C.** Parce que `Pod` est immuable
- [ ] **D.** Pour permettre plusieurs appels

<details><summary>Réponse</summary>

**B.** Regarde ce qui change réellement :
- `node.cpu_used` — un `RefCell<u32>`, muté via `borrow_mut()`
- `pod.node` — un `RefCell<Option<Rc<Node>>>`, idem

Les champs du `Scheduler` (`nodes`, `policy`) ne bougent pas. Aucun `&mut` n'est nécessaire.

Et ce n'est pas un détail cosmétique : `&mut self` serait **impossible à obtenir** si le `Scheduler` était lui-même derrière un `Rc`. La mutabilité intérieure est ce qui rend le graphe d'objets manipulable.

Contraste utile avec ex11, où `Shop::sell(&mut self, ...)` mute directement `self.inventory` et `self.gold`. Là, un `&mut` classique suffisait, et c'était **mieux** : vérification à la compilation, zéro coût. Ne passe à `RefCell` que quand `&mut` est hors d'atteinte.

</details>

---

### Q9. 🔥 Trace le placement. Pourquoi `p4` (4 cpu) échoue-t-il alors que le cluster a 12 cpu au total ?

Nœuds : `node-a` (4), `node-b` (8). Pods dans l'ordre : `p1`(3), `p2`(5), `p3`(2), `p4`(4).

- [ ] **A.** Bug de `FirstFit`
- [ ] **B.** Parce que `FirstFit` place **gloutonnement** sans anticiper : il reste 1+1 = 2 cpu libres, mais **fragmentés** sur deux nœuds
- [ ] **C.** Parce que `p4` dépasse `cpu_total`
- [ ] **D.** Parce que le cluster est plein

<details><summary>Réponse</summary>

**B.** Le déroulé :

| Pod | node-a (4) | node-b (8) | Décision |
|---|---|---|---|
| `p1` (3) | 3/4 → **1 libre** | 0/8 | node-a (premier qui tient) |
| `p2` (5) | 1 libre → non | 5/8 → **3 libres** | node-b |
| `p3` (2) | 1 libre → non | 7/8 → **1 libre** | node-b |
| `p4` (4) | 1 libre → non | 1 libre → non | `NoNodeAvailable` |

Total libre : 2 cpu. Mais **répartis sur deux nœuds**, et un pod n'est pas divisible. C'est la **fragmentation**, le problème central de tout ordonnanceur réel (Kubernetes, Slurm, Nomad).

Ce que ça montre : un ordonnancement glouton est **dépendant de l'ordre**. Avec l'ordre `p2, p4, p1, p3` : node-b prend p2 (3 libres), node-a prend p4 (0 libres), node-b prend p1 (0 libres), p3 échoue. Autre ordre, autre résultat — mais toujours un échec.

C'est précisément pourquoi `Box<dyn SchedulingPolicy>` existe (Q1) : `BestFit` (le nœud au reste le plus serré qui tient encore) laisserait ici les mêmes 2 cpu fragmentés, mais gère mieux d'autres distributions. Aucune politique gloutonne n'est optimale — le problème est NP-difficile (*bin packing*).

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 9/9 | Phase 2 bouclée. Tu sais **choisir** entre générique et `dyn`, entre `&mut` et `RefCell` |
| 7-8/9 | Solide. Reviens sur Q2 (le `RefMut` en variable) — c'est le panic que tu auras en écrivant l'exercice |
| < 7 | Reprends Q1, Q5, Q8 : ce sont les trois décisions de conception que l'exercice t'apprend |
