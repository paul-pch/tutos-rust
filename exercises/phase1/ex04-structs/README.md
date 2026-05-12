# Exercice 04 — Structs

## Concept

Une struct permet de définir un type composite avec des champs nommés.

```rust
struct Player {
    name: String,
    health: u32,
}
```

---

## Instanciation

```rust
let p = Player {
    name: String::from("Alice"),
    health: 100,
};

println!("{}", p.name);
```

Pour modifier un champ, toute la struct doit être `mut` :

```rust
let mut p = Player { ... };
p.health = 80;
```

---

## Méthodes

Les méthodes sont définies dans un bloc `impl`. `&self` est une référence immutable vers l'instance, `&mut self` pour la modifier.

```rust
impl Player {
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn take_damage(&mut self, damage: u32) {
        self.health -= damage;
    }
}
```

---

## Méthodes associées

Sans `self`, c'est l'équivalent d'une méthode statique. Convention : `new` pour les constructeurs.

```rust
impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            health: 100,
        }
    }
}

let p = Player::new("Alice");
```

---

## Debug printing

Pour afficher une struct avec `println!`, il faut dériver le trait `Debug` :

```rust
#[derive(Debug)]
struct Player { ... }

println!("{:?}", p);  // compact
println!("{:#?}", p); // pretty print
```

---

## Exercice

Tu modélises un item d'inventaire pour un jeu.

1. Crée une struct `Item` avec les champs : `name` (`String`), `weight` (`f32`), `value` (`u32`)
2. Implémente `Item::new(name: &str, weight: f32, value: u32) -> Item`
3. Implémente une méthode `describe(&self)` qui affiche les infos de l'item
4. Implémente une méthode `is_valuable(&self) -> bool` qui retourne `true` si `value > 50`
5. Dans le `main`, crée 2 items et affiche leurs descriptions et si ils sont précieux

**Output attendu :**
```
Item: Sword | Weight: 5.5kg | Value: 120 gold
  -> Valuable: true
Item: Rope | Weight: 1.0kg | Value: 10 gold
  -> Valuable: false
```

---

## Pistes

- Pour afficher un `f32` avec une décimale : `{:.1}` dans le format string
- `describe` peut utiliser `println!` directement, pas besoin de retourner une String

---

## Lancer l'exercice

```bash
cargo run -p ex04-structs
```
