# Exercice 07 — Error Handling

## Concept

Rust n'a pas d'exceptions. Les erreurs sont des **valeurs** retournées par les fonctions, via deux types de la stdlib :

**`Option<T>`** — une valeur qui peut être absente :
```rust
enum Option<T> {
    Some(T), // il y a une valeur
    None,    // pas de valeur
}
```

**`Result<T, E>`** — une opération qui peut échouer :
```rust
enum Result<T, E> {
    Ok(T),  // succès, contient la valeur
    Err(E), // échec, contient l'erreur
}
```

---

## Gérer un Result

Avec `match` :
```rust
match File::open("file.txt") {
    Ok(file) => println!("Ouvert"),
    Err(e)   => println!("Erreur : {}", e),
}
```

Avec l'opérateur `?` — propage l'erreur au caller si `Err`, déroule la valeur si `Ok` :
```rust
fn read_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}
```

Le `?` ne fonctionne que dans une fonction qui retourne un `Result` (ou `Option`).

---

## Gérer un Option

```rust
let inventory: HashMap<String, Item> = ...;

match inventory.get("Sword") {
    Some(item) => println!("{}", item.name),
    None       => println!("Item introuvable"),
}
```

Ou avec `if let` quand seul le cas `Some` t'intéresse :
```rust
if let Some(item) = inventory.get("Sword") {
    println!("{}", item.name);
}
```

---

## Exercice

Tu étends l'inventaire de jeu vidéo pour charger des items depuis un fichier CSV.

Format du fichier `items.csv` :
```
Sword,5.5,120
Rope,1.0,10
Potion,0.5,30
```

1. Crée le fichier `items.csv` à la racine de l'exercice
2. Écris une fonction `load_items(path: &str) -> Result<Vec<Item>, std::io::Error>` qui lit le fichier ligne par ligne et parse chaque ligne en `Item`
3. Chaque ligne a 3 champs séparés par une virgule : `name`, `weight` (`f32`), `value` (`u32`)
4. Utilise `?` pour propager les erreurs de lecture
5. Dans le `main`, appelle `load_items` et gère le `Result` avec un `match`
6. Affiche chaque item chargé

**Output attendu :**
```
Loaded 3 items:
- Sword | 5.5kg | 120 gold
- Rope | 1.0kg | 10 gold
- Potion | 0.5kg | 30 gold
```

---

## Pistes

- Pour lire un fichier ligne par ligne : `std::fs::read_to_string` puis `.lines()`
- Pour parser une `&str` en `f32` : `"5.5".parse::<f32>()` retourne un `Result`
- `.parse()` retourne un `Result<T, ParseFloatError>` — pas compatible directement avec `std::io::Error`. Pour simplifier, utilise `.unwrap()` sur le parse pour l'instant
- Pour splitter une ligne : `.split(',')`

---

## Lancer l'exercice

```bash
cargo run -p ex07-error-handling
```
