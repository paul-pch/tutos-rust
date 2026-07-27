# QUIZ — ex05 : Enums & Pattern Matching

> 8 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Ce code compile-t-il ?

```rust
enum ItemKind { Weapon { attack: u32 }, Armor { defense: u32 }, Quest }

fn label(k: &ItemKind) -> &str {
    match k {
        ItemKind::Weapon { .. } => "Weapon",
        ItemKind::Armor { .. } => "Armor",
    }
}
```

- [ ] **A.** Oui, `Quest` est simplement ignorée
- [ ] **B.** Non : `non-exhaustive patterns: ItemKind::Quest not covered`
- [ ] **C.** Oui, mais avec un warning
- [ ] **D.** Non : il manque un `_ => ...`

<details><summary>Réponse</summary>

**B.** `match` est **exhaustif** : le compilateur exige que chaque valeur possible soit couverte. C'est une **erreur**, pas un warning.

C'est la fonctionnalité la plus sous-estimée de Rust : ajoute une variante `Consumable` à l'enum six mois plus tard, et le compilateur te liste **tous** les `match` à mettre à jour. Un refactoring qui serait un bug silencieux ailleurs devient une erreur de compilation.

Corollaire : le `_ => {}` fourre-tout est un **anti-pattern** quand tu matches sur tes propres enums — il désactive précisément ce filet.

</details>

---

### Q2. 🔥 Ces deux méthodes compilent-elles ?

```rust
enum Kind { Weapon { attack: u32 }, Tag(String) }
struct Item { kind: Kind }

impl Item {
    fn a(&self) {
        match self.kind {
            Kind::Weapon { attack } => println!("{attack}"),
            _ => {}
        }
    }
    fn b(&self) {
        match self.kind {
            Kind::Tag(s) => println!("{s}"),
            _ => {}
        }
    }
}
```

- [ ] **A.** Les deux compilent
- [ ] **B.** Aucune ne compile
- [ ] **C.** `a` compile, `b` échoue
- [ ] **D.** `b` compile, `a` échoue

<details><summary>Réponse</summary>

**C.** Vérifié au compilateur :

```
error[E0507]: cannot move out of `self.kind` as enum variant `Tag`
              which is behind a shared reference
help: consider borrowing here: match &self.kind
```

Le scrutin `self.kind` a le type `Kind` (pas `&Kind`), donc les liaisons se font **par valeur** :
- `attack: u32` est `Copy` → **copie** → `a` passe ✅
- `s: String` n'est pas `Copy` → **move** hors d'un `&self` → `b` échoue ❌

**Le réflexe à prendre : `match &self.kind`.** Le scrutin devient `&Kind`, les *match ergonomics* passent en mode « liaison par référence », et `s` devient un `&String` — plus aucun move. Ça marche pour les deux cas, donc autant l'écrire systématiquement.

Ce piège frappe dès que tu mets une `String` dans une variante — donc dans presque tous les exercices suivants.

</details>

---

### Q3. Quelle est la différence entre `_` et `..` ?

```rust
ItemKind::Weapon { attack: _ }   // (1)
ItemKind::Weapon { .. }          // (2)
```

- [ ] **A.** Aucune
- [ ] **B.** (1) ignore le champ `attack` nommément ; (2) ignore **tous** les champs restants
- [ ] **C.** (1) est invalide
- [ ] **D.** (2) ne marche que sur les variantes tuple

<details><summary>Réponse</summary>

**B.** Sur une variante à un seul champ, les deux sont équivalents. La différence se voit dès qu'il y en a plusieurs :

```rust
Shape::Rect { width, .. }           // je veux width, le reste peu importe
Shape::Rect { width, height: _ }    // je dois lister chaque champ
```

`..` est **résilient** : ajoute un champ à la variante, le pattern continue de compiler. Version tuple : `Rectangle(w, ..)` ou `Rectangle(.., h)`.

Nuance sur `_` seul : `let _ = value;` **libère immédiatement** la valeur, alors que `let _x = value;` la garde vivante jusqu'à la fin du scope. Ça compte dès qu'il y a un `Drop` — tu le retrouveras avec les `Ref` de ex15.

</details>

---

### Q4. 🔥 Quelle est la sortie ?

```rust
let n = 7;
match n {
    x if x < 0 => println!("négatif"),
    0 => println!("zéro"),
    x @ 1..=5 => println!("petit: {x}"),
    x if x % 2 == 0 => println!("pair: {x}"),
    _ => println!("autre: {n}"),
}
```

- [ ] **A.** `petit: 7`
- [ ] **B.** `pair: 7`
- [ ] **C.** `autre: 7`
- [ ] **D.** Erreur : les guards ne sont pas exhaustifs

<details><summary>Réponse</summary>

**C.** Les bras sont testés **dans l'ordre**, du haut vers le bas, premier match gagne.
`7 < 0` non → `7 == 0` non → `1..=5` non → `7 % 2 == 0` non → `_` ✅.

Deux syntaxes à connaître :
- `x @ 1..=5` — l'opérateur **binding** : teste le pattern **et** capture la valeur dans `x`
- `x if cond` — un **guard**. Attention : le compilateur ne raisonne pas sur les guards pour l'exhaustivité. Un `match` composé uniquement de bras gardés exigera toujours un `_` final, même si tu sais que les cas sont couverts

</details>

---

### Q5. Quand préférer `if let` à `match` ?

```rust
if let ItemKind::Weapon { attack } = &item.kind {
    println!("{attack}");
}
```

- [ ] **A.** Jamais, `match` est toujours plus idiomatique
- [ ] **B.** Quand une seule variante t'intéresse et que tu n'as rien à faire des autres
- [ ] **C.** `if let` est plus rapide à l'exécution
- [ ] **D.** Quand l'enum a plus de 3 variantes

<details><summary>Réponse</summary>

**B.** `if let` est du sucre pour un `match` à deux bras dont l'un est `_ => {}`. Même code machine, moins de bruit.

Ce que ça t'apprend sur `match` : il **n'exige pas d'être exhaustif sur les variantes**, il exige d'être exhaustif sur les **valeurs**. Un `_ => {}` remplit ce contrat — d'où le fait qu'`if let` soit toujours légal.

Le corollaire est celui de Q1 : `if let` (comme `_`) te fait perdre l'alerte du compilateur quand l'enum évolue. Sur ton propre enum métier, `match` explicite est plus sûr. Sur `Option`/`Result`, `if let` est parfait.

Depuis Rust 1.65 il existe aussi `let ... else` pour l'inverse (extraire ou sortir) :
```rust
let ItemKind::Weapon { attack } = &item.kind else { return };
```

</details>

---

### Q6. 🔥 Combien vaut `std::mem::size_of::<Kind>()` ?

```rust
enum Kind {
    Weapon { attack: u32 },   // 4 octets
    Tag(String),              // 24 octets
    Quest,                    // 0 octet
}
```

- [ ] **A.** 28 — la plus grande variante + 4 pour le discriminant
- [ ] **B.** 32 — 24 + 8 alignés
- [ ] **C.** 24
- [ ] **D.** 28 arrondi à 32

<details><summary>Réponse</summary>

**C — 24.** Mesuré : `size_of::<Kind>() == 24`.

Un enum occupe la taille de sa **plus grande variante** + un discriminant… **sauf** si le compilateur trouve une **niche**, c'est-à-dire une combinaison de bits impossible dans les données existantes.

Ici, `String` contient un pointeur qui ne peut **jamais** être nul. Rust utilise cette valeur interdite pour encoder les autres variantes : zéro octet de surcoût.

L'exemple canonique, à retenir absolument :

```rust
size_of::<Box<u64>>()          // 8
size_of::<Option<Box<u64>>>()  // 8  ← identique !
```

`Option<Box<T>>`, `Option<&T>`, `Option<Rc<T>>` sont **gratuits** : `None` est encodé par le pointeur nul. C'est pourquoi un `Option<Box<Node>>` (la liste chaînée classique) ne coûte rien de plus qu'un pointeur C — avec la sûreté en prime. Utile à savoir pour ex14.

</details>

---

### Q7. L'énoncé suggère « un seul `match` peut retourner un tuple `(label, bonus)` ». Ce code compile-t-il ?

```rust
let (label, bonus) = match &self.kind {
    ItemKind::Weapon { attack } => ("Weapon", *attack),
    ItemKind::Armor { defense } => ("Armor", *defense),
    ItemKind::Consumable { heal } => ("Consumable", *heal),
    ItemKind::Quest => ("Quest", 0),
};
```

- [ ] **A.** Non : les bras retournent des types différents
- [ ] **B.** Oui
- [ ] **C.** Non : `*attack` est invalide
- [ ] **D.** Non : `match` ne peut pas être une expression

<details><summary>Réponse</summary>

**B.** `match` est une **expression** : tous les bras doivent produire le **même** type, ici `(&str, u32)`.

Le `*attack` est nécessaire : avec `match &self.kind`, les *match ergonomics* lient `attack` en `&u32`. Le déréférencement produit un `u32` (`Copy`, donc gratuit) pour s'accorder avec le `0` littéral du dernier bras.

Si tu oublies les `*`, l'erreur est claire : `expected &u32, found integer` sur le bras `Quest`.

</details>

---

### Q8. 🔥 Ce code compile-t-il ?

```rust
enum Direction { North, South }

fn go(d: Direction) {
    match d {
        Direction::North | Direction::South => println!("vertical"),
    }
    println!("{:?}", d);
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `d` a été déplacé par le `match`
- [ ] **C.** Non : `Direction` n'implémente pas `Debug`
- [ ] **D.** Non : les or-patterns `|` sont invalides dans un `match`

<details><summary>Réponse</summary>

**C**, et seulement C.

- Les **or-patterns** (`A | B`) sont parfaitement valides, et ils préservent l'exhaustivité
- Le `match d` ne déplace **rien** : aucune liaison n'est créée dans les patterns, donc rien n'est extrait de `d`. Matcher une valeur ne la consomme que si un binding en extrait un champ non-`Copy` (cf. Q2)
- Il manque seulement `#[derive(Debug)]` sur `Direction`

Bonus : ajoute `#[derive(Clone, Copy)]` sur un enum sans données (*fieldless*) et il devient `Copy` — plus aucune question de move. C'est courant et gratuit pour les enums de type `Status`, `Priority`, `Rarity`.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | Enums + patterns solides. Q2 et Q6 t'éviteront des heures plus tard |
| 6-7/8 | Bien. Retiens surtout le réflexe `match &self.champ` |
| < 6 | Reprends Q2 en la tapant dans ton `main.rs` : c'est le piège que tu vas rencontrer le plus souvent |
