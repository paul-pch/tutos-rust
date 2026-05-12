# Exercice 06 — Collections : Vec et HashMap

## Concept

Après avoir modélisé un `Item` avec une struct (ex04) et catégorisé les items avec des enums (ex05), on va maintenant stocker plusieurs items dans des collections.

Rust propose deux collections essentielles :

### `Vec<T>` — Vecteur

Un tableau redimensionnable qui stocke des éléments de même type dans un ordre séquentiel.

```rust
let mut nombres: Vec<i32> = Vec::new();  // Vecteur vide
let mut nombres = vec![1, 2, 3];          // Vecteur avec valeurs

nombres.push(4);                          // Ajoute un élément
let premier = nombres[0];                 // Accès par index
let longueur = nombres.len();             // Nombre d'élélements
```

### `HashMap<K, V>` — Table de hachage

Un dictionnaire qui associe des clés à des valeurs.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Épée", 10);
scores.insert("Bouclier", 5);

let valeur = scores.get("Épée");          // Option<&V>
scores.remove("Bouclier");
```

### Pattern matching avec enums

On combine pattern matching et collections pour traiter différents types d'items :

```rust
for item in inventaire.iter() {
    match item.rarity {
        Rarity::Common => println!("Commune"),
        Rarity::Rare => println!("Rare!"),
        Rarity::Legendary => println!("LÉGENDAIRE!"),
    }
}
```

---

## Exercice

**Objectif** : Créer un système d'inventaire qui stocke des items dans un `Vec` et les organise par catégorie avec une `HashMap`.

### Étape 1 — Définir les structures

Dans `src/main.rs` :

1. Importe `std::collections::HashMap`
2. Recrée la struct `Item` de l'exercice 04 avec les champs :
   - `name: String`
   - `rarity: Rarity` (enum à définir)
   - `value: u32`
3. Recrée l'enum `Rarity` de l'exercice 05 avec les variants :
   - `Common`
   - `Rare`
   - `Legendary`
4. Ajoute une méthode `display(&self)` à `Item` qui retourne une chaîne formatée :
   - `"name (rarity) - value gold"`
   - Exemple : `"Épée de fer (Common) - 10 gold"`

### Étape 2 — Créer un inventaire avec Vec

Dans la fonction `main` :

1. Crée un `Vec<Item>` nommé `inventaire` avec au moins 4 items de différentes raretés
2. Parcourt le vecteur et affiche chaque item en appelant `display()`

### Étape 3 — Organiser avec HashMap

Ajoute une nouvelle organisation :

1. Crée une `HashMap<String, Vec<Item>>` appelée `par_categorie`
2. Les clés seront : `"Armes"`, `"Armures"`, `"Potions"`
3. Les valeurs sont des `Vec<Item>` contenant les items de chaque catégorie
4. Remplis la map avec tes items
5. Parcourt la HashMap et affiche chaque catégorie avec ses items

### Étape 4 — Calculer la valeur totale

1. Calcule la valeur totale de tout l'inventaire en sommant les `value` de tous les items
2. Affiche `"Valeur totale de l'inventaire : X gold"`

---

## Output attendu

```
=== Inventaire ===
Épée de fer (Common) - 10 gold
Potion de soin (Common) - 5 gold
Bouclier d'acier (Rare) - 50 gold
Amulette dragon (Legendary) - 500 gold

=== Par catégorie ===
Armes :
  Épée de fer (Common) - 10 gold
Armures :
  Bouclier d'acier (Rare) - 50 gold
Potions :
  Potion de soin (Common) - 5 gold
  Amulette dragon (Legendary) - 500 gold

Valeur totale de l'inventaire : 565 gold
```

---

## Pistes

- Utilise `vec![]` pour créer le vecteur d'items
- Pour la HashMap, utilise `insert(key, vec![item1, item2])`
- Pour parcourir une HashMap : `for (categorie, items) in par_categorie.iter()`
- N'oublie pas `mut` sur les collections modifiables
- L'ordre d'affichage des catégories dans la HashMap n'est pas garanti — c'est normal

---

## Lancer l'exercice

```bash
cargo run -p ex06-collections
```

---

## Tests

Les tests vérifient :
- La struct `Item` existe avec les bons champs
- L'enum `Rarity` a les 3 variants
- La méthode `display()` retourne le bon format
- L'inventaire contient au moins 4 items
- La HashMap organise correctement les items
- Le calcul de la valeur totale est correct
