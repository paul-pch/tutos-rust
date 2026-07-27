# QUIZ — ex14 : `Box<T>`, heap et types récursifs

> 9 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Pourquoi ce code est-il refusé ?

```rust
enum FsNode {
    File(String, u64),
    Folder(String, FsNode),
}
```

- [ ] **A.** Un enum ne peut pas se référencer lui-même
- [ ] **B.** `recursive type FsNode has infinite size` — le compilateur doit connaître la taille à la compilation, et elle se définirait ici récursivement
- [ ] **C.** Il manque `#[derive(Debug)]`
- [ ] **D.** `Folder` doit avoir un champ nommé

<details><summary>Réponse</summary>

**B.** Le compilateur calcule `size_of::<FsNode>()` = max des variantes. Or `Folder` contient un `FsNode`, dont la taille est… `size_of::<FsNode>()`. L'équation `x = 24 + x` n'a pas de solution finie.

`Box<FsNode>` casse la récursion : c'est un **pointeur**, de taille fixe (8 octets sur 64 bits), quelle que soit la taille de ce qu'il pointe. La récursion passe alors par le tas, pas par le type.

Le message est excellent : *`insert some indirection (e.g., a Box, Rc, or &) to break the cycle`*.

</details>

---

### Q2. 🔥 Ces deux définitions sont-elles toutes deux valides ?

```rust
Folder(String, Vec<Box<FsNode>>)   // (1) — ton code
Folder(String, Vec<FsNode>)        // (2)
```

- [ ] **A.** Seule (1) compile
- [ ] **B.** Les deux compilent, mais (2) est **meilleure** : `Vec` fournit déjà l'indirection
- [ ] **C.** Seule (2) compile
- [ ] **D.** Les deux compilent et sont strictement équivalentes

<details><summary>Réponse</summary>

**B.** C'est la question que le README pose en piste (« réfléchis à la différence ») — voici la réponse complète.

Un `Vec<T>` est **toujours** 24 octets (pointeur + capacité + longueur), quel que soit `T`. Ses éléments vivent sur le tas. L'indirection nécessaire à la récursion **existe déjà**.

Donc `Vec<Box<FsNode>>` ajoute une allocation **par fichier** et un déréférencement supplémentaire à chaque accès — pour rien.

| | Allocations pour 3 fichiers | Layout |
|---|---|---|
| `Vec<Box<FsNode>>` | 1 (le Vec) + 3 (les Box) | Vec → [ptr, ptr, ptr] → 3 nœuds dispersés |
| `Vec<FsNode>` | 1 | Vec → [nœud, nœud, nœud] contigus, *cache-friendly* |

`Box` est **obligatoire** uniquement dans le cas d'une récursion **directe** : `Folder(String, Box<FsNode>)`, ou `Option<Box<Node>>` pour une liste chaînée.

Garde `Box` puisque c'est le sujet de l'exercice, mais sache qu'en vrai code tu écrirais `Vec<FsNode>`.

</details>

---

### Q3. 🔥 Combien pèsent ces types ?

```rust
size_of::<Box<u64>>()        // (1)
size_of::<Box<[u8; 1024]>>() // (2)
size_of::<Box<dyn Display>>()// (3)
size_of::<Box<[u8]>>()       // (4)
```

- [ ] **A.** 8, 1024, 8, 8
- [ ] **B.** 8, 8, 16, 16
- [ ] **C.** 8, 8, 8, 8
- [ ] **D.** 8, 1032, 16, 16

<details><summary>Réponse</summary>

**B — 8, 8, 16, 16.** Mesuré au compilateur.

(1) et (2) : `Box<T>` d'un `T: Sized` est un **pointeur simple**, 8 octets. Peu importe que la donnée pointée fasse 8 octets ou 1 Ko — elle est sur le tas.

(3) et (4) : dès que `T` n'est **pas** `Sized`, le pointeur devient **gras** (*fat pointer*), 16 octets :
- `Box<dyn Trait>` = pointeur données + pointeur **vtable**
- `Box<[u8]>` = pointeur données + **longueur**

C'est le mécanisme exact qui rend le dispatch dynamique de ex09/Q5 possible, et c'est pourquoi `Vec<Box<dyn Trait>>` est la seule façon de stocker des types hétérogènes.

Le point (2) explique aussi le second usage de `Box` cité dans le README : déplacer une valeur volumineuse devient le déplacement de 8 octets, sans memcpy.

</details>

---

### Q4. 🔥 Ce code compile-t-il ?

```rust
FsNode::Folder(name, fs_nodes) => {
    for node in fs_nodes.iter() {
        write!(f, "  {}", node)?;      // node : &Box<FsNode>
    }
    Ok(())
}
```

- [ ] **A.** Non : `Box<FsNode>` n'implémente pas `Display`
- [ ] **B.** Non : il faut `write!(f, "{}", **node)`
- [ ] **C.** Oui — la stdlib fournit `impl<T: Display + ?Sized> Display for Box<T>`
- [ ] **D.** Non : `.iter()` ne marche pas sur `Vec<Box<T>>`

<details><summary>Réponse</summary>

**C.** Le `Box` ne pose **aucun** problème ici. Deux mécanismes se combinent :

1. La stdlib implémente `Display` (et `Debug`, `Hash`, `PartialEq`…) pour `Box<T>` dès que `T` les implémente
2. Les macros de format prennent leurs arguments **par référence**, donc un `&Box<FsNode>` convient

Même chose pour les **appels de méthode** grâce à l'auto-deref : `node.total_size()` sur un `&Box<FsNode>` fonctionne, le compilateur insère `(**node).total_size()`.

Si tu veux quand même un `&FsNode` explicite (utile quand l'auto-deref ne se déclenche pas — bornes génériques, comparaisons) :

| Expression | Type |
|---|---|
| `node` | `&Box<FsNode>` |
| `&**node` | `&FsNode` |
| `node.as_ref()` | `&FsNode` — plus lisible |

</details>

---

### Q5. 🔥 Pourquoi ce code ne compile-t-il **pas** ?

```rust
FsNode::Folder(name, fs_nodes) => {
    write!(f, "[dir] {}/", name);
    for node in fs_nodes.iter() {
        write!(f, "  {}", node)
    }
}
```

- [ ] **A.** Le `Box` empêche l'affichage
- [ ] **B.** Deux erreurs : le corps d'un `for` doit valoir `()`, et le bras de `match` doit valoir `fmt::Result`
- [ ] **C.** `write!` ne peut pas être appelé deux fois
- [ ] **D.** Il manque un `&` sur `name`

<details><summary>Réponse</summary>

**B.** Deux `E0308` en cascade, rien à voir avec `Box` :

```
error[E0308]: expected `()`, found `Result<(), Error>`        ← le write! dans le for
error[E0308]: expected `Result<(), Error>`, found `()`        ← le for comme dernière expression
```

Deux règles se percutent :
- une boucle `for` s'évalue **toujours** en `()` — son corps doit donc valoir `()`
- la dernière expression de `fmt` doit valoir `std::fmt::Result`

Le correctif :

```rust
write!(f, "[dir] {}/", name)?;   // le ? manquait aussi : sans lui, warning unused_must_use
for node in fs_nodes.iter() {
    write!(f, "  {}", node)?;    // ? consomme le Result → le corps vaut ()
}
Ok(())                            // valeur de retour du bras
```

Le `?` fait ici **deux** choses : propager l'erreur d'écriture, et « aplatir » le `Result` en `()` pour satisfaire le corps de boucle.

</details>

---

### Q6. Que vaut `depth()` sur un répertoire **vide** avec cette implémentation ?

```rust
FsNode::Folder(_, content) => {
    1 + content.iter().map(|f| f.depth()).max().unwrap()
}
```

- [ ] **A.** `0`
- [ ] **B.** `1`
- [ ] **C.** **Panique** — `max()` sur un itérateur vide rend `None`
- [ ] **D.** Erreur de compilation

<details><summary>Réponse</summary>

**C.** C'est la piste du README (« combien vaut `[].iter().map(...).max()` ? ») : `Iterator::max` rend `Option<T>` justement parce qu'il n'existe pas de maximum d'un ensemble vide. `.unwrap()` sur `None` → panic.

Et la spec exige `0` pour un répertoire vide, pas `1`.

Trois écritures correctes, de la plus verbeuse à la plus élégante :

```rust
// (1) — garde explicite, celle de ton code actuel
if content.is_empty() { 0 } else { 1 + content.iter().map(|f| f.depth()).max().unwrap() }

// (2) — map_or : gère les deux cas d'un coup
content.iter().map(|f| f.depth()).max().map_or(0, |d| d + 1)

// (3) — le neutre de max sur usize est 0
content.iter().map(|f| f.depth() + 1).max().unwrap_or(0)
```

(2) est la plus idiomatique. Note que (1) et (3) donnent le même résultat, mais (3) déplace le `+1` à l'intérieur du `map` — attention, ces deux formes ne sont **pas** interchangeables en général.

</details>

---

### Q7. 🔥 Quelle affirmation est vraie ?

```rust
let b = Box::new(String::from("hi"));
let s: String = *b;          // (1)

let r = std::rc::Rc::new(String::from("hi"));
let s2: String = *r;         // (2)
```

- [ ] **A.** Les deux compilent
- [ ] **B.** (1) compile, (2) échoue — `Box` est le seul pointeur intelligent dont on peut **extraire** la valeur par déréférencement
- [ ] **C.** (2) compile, (1) échoue
- [ ] **D.** Aucune ne compile

<details><summary>Réponse</summary>

**B.** Vérifié : (2) donne `error[E0507]: cannot move out of an Rc`.

`Box<T>` bénéficie d'un traitement **spécial du compilateur** (parfois appelé `DerefMove`, un trait qui n'existe pas publiquement) : `*b` peut *déplacer* la valeur hors de la boîte, qui est ensuite désallouée.

Pour tous les autres — `Rc`, `Arc`, `RefCell`, `MutexGuard` — `*x` ne donne qu'un accès en lecture (`Deref`) ou en écriture (`DerefMut`), jamais un move : ils ne peuvent pas garantir qu'ils sont l'unique détenteur.

L'équivalent pour `Rc`, quand tu **es** le dernier détenteur : `Rc::try_unwrap(r)` → `Result<T, Rc<T>>`. Tu le croiseras en ex15.

</details>

---

### Q8. L'énoncé demande une indentation de 2 espaces par niveau. Pourquoi `" ".repeat(self.depth())` ne marche-t-il **pas** ?

- [ ] **A.** `repeat` n'existe pas sur `&str`
- [ ] **B.** Parce que `depth()` mesure la hauteur **sous** le nœud, alors que l'indentation dépend de la profondeur **au-dessus** — que `Display::fmt` ne connaît pas
- [ ] **C.** Parce qu'il faut 2 espaces, pas 1
- [ ] **D.** Parce que `self.depth()` est trop lent

<details><summary>Réponse</summary>

**B**, et c'est le vrai défi de conception de l'exercice.

`Display::fmt(&self, f)` ne reçoit **que** le nœud. Il n'a aucun moyen de savoir s'il est la racine ou un petit-enfant. `depth()` est même exactement l'inverse de ce qu'il faut : la racine a la plus grande `depth` et zéro indentation.

Les deux solutions propres :

```rust
// (1) — méthode helper récursive, la piste du README
impl FsNode {
    fn fmt_indent(&self, f: &mut Formatter, level: usize) -> fmt::Result {
        write!(f, "{}", "  ".repeat(level))?;
        match self {
            FsNode::File(n, s) => writeln!(f, "[file] {n} ({s} bytes)"),
            FsNode::Folder(n, children) => {
                writeln!(f, "[dir] {n}")?;
                for c in children { c.fmt_indent(f, level + 1)?; }
                Ok(())
            }
        }
    }
}
impl Display for FsNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { self.fmt_indent(f, 0) }
}

// (2) — passer le niveau via la largeur du Formatter : f.width()
//       astucieux mais obscur, et incompatible avec un vrai usage de {:>10}
```

Prends (1). Attention aussi à `write!` vs `writeln!` : la spec attend un saut de ligne après chaque nœud.

</details>

---

### Q9. 🔥 Quand un `Box<FsNode>` contenant tout l'arbre est-il libéré, et comment ?

- [ ] **A.** Jamais, il faut appeler `drop` manuellement
- [ ] **B.** À la sortie de scope du propriétaire — la libération est **récursive**, `Drop` descend tout l'arbre
- [ ] **C.** Par un garbage collector
- [ ] **D.** À la fin du programme

<details><summary>Réponse</summary>

**B.** `Box<T>` implémente `Drop` : à la sortie de scope, il libère la valeur pointée. Et cette valeur (`FsNode::Folder`) contient un `Vec<Box<FsNode>>`, dont le `Drop` libère chaque `Box`, qui libère chaque nœud… La destruction **descend récursivement** tout l'arbre, dans un ordre déterministe, sans une ligne de code de ta part.

C'est le RAII de Rust : la libération est **prouvée à la compilation**, pas déléguée à un runtime.

Deux limites à connaître :
- sur un arbre très **profond** (des dizaines de milliers de niveaux), ce `Drop` récursif peut faire déborder la pile. Les vraies structures de données implémentent un `Drop` itératif
- `Box` ne peut pas exprimer de **cycle** (un enfant qui pointe vers son parent). C'est structurellement un arbre. Pour un graphe, il faut `Rc` — et là, les cycles fuient. C'est le sujet exact de ex15

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 9/9 | Excellent — Q2, Q3 et Q7 dépassent le cadre de l'exercice |
| 7-8/9 | Solide. Retiens Q2 (`Vec` indirecte déjà) et Q4 (`Box` est transparent pour `Display`) |
| < 7 | Reprends Q4 et Q5 : le vrai obstacle de cet exercice, ce n'est pas `Box`, c'est le `Result` de `fmt` |
