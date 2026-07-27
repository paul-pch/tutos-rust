# QUIZ — ex15 : `Rc<T>` & `RefCell<T>`

> 9 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Que fait `Rc::clone(&a)` ?

- [ ] **A.** Duplique la donnée pointée
- [ ] **B.** Incrémente un compteur et rend un second pointeur vers **la même** donnée
- [ ] **C.** Crée une référence `&T`
- [ ] **D.** Déplace la donnée

<details><summary>Réponse</summary>

**B.** `Rc` = *Reference Counted*. `Rc::clone` copie 8 octets (le pointeur) et incrémente le compteur `strong`. La donnée n'est **jamais** dupliquée.

Pourquoi le README insiste sur `Rc::clone(&a)` plutôt que `a.clone()` alors que c'est strictement identique ? Pure **lisibilité** : dans une revue de code, `x.clone()` fait craindre une copie profonde coûteuse. `Rc::clone(&x)` annonce sans ambiguïté « j'incrémente un compteur, c'est gratuit ».

Le piège inverse, bien réel : si `T: Clone`, alors `rc.clone()` reste `Rc::clone` (l'impl inhérente gagne), mais `(*rc).clone()` fait, lui, une vraie copie profonde. Un `*` de trop et ta perf s'effondre.

</details>

---

### Q2. Ce code compile-t-il ?

```rust
use std::rc::Rc;
let a = Rc::new(String::from("shared"));
a.push_str("!");
```

- [ ] **A.** Oui
- [ ] **B.** Non : `Rc<T>` ne donne qu'un accès **immuable** — pas de `&mut T`
- [ ] **C.** Non : il faut `let mut a`
- [ ] **D.** Oui, mais les autres `Rc` ne verront pas la modification

<details><summary>Réponse</summary>

**B.** `Rc<T>` implémente `Deref<Target = T>` mais **pas** `DerefMut`. C'est fondamental : plusieurs propriétaires existent simultanément, donner un `&mut` violerait la règle d'exclusivité de ex03.

Note que `let mut a` ne changerait rien — la mutabilité de la liaison ne perce pas le `Rc`.

Deux sorties :
- `RefCell<T>` **à l'intérieur** du `Rc` → `Rc<RefCell<T>>`, la combinaison de l'exercice
- `Rc::get_mut(&mut a)` → `Option<&mut T>`, qui rend `Some` **seulement** si le compteur vaut exactement 1. Sûr, mais inutilisable dès qu'il y a du partage réel

</details>

---

### Q3. 🔥 Quelle est la sortie ?

```rust
let disk = Service::new("disk", vec![]);
let database = Service::new("database", vec![Rc::clone(&disk)]);
let api = Service::new("api", vec![Rc::clone(&database)]);
let web = Service::new("web", vec![Rc::clone(&database)]);

println!("{} {}", Rc::strong_count(&disk), Rc::strong_count(&database));
```

- [ ] **A.** `1 1`
- [ ] **B.** `2 3`
- [ ] **C.** `1 2`
- [ ] **D.** `4 3`

<details><summary>Réponse</summary>

**B — `2 3`**, ce qui correspond bien à l'output attendu de l'exercice.

Compte les **liaisons vivantes**, pas les usages :

| Cible | Détenteurs |
|---|---|
| `disk` | la variable locale `disk` (1) + le clone dans `database.deps` (2) → **2** |
| `database` | la variable locale `database` (1) + le clone dans `api.deps` (2) + le clone dans `web.deps` (3) → **3** |

Le compteur descend quand un `Rc` est *droppé*. Si tu insérais `drop(disk);` avant l'affichage, tu lirais `1`… mais tu ne pourrais plus écrire `Rc::strong_count(&disk)`. Pour observer la décrémentation, il faut un `Rc` témoin dans un scope imbriqué.

`Rc::weak_count` existe aussi, et compte les `Weak` (cf. Q8).

</details>

---

### Q4. 🔥 Ces deux méthodes se comportent-elles pareil ?

```rust
fn a(&self) {
    if *self.status.borrow() == Status::Up { self.set_status(Status::Down); }
}

fn b(&self) {
    match *self.status.borrow() {
        Status::Up => self.set_status(Status::Down),
        _ => {}
    }
}
```

avec `fn set_status(&self, s: Status) { *self.status.borrow_mut() = s; }`

- [ ] **A.** Oui, les deux fonctionnent
- [ ] **B.** Oui, les deux paniquent
- [ ] **C.** `a` fonctionne, `b` **panique** : `RefCell already borrowed`
- [ ] **D.** `b` fonctionne, `a` panique

<details><summary>Réponse</summary>

**C.** Vérifié à l'exécution. C'est **le** piège de `RefCell`, et il est purement une question de **portée des temporaires** :

- dans `if cond { … }`, les temporaires de `cond` sont libérés **à la fin de la condition**. Le `Ref` est mort avant que le corps ne s'exécute → ✅
- dans `match scrutin { … }`, les temporaires du scrutin vivent jusqu'à la **fin de tout le `match`**. Le `Ref` est encore vivant quand `set_status` appelle `borrow_mut()` → 💥

```
thread 'main' panicked at: RefCell already borrowed
```

Ce n'est pas un bug : le scrutin doit rester vivant pour que les liaisons du pattern puissent l'emprunter.

Les parades :

```rust
// (1) copier la valeur, refermer l'emprunt tout de suite
let current = *self.status.borrow();          // Status: Copy
match current { Status::Up => self.set_status(Status::Down), _ => {} }

// (2) isoler l'emprunt dans un bloc
let is_up = { matches!(*self.status.borrow(), Status::Up) };
if is_up { self.set_status(Status::Down); }
```

Règle générale : **ne jamais laisser un `borrow()` vivant pendant qu'on appelle une méthode qui pourrait re-emprunter.** Et comme tu ne sais pas toujours ce que fait la méthode appelée, extrais la valeur au plus tôt.

</details>

---

### Q5. Quelle est la différence fondamentale entre le borrow checker et `RefCell` ?

- [ ] **A.** Aucune, `RefCell` est plus rapide
- [ ] **B.** Le borrow checker vérifie à la **compilation** (erreur) ; `RefCell` vérifie à l'**exécution** (panic)
- [ ] **C.** `RefCell` désactive les règles d'emprunt
- [ ] **D.** `RefCell` autorise plusieurs `&mut` simultanés

<details><summary>Réponse</summary>

**B.** Les **règles sont identiques** — N lecteurs XOR 1 écrivain. Seul le **moment** de la vérification change.

| | Borrow checker | `RefCell` |
|---|---|---|
| Vérification | compilation | exécution |
| Échec | erreur `E0502` | **panic** |
| Coût runtime | zéro | un compteur `isize` + un test par emprunt |
| Souplesse | conservateur | accepte tout ce qui est réellement correct |

Le vrai argument pour `RefCell` : le borrow checker est **conservateur**. Il refuse des programmes corrects qu'il ne sait pas prouver. `RefCell` te laisse dire « je sais que c'est bon, vérifie-le à l'exécution ».

Le prix : un programme qui aurait été refusé à la compilation devient un **crash en production**. À n'utiliser que quand tu ne peux vraiment pas faire autrement — pas pour contourner un emprunt que tu n'as pas envie de réorganiser.

Note : `RefCell<T>` n'est **pas** `Sync` — l'équivalent multi-thread est `Mutex<T>` ou `RwLock<T>`, où le conflit bloque au lieu de paniquer.

</details>

---

### Q6. 🔥 Pourquoi `Rc<RefCell<T>>` et non `RefCell<Rc<T>>` ?

- [ ] **A.** C'est équivalent
- [ ] **B.** `Rc<RefCell<T>>` = plusieurs propriétaires d'une donnée **mutable partagée** ; `RefCell<Rc<T>>` = une cellule mutable qui change **quel** `Rc` elle contient
- [ ] **C.** `RefCell<Rc<T>>` ne compile pas
- [ ] **D.** `Rc<RefCell<T>>` est plus rapide

<details><summary>Réponse</summary>

**B.** L'ordre d'imbrication change complètement la sémantique. Lis toujours **de l'extérieur vers l'intérieur** :

```rust
Rc<RefCell<T>>     // « un T partagé, dont le contenu est mutable »
                   // tous les détenteurs voient les mutations ✅

RefCell<Rc<T>>     // « un emplacement mutable qui contient un pointeur vers un T »
                   // on peut changer VERS QUI on pointe, pas ce qui est pointé
```

Les deux compilent, et les deux sont utiles — mais pour des besoins opposés. L'exercice utilise d'ailleurs **les deux** :

```rust
status: RefCell<Status>          // ex15 : le statut change, la struct est derrière &self
deps: Vec<Rc<Service>>           // ex15 : chaque service est partagé

node: RefCell<Option<Rc<Node>>>  // ex16 : on change à QUEL nœud le pod est assigné
```

Ce dernier est un `RefCell<Option<Rc<...>>>` : trois couches, chacune avec un rôle précis (mutable / peut être absent / partagé).

</details>

---

### Q7. Ce code compile-t-il ?

```rust
let counter = Rc::new(RefCell::new(0));
let view = Rc::clone(&counter);

std::thread::spawn(move || {
    *view.borrow_mut() += 1;
});
```

- [ ] **A.** Oui
- [ ] **B.** Non : `Rc<RefCell<i32>> cannot be sent between threads safely` — ni `Rc` ni `RefCell` ne sont thread-safe
- [ ] **C.** Oui, mais avec une *data race*
- [ ] **D.** Non : il manque `move`

<details><summary>Réponse</summary>

**B.** Et c'est exactement le genre de garantie qui fait l'intérêt de Rust : **la data race est une erreur de compilation**, pas un bug qu'on découvre en prod un vendredi soir.

Deux raisons distinctes :
- `Rc` incrémente son compteur **sans atomique** (c'est ce qui le rend rapide). Deux threads → compteur corrompu → double-free ou fuite. Donc `Rc: !Send`
- `RefCell` fait pareil avec son compteur d'emprunts. Donc `RefCell: !Sync`

Les équivalents multi-thread :

| Mono-thread | Multi-thread | Différence |
|---|---|---|
| `Rc<T>` | `Arc<T>` | compteur **atomique** |
| `RefCell<T>` | `Mutex<T>` / `RwLock<T>` | **bloque** au lieu de paniquer |
| `Rc<RefCell<T>>` | `Arc<Mutex<T>>` | l'idiome canonique du partage entre threads |

Tu ne paies l'atomique que si tu en as besoin — c'est pourquoi les deux familles coexistent. Sujet direct de ex17.

</details>

---

### Q8. 🔥 Que se passe-t-il ici ?

```rust
struct Node { parent: RefCell<Option<Rc<Node>>>, children: RefCell<Vec<Rc<Node>>> }

let parent = Rc::new(Node { parent: RefCell::new(None), children: RefCell::new(vec![]) });
let child  = Rc::new(Node { parent: RefCell::new(Some(Rc::clone(&parent))), children: RefCell::new(vec![]) });
parent.children.borrow_mut().push(Rc::clone(&child));
```

- [ ] **A.** Rien de spécial
- [ ] **B.** Un **cycle** de `Rc` : les deux compteurs ne retombent jamais à 0 → **fuite mémoire**
- [ ] **C.** Panique à l'exécution
- [ ] **D.** Erreur de compilation

<details><summary>Réponse</summary>

**B.** `parent` détient `child`, `child` détient `parent`. À la sortie de scope, chaque compteur passe de 2 à 1 — jamais à 0. Les deux nœuds ne sont **jamais** libérés.

C'est **la** limite du comptage de références, et Rust ne la résout pas : la fuite mémoire n'est pas considérée comme un problème de sûreté (`mem::forget` est d'ailleurs `safe`). Ton programme reste correct, il consomme juste de la mémoire pour toujours.

La solution est `Weak<T>` : un pointeur qui **n'incrémente pas** le compteur `strong`.

```rust
struct Node {
    parent: RefCell<Weak<Node>>,        // ← lien remontant faible
    children: RefCell<Vec<Rc<Node>>>,   // ← lien descendant fort
}

// pour l'utiliser, il faut le "réveiller" :
if let Some(p) = node.parent.borrow().upgrade() {   // Option<Rc<Node>>
    println!("{}", p.name);
}
```

**La règle générale : le sens de la propriété est `Rc`, le sens inverse est `Weak`.** Parent → enfant fort, enfant → parent faible. `upgrade()` rend `None` si la cible a été libérée entre-temps — ce qui est exactement l'information que tu veux.

Ton graphe de services est acyclique (api → database → disk), donc pas de fuite. Mais ajoute une dépendance circulaire et tu y es.

</details>

---

### Q9. Pourquoi `set_status` prend-elle `&self` et non `&mut self` ?

```rust
fn set_status(&self, status: Status) { *self.status.borrow_mut() = status; }
```

- [ ] **A.** Par erreur de l'énoncé
- [ ] **B.** Parce que la mutation passe par le `RefCell` — et surtout parce qu'un `Rc<Service>` **ne peut pas** donner de `&mut Service`
- [ ] **C.** Pour être plus rapide
- [ ] **D.** Parce que `Status` est `Copy`

<details><summary>Réponse</summary>

**B**, et c'est la raison d'être de la *mutabilité intérieure*.

Les services sont stockés en `Rc<Service>`. Or `Rc` ne fournit que `&Service` (Q2). Une méthode `&mut self` serait donc **inappelable** — tu ne pourrais jamais changer un statut.

`RefCell` déplace la question : la struct entière reste immuable du point de vue du système de types, mais le champ `status` est mutable via son propre mécanisme, vérifié à l'exécution.

La signature `fn f(&self)` qui mute est le **signal** de l'*interior mutability* dans tout l'écosystème. Tu la retrouveras sur `Cell`, `RefCell`, `Mutex`, `AtomicUsize`, `OnceCell` — tous marqués par le type `UnsafeCell` en interne, le seul moyen légal en Rust d'obtenir un `&mut` depuis un `&`.

C'est aussi le mécanisme derrière `Node::allocate(&self, cpu)` en ex16.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 9/9 | Excellent — Q4 et Q8 sont des bugs qu'on ne comprend qu'après les avoir vécus |
| 7-8/9 | Solide. Ancre Q4 (`match` garde le `borrow` vivant) : c'est le panic que tu auras en ex16 |
| < 7 | Reprends Q5 puis Q4. Tant que « mêmes règles, moment de vérification différent » n'est pas clair, `RefCell` reste de la magie |
