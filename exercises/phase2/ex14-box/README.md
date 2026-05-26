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
