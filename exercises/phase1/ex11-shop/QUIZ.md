# QUIZ — ex11 : Itérateurs & closures (intégration)

> 9 questions — exercice d'intégration, quiz d'intégration.
> 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Que fait ce code ?

```rust
shop.inventory.iter().map(|i| println!("{i}"));
println!("fin");
```

- [ ] **A.** Affiche tous les items, puis `fin`
- [ ] **B.** Affiche seulement `fin`, avec un warning
- [ ] **C.** Erreur de compilation
- [ ] **D.** Affiche `fin` puis les items

<details><summary>Réponse</summary>

**B.** Les itérateurs Rust sont **paresseux** : `map` construit une structure `Map<...>` et ne fait **rien** tant qu'un consommateur ne tire pas dessus.

Le compilateur t'avertit : *`unused Map that must be used — iterators are lazy and do nothing unless consumed`*. Mais c'est un warning, donc ton programme tourne et n'affiche rien.

Les **consommateurs** (qui déclenchent l'itération) : `collect`, `sum`, `count`, `for_each`, `fold`, `max`, `find`, `any`, `all`, `last`, et la boucle `for`.

Pour un effet de bord, `for_each` ou une boucle `for` — jamais `map`.

</details>

---

### Q2. 🔥 Quel est le type de `i` dans la closure ?

```rust
let count = shop.inventory
    .iter()
    .filter(|i| i.category == ItemCategory::Weapon)
    .count();
```

- [ ] **A.** `Item`
- [ ] **B.** `&Item`
- [ ] **C.** `&&Item`
- [ ] **D.** `Option<&Item>`

<details><summary>Réponse</summary>

**C — `&&Item`.** Double référence :
- `.iter()` produit des `&Item`
- `filter` reçoit `FnMut(&Self::Item) -> bool`, donc `&(&Item)` = `&&Item`

Pourquoi ça passe quand même ? L'**auto-deref** sur l'accès aux champs : `i.category` déréférence autant de niveaux que nécessaire.

Là où ça mord, c'est quand tu dois écrire le déréférencement toi-même :

```rust
.filter(|i| i.value > 50)          // ✅ auto-deref sur le champ
.map(|i| *i)                       // ❌ un seul * ne suffit pas : &&Item → &Item
.filter(|&i| i.value > 50)         // ✅ pattern qui déballe un niveau
.copied() / .cloned()              // ✅ pour passer de &T à T
```

Et c'est pour ça que `.filter(...).map(|i| ...)` change le nombre de `&` par rapport à `.map(|i| ...)` direct — un classique des messages d'erreur cryptiques.

</details>

---

### Q3. Quelle est la différence entre `Fn`, `FnMut` et `FnOnce` ?

- [ ] **A.** Le nombre d'arguments acceptés
- [ ] **B.** Ce que la closure fait de son **environnement capturé** : lecture / mutation / consommation
- [ ] **C.** La vitesse d'exécution
- [ ] **D.** `FnOnce` est déprécié

<details><summary>Réponse</summary>

**B.**

| Trait | La closure… | Appelable |
|---|---|---|
| `FnOnce` | **consomme** ce qu'elle capture | une seule fois |
| `FnMut` | **mute** ce qu'elle capture | plusieurs fois, en `&mut` |
| `Fn` | ne fait que **lire** | plusieurs fois, en `&` |

Ils sont **emboîtés** : `Fn` ⊂ `FnMut` ⊂ `FnOnce`. Une closure `Fn` satisfait donc n'importe quelle borne.

Tu ne les écris jamais — le compilateur déduit le plus permissif possible en analysant le corps. Ils n'apparaissent que dans les **signatures** que tu écris :

```rust
fn sum_by<T, F>(items: &[T], f: F) -> u32 where F: Fn(&T) -> u32   // ex16
```

Ici `Fn` est le bon choix : `map` va appeler `f` une fois par élément, et `f` n'a rien à muter. Exiger `FnOnce` interdirait la boucle ; exiger `FnMut` marcherait aussi mais serait moins permissif pour l'appelant.

</details>

---

### Q4. 🔥 Quel item est renvoyé ?

```rust
let items = [("Sword", 120), ("Wand", 120), ("Rope", 5)];
let best = items.iter().max_by_key(|i| i.1);
```

- [ ] **A.** `Some(("Sword", 120))` — le premier
- [ ] **B.** `Some(("Wand", 120))` — le **dernier**
- [ ] **C.** `None` — égalité ambiguë
- [ ] **D.** Indéterminé

<details><summary>Réponse</summary>

**B — le dernier.** Vérifié :

```
max_by_key → Some(("Wand", 120))
min_by_key → le PREMIER des minimums
```

C'est documenté et asymétrique, volontairement : ainsi `max_by_key` et `min_by_key` appliqués au même itérateur ne rendent jamais le même élément quand tout est à égalité.

Pourquoi ça compte : ton `tests/output.rs` compare la sortie **caractère par caractère**. Si deux items partagent la valeur maximale, l'item affiché par `most_valuable()` dépend de cette règle — pas de ton intuition. Dans l'énoncé, `Sword` (120) est strictement le plus cher, donc pas de piège… tant que tu ne changes pas les données.

</details>

---

### Q5. Ce code compile-t-il ?

```rust
let total = shop.inventory
    .iter()
    .map(|i| i.weight * i.quantity)
    .sum::<f32>();
```

avec `weight: f32` et `quantity: u32`.

- [ ] **A.** Oui
- [ ] **B.** Non : `cannot multiply f32 by u32`
- [ ] **C.** Non : `sum` ne marche pas sur `f32`
- [ ] **D.** Oui, `quantity` est promu en `f32`

<details><summary>Réponse</summary>

**B.** Aucune promotion numérique implicite en Rust (cf. ex02/Q6). Il faut caster explicitement :

```rust
.map(|i| i.weight * i.quantity as f32)
```

Attention à la **priorité** : `as` lie plus fort que `*`, donc `i.weight * i.quantity as f32` se lit `i.weight * (i.quantity as f32)` — c'est bien ce qu'on veut. Mais dans le doute, parenthèse.

Et le turbofish `::<f32>` est nécessaire ici pour la raison de ex06/Q8 : `sum` est générique sur sa sortie.

Le résultat attendu, pour vérifier ta logique : 5.5×2 + 8.0×1 + 0.5×5 + 1.0×3 + 1.5×4 = 11 + 8 + 2.5 + 3 + 6 = **30.5 kg**.

</details>

---

### Q6. 🔥 Pourquoi l'énoncé recommande-t-il `.position()` plutôt que `.iter_mut().find()` dans `sell` ?

```rust
fn sell(&mut self, name: &str, qty: u32) -> Result<u32, ShopError> {
    let item = self.inventory.iter_mut()
        .find(|i| i.name == name)
        .ok_or_else(|| ShopError::ItemNotFound(name.into()))?;
    item.quantity -= qty;
    self.gold += item.value * qty;      // ← ici
    Ok(item.value * qty)
}
```

- [ ] **A.** `.position()` est plus rapide
- [ ] **B.** `item` détient un emprunt **mutable** de `self.inventory`, donc de `self` — `self.gold += ...` demande un second emprunt mutable de `self` → `E0499`
- [ ] **C.** `find` ne fonctionne pas sur `iter_mut`
- [ ] **D.** Pour éviter un clone

<details><summary>Réponse</summary>

**B.** `iter_mut()` emprunte `self.inventory` en mutable, et cet emprunt vit aussi longtemps que `item`. Comme `item` est utilisé **après** `self.gold += ...`, les deux emprunts mutables de `self` se chevauchent :

```
error[E0499]: cannot borrow `self.gold` as mutable more than once at a time
```

C'est la même famille que ex03/Q3 et ex06/Q3 — le borrow checker ne distingue pas les champs à travers un emprunt de méthode.

Les deux sorties possibles :

```rust
// (1) l'index — pas d'emprunt persistant
let idx = self.inventory.iter().position(|i| i.name == name).ok_or(...)?;
let gold = self.inventory[idx].value * qty;
self.inventory[idx].quantity -= qty;
self.gold += gold;

// (2) refermer l'emprunt avant de toucher self.gold
let gold = {
    let item = self.inventory.iter_mut().find(...).ok_or(...)?;
    item.quantity -= qty;
    item.value * qty
};              // ← l'emprunt meurt ici
self.gold += gold;
```

La leçon générale : **extraire une valeur `Copy` puis relâcher l'emprunt** débloque presque tous les `E0499`.

</details>

---

### Q7. 🔥 Quelle est la sortie ?

```rust
let mut count = 0;
let items = vec![1, 2, 3];
items.iter().for_each(|_| count += 1);
println!("{count}");

let names = vec![String::from("a")];
let f = move || println!("{}", names.len());
f();
println!("{}", names.len());     // ← ici
```

- [ ] **A.** `3` puis `1` puis `1`
- [ ] **B.** `3` puis `1`, puis erreur de compilation sur la dernière ligne
- [ ] **C.** `0` puis `1` puis `1`
- [ ] **D.** Erreur : `count` n'est pas capturable

<details><summary>Réponse</summary>

**B.**

- La première closure capture `count` en `&mut` (déduit `FnMut`) → `count` vaut bien `3`. Pas besoin de `move`
- La seconde a `move` : elle prend l'**ownership** de `names`. La dernière ligne est un `E0382: borrow of moved value: names`

`move` ne dit **pas** « appelable une fois » — il dit « capture par valeur plutôt que par référence ». Une closure `move` peut très bien être `Fn` et appelable dix fois (c'est le cas ici : `names.len()` ne consomme rien).

Où `move` est **obligatoire** : quand la closure doit survivre au scope courant — retournée par une fonction, stockée dans une struct, ou envoyée à un thread. C'est exactement le sujet de ex17.

</details>

---

### Q8. Que renvoie `.position()` et pourquoi pas un `usize` direct ?

```rust
let idx = shop.inventory.iter().position(|i| i.name == "Arrows");
```

- [ ] **A.** `usize`, avec `-1` si absent
- [ ] **B.** `Option<usize>` — il n'existe pas de « valeur index invalide » en Rust
- [ ] **C.** `Result<usize, Error>`
- [ ] **D.** `Option<&Item>`

<details><summary>Réponse</summary>

**B.** Pas de sentinelle `-1` : l'absence est encodée dans le **type**, pas dans une valeur magique qu'on peut oublier de tester.

Et grâce à la niche vue en ex05/Q6, `Option<usize>`… ne bénéficie **pas** de l'optimisation (tous les `usize` sont des valeurs valides), donc il fait 16 octets. En revanche `Option<&Item>` ou `Option<Box<T>>` sont gratuits.

Le chaînage idiomatique pour ton `sell` :

```rust
let idx = self.inventory.iter()
    .position(|i| i.name == name)
    .ok_or_else(|| ShopError::ItemNotFound(name.to_string()))?;
```

`ok_or_else` plutôt que `ok_or` : la `String` n'est allouée **que** si l'item est absent.

</details>

---

### Q9. 🔥 Ces trois versions sont-elles équivalentes ?

```rust
// (1)
let mut total = 0;
for i in &shop.inventory { total += i.value * i.quantity; }

// (2)
let total: u32 = shop.inventory.iter().map(|i| i.value * i.quantity).sum();

// (3)
let total = shop.inventory.iter().fold(0, |acc, i| acc + i.value * i.quantity);
```

- [ ] **A.** Oui, y compris en performance
- [ ] **B.** (2) et (3) sont plus lentes à cause des closures
- [ ] **C.** (1) est la plus rapide
- [ ] **D.** (3) ne compile pas

<details><summary>Réponse</summary>

**A.** Les trois compilent vers **le même code machine** en release. Les itérateurs Rust sont une abstraction à coût nul : monomorphisation + inlining + LLVM éliminent entièrement la couche.

Le choix est donc purement une question de **lisibilité** :
- (2) `map` + `sum` — le plus déclaratif, préfère-le
- (3) `fold` — quand l'accumulation n'est pas une simple somme
- (1) la boucle — quand il y a des effets de bord, des `break`, ou plusieurs accumulateurs

Petite subtilité sur (3), si tu as hésité : le `0` littéral n'est **pas** figé à `i32`. C'est une variable d'inférence entière, et la contrainte `acc + i.value * i.quantity` (un `u32`) la résout en `u32`. Vérifié au compilateur — `total` vaut bien un `u32`.

Le *fallback* `i32` de ex02/Q1 ne s'applique que **s'il ne reste aucune contrainte**. Ici il y en a une, donc elle gagne.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 9/9 | Phase 1 bouclée — tu es prêt pour les generics |
| 7-8/9 | Solide. Q2 (`&&T`) et Q6 (le borrow dans `sell`) sont les deux qui te coûteront du temps en vrai |
| < 7 | Relis Q1, Q2, Q6. Ces trois-là résument tous les messages d'erreur que tu vas rencontrer avec les itérateurs |
