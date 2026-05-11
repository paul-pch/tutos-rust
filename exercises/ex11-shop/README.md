# ex11-shop — Système de boutique : exercice d'intégration

## Concept

### Iterateurs & closures

Plutôt que des boucles `for` impératives, Rust expose une API d'itérateurs qui se chaîne :

```rust
// map + sum — transformer puis agréger
let total: u32 = inventory.iter()
    .map(|item| item.value * item.quantity)
    .sum();

// filter + count — compter sous condition
let count = inventory.iter()
    .filter(|item| item.category == ItemCategory::Weapon)
    .count();

// max_by_key — trouver le maximum selon un critère
let best = inventory.iter()
    .max_by_key(|item| item.value); // renvoie Option<&Item>

// position — trouver l'index d'un élément
let idx = inventory.iter()
    .position(|item| item.name == "Sword"); // renvoie Option<usize>
```

Les closures (`|param| expr`) capturent leur environnement. Le compilateur infère si elles implémentent `Fn`, `FnMut` ou `FnOnce` selon leur usage.

> Cet exercice est un exercice d'intégration : iterateurs & closures sont le fil directeur, mais tu vas mobiliser structs, enums, traits, collections et gestion d'erreurs en même temps.

---

## Exercice

Construis un système de boutique RPG de bas en haut. Pas de code fourni — tu écris tout dans `src/main.rs`.

### Étape 1 — `ItemCategory`

Définis un enum `ItemCategory` avec quatre variants : `Weapon`, `Armor`, `Consumable`, `Misc`.

Dérive `PartialEq` et implémente `Display` pour afficher `Weapon`, `Armor`, `Consumable`, `Misc`.

### Étape 2 — `Item`

Définis une struct `Item` avec les champs :
- `name: String`
- `category: ItemCategory`
- `weight: f32` — kg par unité
- `value: u32` — gold par unité
- `quantity: u32`

Implémente `Item::new(name, category, weight, value, quantity) -> Item`.

Implémente `Display` pour `Item` au format :
```
[Weapon] Sword | 5.5 kg | 120 gold | x2
```

### Étape 3 — `ShopError`

Définis un enum `ShopError` avec deux variants :
- `ItemNotFound(String)`
- `NotEnoughStock { item: String, have: u32, want: u32 }`

Implémente `Display` pour `ShopError` :
- `item not found: <name>`
- `not enough stock for <item> (have <have>, want <want>)`

### Étape 4 — `Shop`

Définis une struct `Shop` avec les champs :
- `name: String`
- `inventory: Vec<Item>`
- `gold: u32`

Implémente `Shop::new(name: &str, gold: u32) -> Shop` et `add_item(&mut self, item: Item)`.

### Étape 5 — Statistiques avec itérateurs

Ajoute trois méthodes à `Shop` :

- `total_weight(&self) -> f32` — somme de `weight * quantity` sur tous les items
- `total_value(&self) -> u32` — somme de `value * quantity` sur tous les items
- `most_valuable(&self) -> Option<&Item>` — item avec la valeur unitaire la plus haute

Utilise `.iter().map(|i| ...).sum()` pour les sommes, `.iter().max_by_key(|i| i.value)` pour le maximum.

### Étape 6 — Filtrage

Ajoute :

- `count_by_category(&self, cat: &ItemCategory) -> usize` — nombre de types d'items dans la catégorie

Utilise `.iter().filter(|i| ...).count()`.

### Étape 7 — Transaction avec gestion d'erreur

Ajoute :

- `sell(&mut self, name: &str, qty: u32) -> Result<u32, ShopError>`

La méthode doit :
1. Chercher l'index de l'item par nom — `Err(ItemNotFound)` si absent
2. Vérifier que le stock est suffisant — `Err(NotEnoughStock)` sinon
3. Déduire la quantité, ajouter l'or à `self.gold`, renvoyer `Ok(or_gagné)`

### Étape 8 — `main`

1. Crée la boutique `"Merchant Guild Shop"` avec 500 gold
2. Ajoute 5 items dans cet ordre :

   | Nom           | Catégorie   | Poids | Valeur | Qté |
   |---------------|-------------|-------|--------|-----|
   | Sword         | Weapon      | 5.5   | 120    | 2   |
   | Iron Shield   | Armor       | 8.0   | 85     | 1   |
   | Health Potion | Consumable  | 0.5   | 30     | 5   |
   | Rope          | Misc        | 1.0   | 5      | 3   |
   | Dagger        | Weapon      | 1.5   | 45     | 4   |

3. Affiche le nom de la boutique encadré (`=== ... ===`), puis la liste des items
4. Affiche total weight, total value, gold de la boutique
5. Affiche l'item le plus cher et le nombre de types d'armes
6. Effectue ces 4 transactions et affiche le résultat de chacune :
   - Vendre 2x `"Health Potion"`
   - Vendre 3x `"Rope"`
   - Vendre 10x `"Dagger"` ← stock insuffisant
   - Vendre 1x `"Arrows"` ← introuvable
7. Affiche le gold final

---

## Output attendu

```
=== Merchant Guild Shop ===

[Weapon] Sword | 5.5 kg | 120 gold | x2
[Armor] Iron Shield | 8.0 kg | 85 gold | x1
[Consumable] Health Potion | 0.5 kg | 30 gold | x5
[Misc] Rope | 1.0 kg | 5 gold | x3
[Weapon] Dagger | 1.5 kg | 45 gold | x4

Total weight: 30.5 kg
Total value: 670 gold
Shop gold: 500 gold

Most valuable: Sword (120 gold)
Weapons: 2 types

--- Transactions ---
Sold 2x Health Potion for 60 gold
Sold 3x Rope for 15 gold
Error: not enough stock for Dagger (have 4, want 10)
Error: item not found: Arrows

Shop gold: 575 gold
```

---

## Pistes

- `#[derive(PartialEq)]` sur `ItemCategory` active l'opérateur `==` entre variants.
- `{:.1}` formate un `f32` avec exactement une décimale.
- `.map(|i| i.weight * i.quantity as f32).sum::<f32>()` — le turbofish sur `sum` est parfois nécessaire quand le type n'est pas inféré.
- Dans `sell`, cherche l'index avec `.position()` puis accède à `self.inventory[idx]` — cela évite les conflits de borrow que `.iter_mut().find()` génère.
- Pour les transactions, retiens le nom et la quantité avant l'appel pour les afficher dans le `Ok` :
  ```
  match shop.sell(name, qty) {
      Ok(gold) => println!("Sold {}x {} for {} gold", qty, name, gold),
      Err(e)   => println!("Error: {}", e),
  }
  ```
- La somme du gold final : 500 + 60 (Health Potion) + 15 (Rope) = 575. Les deux erreurs ne modifient pas le gold.

---

## Lancer l'exercice

```sh
cargo run -p ex11-shop
cargo test -p ex11-shop
```
