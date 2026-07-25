# ex14-box — `Box<T>`, heap et types récursifs

## Concept

`Box<T>` est un pointeur intelligent qui alloue `T` sur le **tas** (heap) au lieu de la pile (stack).

```rust
// Allocation sur le tas
let b: Box<i32> = Box::new(42);
println!("{}", *b); // déréférencement explicite, ou implicite via Deref
```

**Pourquoi Box ?**

- **Taille inconnue à la compilation** : le compilateur refuse un type dont la taille dépend de lui-même. `Box<T>` a une taille fixe (un pointeur), ce qui casse la récursion.
- **Déplacement de données volumineuses** : évite de copier de gros blocs sur la pile.
- **Trait objects** : `Box<dyn Trait>` — déjà vu en ex09.

**Types récursifs sans Box — l'erreur classique :**

```rust
enum Tree<T> {
    Leaf(T),
    Node(T, Tree<T>, Tree<T>), // ❌ taille infinie
}

enum Tree<T> {
    Leaf(T),
    Node(T, Box<Tree<T>>, Box<Tree<T>>), // ✅
}
```

**Aparté — les trois formes de variantes d'enum**

Dans `Leaf(T)` ci-dessus, `(T)` est une variante **tuple** : les données sont anonymes, accédées par position. Rust propose en réalité trois formes, toutes mélangeables dans un même enum :

```rust
enum Shape {
    // 1. Variante unitaire : aucune donnée
    Empty,

    // 2. Variante tuple : champs anonymes, accès par position (.0, .1)
    Circle(f64),              // .0 = rayon
    Rectangle(f64, f64),      // .0 = largeur, .1 = hauteur

    // 3. Variante struct : champs nommés, comme une struct
    Triangle { base: f64, height: f64 },
}
```

C'est **la même chose** conceptuellement, seule la façon de nommer/lire les données change. On les construit et on les filtre différemment :

```rust
let a = Shape::Circle(2.0);
let b = Shape::Triangle { base: 3.0, height: 4.0 };

match a {
    Shape::Empty => {}
    Shape::Circle(r) => println!("rayon {r}"),          // tuple : liaison par position
    Shape::Rectangle(w, h) => println!("{w} x {h}"),
    Shape::Triangle { base, height } => {               // struct : liaison par nom
        println!("{base} / {height}");
    }
}
```

**Quand choisir quoi ?**

- **Tuple** (`Circle(f64)`) : concis, idéal quand il y a 1–2 champs dont le rôle est évident.
- **Struct** (`Triangle { base, height }`) : plus lisible dès qu'il y a plusieurs champs ou que leur rôle n'est pas évident à la position seule — pas de risque d'inverser `base` et `height`.

Pour cet exercice, les deux formes sont valides pour modéliser `FsNode`. Choisis celle qui rend ton code le plus clair : `File(String, u64)` ou `File { name: String, size: u64 }` — à toi de trancher.

**Déréférencement :**

```rust
let s = Box::new(String::from("hello"));
println!("{}", s.len());  // Deref implicite
println!("{}", (*s).len()); // Deref explicite — équivalent
```

`Box<T>` implémente `Deref<Target = T>` : tu peux l'utiliser comme une référence `&T` dans la plupart des contextes.

**Ownership :** `Box<T>` est owné. Quand il sort de scope, la valeur sur le tas est libérée (via `Drop`).

---

## Exercice

Tu vas modéliser un système de fichiers simplifié sous forme d'arbre récursif.

Un **système de fichiers** est un arbre : chaque nœud est soit un **fichier** (une feuille : un nom + une taille), soit un **répertoire** (un nom + zéro ou plusieurs nœuds enfants, qui peuvent eux-mêmes être des fichiers ou des répertoires). C'est cette imbrication « un répertoire contient des nœuds du même type que lui » qui rend le type **récursif** — et donc pourquoi `Box` entre en jeu (voir la section *Concept*).

1. Crée un enum `FsNode` représentant soit un **fichier** (avec nom et taille en octets), soit un **répertoire** (avec nom et une liste de nœuds enfants). Utilise `Box` ou `Vec<Box<FsNode>>` pour gérer la récursivité.

2. Implémente une méthode `total_size(&self) -> u64` qui calcule récursivement la taille totale d'un nœud (fichier → sa taille, répertoire → somme des enfants).

3. Implémente une méthode `depth(&self) -> usize` qui retourne la profondeur maximale de l'arbre (un fichier ou répertoire vide = 0, un répertoire avec enfants = 1 + max des profondeurs enfants).

4. Implémente `Display` pour `FsNode` : affiche l'arbre avec une indentation de 2 espaces par niveau. Les répertoires sont préfixés par `[dir]`, les fichiers par `[file]`.

5. Dans `main`, construis un arbre représentant cette structure :
```
/
├── src/
│   ├── main.rs     (1_024 octets)
│   └── lib.rs      (2_048 octets)
├── README.md       (512 octets)
└── target/
    └── debug/
        └── app     (1_048_576 octets)
```

6. Affiche l'arbre, sa taille totale et sa profondeur.

---

## Spécification détaillée des attendus

### `total_size(&self) -> u64`

- Sur un **fichier** : renvoie sa taille en octets.
- Sur un **répertoire** : renvoie la **somme** des `total_size` de tous ses enfants (récursif). Un répertoire n'a pas de taille propre.

### `depth(&self) -> usize`

La profondeur mesure le nombre de niveaux d'imbrication **sous** le nœud courant.

| Nœud | Profondeur |
|------|-----------|
| Un fichier | `0` |
| Un répertoire **vide** | `0` |
| Un répertoire avec enfants | `1 + max(depth des enfants)` |

### `Display` — règles de formatage exactes

Chaque ligne suit ce gabarit :

```
<indentation><préfixe> <nom>[ (<taille> bytes)]
```

- **Indentation** : `2 espaces × niveau`. La racine est au niveau `0` (aucune indentation), ses enfants directs au niveau `1` (2 espaces), etc.
- **Préfixe** : `[dir]` pour un répertoire, `[file]` pour un fichier.
- **Taille** : affichée **uniquement pour les fichiers**, sous la forme ` (1024 bytes)` (nombre brut, sans séparateur de milliers, mot `bytes` en anglais). Les répertoires n'affichent **pas** de taille.
- Un répertoire est affiché, puis ses enfants juste en dessous, indentés d'un niveau de plus.

Après l'arbre, `main` affiche deux lignes supplémentaires :

```
Total size: <total_size de la racine> bytes
Depth: <depth de la racine>
```

> ⚠️ Le test compare la sortie **caractère par caractère** (`tests/output.rs`). Le moindre écart d'espace, de casse ou de retour à la ligne fait échouer l'exercice.

---

## Exemple travaillé (pour comprendre la sémantique)

Prends cette petite arborescence, **différente** de celle demandée dans `main` :

```
docs/
├── intro.txt   (100 octets)
└── img/
    └── logo.png (900 octets)
```

Décomposition des calculs attendus :

- `total_size(logo.png)` = `900`
- `total_size(img)` = `total_size(logo.png)` = `900`
- `total_size(intro.txt)` = `100`
- `total_size(docs)` = `100 + 900` = **`1000`**

- `depth(logo.png)` = `0` (fichier)
- `depth(img)` = `1 + max(0)` = `1`
- `depth(intro.txt)` = `0`
- `depth(docs)` = `1 + max(depth(intro.txt)=0, depth(img)=1)` = **`2`**

Et son rendu `Display` :

```
[dir] docs
  [file] intro.txt (100 bytes)
  [dir] img
    [file] logo.png (900 bytes)
```

Vérifie que ta logique produit bien ces valeurs sur ce petit cas avant de t'attaquer à l'arbre complet.

---

## Output attendu

```
[dir] /
  [dir] src
    [file] main.rs (1024 bytes)
    [file] lib.rs (2048 bytes)
  [file] README.md (512 bytes)
  [dir] target
    [dir] debug
      [file] app (1048576 bytes)
Total size: 1052160 bytes
Depth: 3
```

---

## Pistes

- Si le compilateur se plaint que `FsNode` a une taille infinie, relis la section sur les types récursifs.
- `Vec<Box<FsNode>>` et `Vec<FsNode>` sont tous les deux valides pour stocker les enfants — réfléchis à la différence.
- Pour `Display` avec indentation, une méthode helper `fn fmt_indent(&self, f: &mut Formatter, depth: usize)` peut simplifier la récursion.
- `depth` sur un répertoire vide : combien vaut `[].iter().map(...).max()` ?

---

## Lancer l'exercice

```bash
cargo run -p ex14-box
```
