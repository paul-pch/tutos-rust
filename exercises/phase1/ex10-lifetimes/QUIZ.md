# QUIZ — ex10 : Lifetimes

> 8 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Que signifie `'a` dans cette signature ?

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
```

- [ ] **A.** `a` et `b` doivent vivre exactement aussi longtemps
- [ ] **B.** Le retour est valide tant que **les deux** entrées le sont — `'a` est en pratique la **plus courte** des deux durées
- [ ] **C.** Le retour vit aussi longtemps que le programme
- [ ] **D.** `'a` force `a` et `b` à vivre plus longtemps

<details><summary>Réponse</summary>

**B.** Les lifetimes sont **contravariantes** sur les entrées : le compilateur unifie `'a` avec la plus petite région où les deux références sont simultanément valides.

Le contrat que tu signes : « je promets que le retour pointe dans `a` **ou** dans `b`, donc l'appelant ne doit pas l'utiliser au-delà de la mort du plus court des deux ».

Ce qui découle du README et mérite d'être répété : **une lifetime ne prolonge rien**. C'est une annotation *descriptive*, purement à la compilation, effacée du binaire. Elle permet au compilateur de vérifier une cohérence que tu affirmes ; elle ne change aucun comportement d'exécution.

</details>

---

### Q2. 🔥 Pourquoi celle-ci compile-t-elle **sans** annotation ?

```rust
fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]
}
```

- [ ] **A.** Parce qu'il n'y a qu'un seul paramètre référence
- [ ] **B.** Parce que `&str` est toujours `'static`
- [ ] **C.** Parce que le retour est une slice
- [ ] **D.** Par chance

<details><summary>Réponse</summary>

**A** — c'est la 2ᵉ **règle d'élision**. Il y en a trois, appliquées dans l'ordre :

1. Chaque paramètre référence **élidé** reçoit sa propre lifetime
2. S'il y a **exactement un** paramètre d'entrée, sa lifetime est attribuée à **toutes** les sorties
3. S'il y a `&self` ou `&mut self`, **sa** lifetime est attribuée à toutes les sorties (elle prime sur la règle 2)

`first_word` est donc réécrite en `fn first_word<'a>(s: &'a str) -> &'a str`.

`longest(a, b)` a **deux** entrées et pas de `self` : aucune règle ne s'applique, le compilateur ne devine pas → `E0106: missing lifetime specifier`.

La règle 3 explique pourquoi tu n'as **jamais** eu à annoter une méthode `&self` jusqu'ici : `fn name(&self) -> &str` marche toujours.

</details>

---

### Q3. Ce code compile-t-il ?

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("Greatsword");
    let result;
    {
        let s2 = String::from("Bow");
        result = longest(&s1, &s2);
    }
    println!("{result}");
}
```

- [ ] **A.** Oui, affiche `Greatsword`
- [ ] **B.** Non : `s2 does not live long enough`
- [ ] **C.** Oui, mais avec un warning
- [ ] **D.** Panique à l'exécution

<details><summary>Réponse</summary>

**B** — `E0597`.

Le compilateur ne raisonne **pas sur les valeurs**. Il sait seulement que le retour est marqué `'a`, et que `'a` doit être contenue dans la vie de `s1` **et** de `s2`. Comme `s2` meurt à l'accolade, `'a` s'arrête là — et `result` est utilisé après.

Peu importe qu'on sache, nous, que `"Greatsword"` est plus long et que le retour pointe forcément dans `s1`. **L'analyse est purement statique, basée sur la signature.** Si tu voulais exprimer « le retour vient toujours de `a` », il faudrait deux lifetimes distinctes :

```rust
fn first_of<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str { a }
```

Et là le code de Q3 compilerait.

</details>

---

### Q4. 🔥 Quelle est l'erreur ?

```rust
fn make_name() -> &str {
    let s = String::from("Sword");
    &s
}
```

- [ ] **A.** `missing lifetime specifier` — et **aucune** annotation ne peut sauver ce code
- [ ] **B.** Il suffit d'écrire `fn make_name<'a>() -> &'a str`
- [ ] **C.** Il suffit d'écrire `-> &'static str`
- [ ] **D.** Ça compile

<details><summary>Réponse</summary>

**A.** Le compilateur commence par `E0106: missing lifetime specifier — this function's return type contains a borrowed value, but there are no values for it to be borrowed from`.

Et si tu « corriges » avec `<'a>`, tu tombes sur `E0515: cannot return reference to local variable s`. Idem avec `'static`. C'est logique : `s` est libérée à la fin de la fonction, aucune annotation ne peut faire mentir la réalité.

**Les lifetimes ne créent pas de durée de vie, elles la décrivent.** Un code qui renvoie un pointeur pendouillant est faux, point.

La vraie solution est de **rendre la propriété** : `fn make_name() -> String { String::from("Sword") }`.

</details>

---

### Q5. 🔥 Lesquelles de ces valeurs sont `&'static str` ?

```rust
let a = "Sword";                          // (1)
let b = String::from("Sword");
let c = b.as_str();                       // (2)
let d: &'static str = Box::leak(b.into_boxed_str()); // (3)
const E: &str = "Sword";                  // (4)
```

- [ ] **A.** (1) et (4)
- [ ] **B.** (1), (3) et (4)
- [ ] **C.** Toutes
- [ ] **D.** (1) seulement

<details><summary>Réponse</summary>

**B — (1), (3) et (4).**

- (1) et (4) : les **littéraux de chaîne** sont embarqués dans le binaire, dans le segment read-only. Ils existent avant `main` et après — d'où `'static`
- (2) : `c` emprunte le buffer heap de `b`. Il meurt avec `b` → durée de vie locale
- (3) : `Box::leak` **fuit** volontairement l'allocation (elle ne sera jamais libérée) et rend une référence `'static`. C'est légal et parfois utile (config chargée au démarrage), mais c'est une fuite mémoire assumée

Piège de vocabulaire fréquent : `T: 'static` (borne) ≠ `&'static T` (référence). `T: 'static` signifie « `T` ne contient **aucune** référence de durée limitée » — une `String` owned satisfait `T: 'static` ! C'est cette distinction qui explique la contrainte de `thread::spawn` en ex17.

</details>

---

### Q6. Ce code compile-t-il ?

```rust
struct InventoryRef<'a> {
    category: &'a str,
    items: Vec<Item>,
}

fn main() {
    let inv;
    let category = String::from("Weapons");
    inv = InventoryRef { category: &category, items: vec![] };
    drop(category);
    inv.describe();
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `cannot move out of category because it is borrowed`
- [ ] **C.** Non : `InventoryRef` ne peut pas mélanger un champ emprunté et un champ owned
- [ ] **D.** Oui, `drop` ne fait rien sur une `String` empruntée

<details><summary>Réponse</summary>

**B** — `E0505`. C'est l'étape 4 de l'énoncé, et elle est très instructive.

`drop(category)` prend `category` **par valeur** — c'est un move. Or `inv` détient un emprunt de `category` encore vivant (utilisé à la ligne suivante). Le compilateur refuse.

C'est **toute la valeur** de l'annotation `<'a>` sur la struct : elle propage la contrainte « `InventoryRef` ne peut pas survivre à ce qu'il emprunte » jusqu'aux sites d'usage.

Et la réponse à (C) : mélanger un champ emprunté et un champ owned est parfaitement normal et courant. La struct est simplement liée à la durée de vie la plus courte de ses emprunts.

</details>

---

### Q7. 🔥 Ce code compile-t-il ?

```rust
struct Parser<'a> { input: &'a str }

impl<'a> Parser<'a> {
    fn rest(&self) -> &str { self.input }
}
```

Et si on veut que le retour vive aussi longtemps que `input` plutôt que `&self` ?

- [ ] **A.** Ça compile, et le retour a déjà la lifetime de `input`
- [ ] **B.** Ça compile, mais le retour est lié à `&self` (règle 3) ; il faut écrire `-> &'a str` pour le lier à `input`
- [ ] **C.** Ça ne compile pas
- [ ] **D.** Il faut `fn rest<'b>(&'b self) -> &'b str`

<details><summary>Réponse</summary>

**B.** Par la règle d'élision n°3, `fn rest(&self) -> &str` est lu comme `fn rest<'s>(&'s self) -> &'s str`. Le retour est artificiellement limité à la vie de l'emprunt de `self`, alors que la donnée vit en réalité aussi longtemps que `'a`.

Concrètement :

```rust
let s = String::from("hello world");
let r;
{
    let p = Parser { input: &s };
    r = p.rest();       // ❌ avec -> &str      : p meurt ici
}                       // ✅ avec -> &'a str  : r pointe dans s, pas dans p
println!("{r}");
```

L'élision est un raccourci pratique, mais elle fait parfois un choix **plus restrictif** que nécessaire. Sur une struct qui emprunte, écrire `-> &'a T` explicitement est souvent le bon geste.

</details>

---

### Q8. Pourquoi la plupart de tes exercices n'ont-ils eu **aucune** annotation de lifetime jusqu'ici ?

- [ ] **A.** Les lifetimes sont optionnelles
- [ ] **B.** Parce que tu manipulais des types **owned** (`String`, `Vec`) — les lifetimes n'apparaissent qu'avec des références stockées ou retournées
- [ ] **C.** Parce que le compilateur les infère toujours
- [ ] **D.** Parce que la phase 1 les désactive

<details><summary>Réponse</summary>

**B.** Les lifetimes n'existent **que** pour les références. Une struct qui possède ses données (`name: String`) n'en a jamais besoin.

D'où la stratégie recommandée en pratique, et notamment pour ex11 et ex16 :

> **Commence par du owned (`String`, `Vec<T>`). Ne passe aux références que si le profilage montre que les clones coûtent.**

C'est aussi pourquoi `Rc<T>` (ex15) est si utile : il donne du partage **sans** annotation de lifetime, en déplaçant la vérification à l'exécution via un compteur.

Et quand une struct auto-référentielle te résiste vraiment (un champ qui pointe vers un autre champ), la réponse n'est jamais « plus de lifetimes » — c'est de repenser le modèle, ou d'utiliser des index plutôt que des pointeurs.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | Les lifetimes ne te feront plus peur — Q7 est un piège que peu de gens voient |
| 6-7/8 | Bien. Mémorise les 3 règles d'élision de Q2, elles expliquent 90 % des cas |
| < 6 | Reprends Q1 et Q4 : tant que « les lifetimes décrivent, elles ne prolongent pas » n'est pas ancré, le reste reste flou |
