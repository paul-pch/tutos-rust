# Exercice 05 — Enums & Pattern Matching

## Concept

Une enum définit un type avec plusieurs variantes, dont certaines peuvent porter des données.

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

Les variantes peuvent contenir des champs nommés :

```rust
enum Shape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Point,
}
```

---

## Pattern matching

`match` permet de brancher selon la variante et d'en extraire les données :

```rust
match shape {
    Shape::Circle { radius } => println!("circle r={}", radius),
    Shape::Rectangle { width, height } => println!("rect {}x{}", width, height),
    Shape::Point => println!("point"),
}
```

`match` est exhaustif : le compilateur refuse si une variante n'est pas couverte.

---

## Exercice

Tu continues l'inventaire de jeu. On veut catégoriser les items.

1. Définis une enum `ItemKind` avec les variantes :
   - `Weapon { attack: u32 }`
   - `Armor { defense: u32 }`
   - `Consumable { heal: u32 }`
   - `Quest` (sans données)

2. Définis une struct `Item` avec les champs : `name` (`String`), `weight` (`f32`), `kind` (`ItemKind`)

3. Implémente `Item::new(name: &str, weight: f32, kind: ItemKind) -> Item`

4. Implémente une méthode `describe(&self)` qui affiche l'item. Utilise `match` sur `self.kind` pour afficher le label et le bonus correspondants.

5. Dans `main`, crée un item de chaque variante et appelle `describe` sur chacun.

**Output attendu :**
```
[Weapon] Sword | 5.5kg | attack bonus: 15
[Armor] Shield | 8.0kg | defense bonus: 10
[Consumable] Health Potion | 0.5kg | heals 50 hp
[Quest] Ancient Key | 0.1kg | quest item
```

---

## Pistes

- Dérive `Debug` sur l'enum si tu veux l'inspecter avec `{:?}`
- Dans le `match`, pour extraire un champ nommé : `ItemKind::Weapon { attack } => ...`
- Si tu n'as besoin que du label sans extraire les données : `ItemKind::Weapon { .. } => ...`
- Un seul `match` peut retourner un tuple `(label, bonus)` pour éviter d'en écrire deux

---

## Lancer l'exercice

```bash
cargo run -p ex05-enums
```
