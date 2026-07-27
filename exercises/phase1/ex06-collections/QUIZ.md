# QUIZ — ex06 : Vec & HashMap

> 8 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Quelle est la différence entre ces deux lignes ?

```rust
let a = inventaire[10];        // (1)
let b = inventaire.get(10);    // (2)
```

- [ ] **A.** Aucune, `get` est juste plus verbeux
- [ ] **B.** (1) **panique** si l'index est hors bornes ; (2) rend `Option<&Item>`
- [ ] **C.** (1) ne compile pas
- [ ] **D.** (2) panique, (1) rend `None`

<details><summary>Réponse</summary>

**B.** L'indexation `[]` est un raccourci qui **panique** hors bornes (`index out of bounds: the len is 4 but the index is 10`). `.get()` rend un `Option<&T>` que tu es forcé de traiter.

Nuance qui coûte cher : si `Item` n'est pas `Copy`, la ligne (1) ne compile même pas — `cannot move out of index`. Il faudrait `&inventaire[10]`. `.get()` n'a pas ce problème, il rend déjà une référence.

Règle : `[]` quand l'index est prouvé valide (issu d'un `.len()`, d'une `.position()`), `.get()` sinon.

</details>

---

### Q2. 🔥 Ce code compile-t-il ?

```rust
let inventaire = vec![item1, item2];
for item in inventaire {
    println!("{}", item.display());
}
println!("{} items", inventaire.len());
```

- [ ] **A.** Oui
- [ ] **B.** Non : `borrow of moved value: inventaire`
- [ ] **C.** Non : il faut `mut`
- [ ] **D.** Oui, avec un warning

<details><summary>Réponse</summary>

**B.** `for x in collection` appelle `IntoIterator::into_iter(collection)` qui **consomme** le `Vec`. Après la boucle, `inventaire` n'existe plus.

Les trois formes, à connaître par cœur :

| Écriture | Appelle | Item | Après la boucle |
|---|---|---|---|
| `for i in v` | `into_iter()` | `Item` (owned) | `v` est **mort** |
| `for i in &v` | `iter()` | `&Item` | `v` intact |
| `for i in &mut v` | `iter_mut()` | `&mut Item` | `v` intact, modifié |

Le fix ici : `for item in &inventaire`. C'est la forme par défaut dans 90 % des cas.

</details>

---

### Q3. 🔥 Ce code compile-t-il ?

```rust
let mut v = vec![1, 2, 3];
for x in &v {
    if *x == 2 { v.push(99); }
}
```

- [ ] **A.** Oui, mais boucle à l'infini
- [ ] **B.** Non : `cannot borrow v as mutable because it is also borrowed as immutable`
- [ ] **C.** Oui
- [ ] **D.** Panique à l'exécution

<details><summary>Réponse</summary>

**B** — `E0502`, à la **compilation**.

`&v` maintient un emprunt immutable pendant toute la boucle ; `push` exige un `&mut`. C'est exactement le cas de ex03/Q3, mais à l'échelle d'une itération.

Et c'est une vraie protection : dans d'autres langages, modifier une collection pendant qu'on l'itère produit un comportement indéfini, une exception à l'exécution, ou pire, des éléments silencieusement sautés. Ici c'est impossible par construction.

Les contournements idiomatiques : collecter les ajouts dans un `Vec` temporaire puis `v.extend(temp)`, ou `v.retain(|x| ...)` pour les suppressions.

</details>

---

### Q4. 🔥 Que retourne `insert` sur une `HashMap` ?

```rust
let mut m = HashMap::new();
let a = m.insert("Épée", 10);
let b = m.insert("Épée", 20);
println!("{a:?} {b:?} {:?}", m.get("Épée"));
```

- [ ] **A.** `() () Some(20)`
- [ ] **B.** `None Some(10) Some(20)`
- [ ] **C.** `None None Some(10)` — la 2ᵉ insertion est refusée
- [ ] **D.** Panique : clé dupliquée

<details><summary>Réponse</summary>

**B.** `insert` **écrase** et rend `Option<V>` : l'**ancienne** valeur si la clé existait, `None` sinon.

Le pattern qu'on utilise vraiment en pratique, c'est l'API `entry` :

```rust
// insérer seulement si absent
m.entry("Épée").or_insert(10);

// accumuler — le classique du comptage
*counts.entry(mot).or_insert(0) += 1;

// construire un Vec par catégorie (exactement ton étape 3 !)
par_categorie.entry("Armes".to_string()).or_default().push(item);
```

`or_default()` évite de construire un `Vec::new()` inutile à chaque appel, contrairement à `or_insert(Vec::new())`.

</details>

---

### Q5. L'énoncé dit « l'ordre d'affichage des catégories n'est pas garanti ». Pourquoi ?

- [ ] **A.** Un bug de la stdlib
- [ ] **B.** `HashMap` n'a aucun ordre, et le seed de hachage est **aléatoire à chaque exécution**
- [ ] **C.** L'ordre dépend de l'ordre d'insertion
- [ ] **D.** L'ordre est alphabétique mais l'accentuation le perturbe

<details><summary>Réponse</summary>

**B.** `HashMap` utilise SipHash avec un seed **aléatoire par instance**, tiré au démarrage du processus. Deux exécutions du même binaire donnent deux ordres différents. C'est une protection contre les attaques par collision de hachage (HashDoS).

Conséquence directe : **ne jamais écrire un test qui dépend de l'ordre d'une `HashMap`**. Si tu as besoin d'un ordre :
- `BTreeMap` — trié par clé, même API
- `Vec<(K, V)>` — ordre d'insertion
- collecter les clés puis `.sort()` avant d'afficher

C'est pour ça que le `tests/output.rs` de cet exercice ne peut pas comparer la sortie brute au caractère près sur la partie HashMap.

</details>

---

### Q6. 🔥 Ce code compile-t-il ?

```rust
let mut m: HashMap<String, u32> = HashMap::new();
m.insert(String::from("Épée"), 10);
println!("{:?}", m.get("Épée"));
```

- [ ] **A.** Non : la clé est une `String`, il faut `m.get(&String::from("Épée"))`
- [ ] **B.** Oui
- [ ] **C.** Non : `get` prend la clé par valeur
- [ ] **D.** Oui, mais avec une allocation cachée

<details><summary>Réponse</summary>

**B.** La signature réelle est :

```rust
fn get<Q>(&self, k: &Q) -> Option<&V> where K: Borrow<Q>, Q: Hash + Eq + ?Sized
```

Comme `String: Borrow<str>`, tu peux interroger une `HashMap<String, V>` avec un simple `&str` — **sans allouer**. Même chose pour `Vec<T>` interrogé via `&[T]`, ou `contains_key`, `remove`, `entry`… (sauf `entry`, qui exige bien une `K` owned puisqu'il peut insérer).

Le réflexe : ne construis **jamais** une `String` juste pour faire un lookup.

</details>

---

### Q7. Pourquoi ce code échoue-t-il, alors que la struct `Item` est bien définie ?

```rust
let mut par_categorie: HashMap<String, Vec<Item>> = HashMap::new();
par_categorie.insert("Armes".to_string(), vec![epee]);
par_categorie.insert("Armures".to_string(), vec![epee]);
```

- [ ] **A.** `HashMap` refuse deux clés différentes
- [ ] **B.** `use of moved value: epee` — `epee` a été déplacée dans le premier `vec!`
- [ ] **C.** Il manque `#[derive(Clone)]`
- [ ] **D.** Le type de la valeur est incorrect

<details><summary>Réponse</summary>

**B.** `vec![epee]` prend l'ownership de l'`Item`. La seconde ligne emprunte une valeur déplacée.

Trois réponses possibles, par ordre de qualité :
1. **repenser la structure** — un item n'appartient qu'à une catégorie ; construis chaque `Vec` avec des items distincts (c'est ce que fait l'énoncé)
2. `#[derive(Clone)]` + `epee.clone()` — duplique réellement les données, acceptable ici mais ça ment sur le modèle
3. `HashMap<String, Vec<&Item>>` — stocke des références, mais tu hérites des lifetimes (ex10)

En phase 2, tu découvriras la vraie réponse quand un objet doit être partagé : `Rc<Item>` (ex15).

</details>

---

### Q8. 🔥 Quel type a `total` et ce code compile-t-il ?

```rust
let total = inventaire.iter().map(|i| i.value).sum();
println!("Valeur totale : {total} gold");
```

- [ ] **A.** `u32`, compile
- [ ] **B.** Ne compile pas : `type annotations needed`
- [ ] **C.** `i32` par défaut, compile
- [ ] **D.** `usize`, compile

<details><summary>Réponse</summary>

**B.** Même piège qu'en ex02/Q2 : `sum()` est générique sur sa sortie (`fn sum<S: Sum<Self::Item>>()`). Le fait que `i.value` soit un `u32` ne détermine **pas** le résultat — on peut sommer des `u32` dans un `u64`.

Les deux fixes :
```rust
let total: u32 = inventaire.iter().map(|i| i.value).sum();
let total = inventaire.iter().map(|i| i.value).sum::<u32>();
```

Le turbofish `::<u32>` te suivra dans tout le cursus, notamment sur `collect::<Vec<_>>()` et `parse::<f32>()`.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | Collections maîtrisées. Q4 (`entry`) et Q6 (`Borrow`) te feront écrire du code plus propre que la moyenne |
| 6-7/8 | Bien. Fixe le tableau de Q2 (`iter` / `into_iter` / `iter_mut`), il resservira sans arrêt |
| < 6 | Reprends Q2 et Q3 : le rapport ownership ↔ boucles est le socle de ex11 |
