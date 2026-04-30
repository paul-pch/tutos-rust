# Exercice 09 — Traits

## Concept

Un trait définit un **comportement partagé** entre plusieurs types. C'est l'équivalent des interfaces dans d'autres langages.

```rust
trait Describable {
    fn describe(&self) -> String;
}
```

N'importe quel type peut implémenter ce trait :

```rust
impl Describable for Item {
    fn describe(&self) -> String {
        format!("{} | {}kg | {} gold", self.name, self.weight, self.value)
    }
}

impl Describable for Player {
    fn describe(&self) -> String {
        format!("Player: {} | HP: {}", self.name, self.health)
    }
}
```

---

## Méthodes par défaut

Un trait peut fournir une implémentation par défaut, que les types peuvent surcharger ou non :

```rust
trait Describable {
    fn describe(&self) -> String;

    fn print(&self) {
        println!("{}", self.describe()); // utilise describe() du type
    }
}
```

---

## Trait comme paramètre de fonction

Pour accepter n'importe quel type qui implémente un trait :

```rust
fn print_all(items: &[impl Describable]) {
    for item in items {
        item.print();
    }
}
```

Syntaxe alternative avec `where` (utile quand c'est complexe) :

```rust
fn print_all<T>(items: &[T]) where T: Describable {
    for item in items {
        item.print();
    }
}
```

---

## Exercice

Tu vas définir un trait commun pour les types de ton inventaire.

1. Définis un trait `Summarize` avec :
   - une méthode requise `summary(&self) -> String`
   - une méthode par défaut `print_summary(&self)` qui affiche le résultat de `summary`
2. Implémente `Summarize` pour `Item` et pour une nouvelle struct `Player` (nom + points de vie)
3. Écris une fonction `print_all` qui accepte une slice de n'importe quel type implémentant `Summarize` et appelle `print_summary` sur chacun
4. Dans le `main`, crée une liste d'items et un player, appelle `print_all` sur chacun

**Output attendu :**
```
[Item] Sword — 5.5kg, 120 gold
[Item] Potion — 0.5kg, 30 gold
[Player] Alice — 100 HP
```

---

## Pistes

- `&[impl Summarize]` est une slice de n'importe quel type implémentant `Summarize`
- Tu ne peux pas mélanger `Item` et `Player` dans la même slice — ce sont des types différents. `print_all` sera appelée deux fois, une par type
- `impl Trait` dans un paramètre est du sucre syntaxique pour un générique — le compilateur résout le type à la compilation

---

## Lancer l'exercice

```bash
cargo run -p ex09-traits
```
