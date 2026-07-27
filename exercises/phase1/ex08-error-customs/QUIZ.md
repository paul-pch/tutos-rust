# QUIZ — ex08 : Erreurs custom

> 8 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Ce code compile-t-il ?

```rust
enum ParseError { InvalidFormat(String) }

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { ParseError::InvalidFormat(s) => write!(f, "Format invalide : {s}") }
    }
}

impl std::error::Error for ParseError {}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `ParseError doesn't implement Debug`
- [ ] **C.** Non : `impl Error` ne peut pas avoir un corps vide
- [ ] **D.** Non : il faut implémenter `source()`

<details><summary>Réponse</summary>

**B.** La déclaration réelle du trait est :

```rust
pub trait Error: Debug + Display { ... }
```

`Debug` et `Display` sont des **supertraits** : impossible d'implémenter `Error` sans eux. Il manque `#[derive(Debug)]` sur l'enum.

Pourquoi les deux ? `Display` pour l'utilisateur (« Format invalide : BadLine »), `Debug` pour le développeur — c'est notamment ce que `main() -> Result<...>` affiche quand le programme sort en erreur, et ce que `unwrap()` imprime dans son panic.

Le corps vide de `impl Error` est en revanche parfaitement correct : les deux méthodes du trait (`source()` et `description()`) ont des implémentations par défaut.

</details>

---

### Q2. 🔥 Pourquoi ce `?` fonctionne-t-il ?

```rust
fn load(path: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;   // io::Error
    let n: f32 = "abc".parse()?;                    // ParseFloatError
    Err(ParseError::InvalidFormat("x".into()))?;    // ParseError
    Ok(vec![])
}
```

Trois types d'erreur différents, un seul type de retour.

- [ ] **A.** `Box<dyn Error>` est un type spécial que le compilateur traite à part
- [ ] **B.** Grâce à `impl<E: Error + 'a> From<E> for Box<dyn Error + 'a>` dans la stdlib
- [ ] **C.** Grâce à un cast implicite
- [ ] **D.** Ça ne compile pas

<details><summary>Réponse</summary>

**B.** Rappel de ex07/Q2 : `?` fait `return Err(From::from(e))`. La stdlib fournit une implémentation **générique** :

```rust
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a>
```

N'importe quel type implémentant `Error` se convertit donc gratuitement en `Box<dyn Error>`. C'est cette ligne, et elle seule, qui rend le pattern utilisable.

Bonus utile : `Box<dyn Error>` implémente aussi `From<String>` et `From<&str>`, donc `Err("oups")?` compile dans une fonction qui rend `Box<dyn Error>`. Pratique pour prototyper.

</details>

---

### Q3. 🔥 Quel est le **coût** de `Box<dyn Error>` par rapport à un enum d'erreur ?

- [ ] **A.** Aucun
- [ ] **B.** L'appelant ne peut plus faire de `match` sur les variantes d'erreur
- [ ] **C.** C'est plus lent à l'exécution
- [ ] **D.** Ça ne marche pas avec `?`

<details><summary>Réponse</summary>

**B**, et c'est le point le plus important de cet exercice.

Avec un enum, l'appelant peut réagir finement :

```rust
match load("items.csv") {
    Err(ParseError::InvalidNumber(s)) => eprintln!("chiffre cassé : {s}, on saute"),
    Err(ParseError::InvalidFormat(_)) => return Err(...),
    Ok(items) => ...,
}
```

Avec `Box<dyn Error>`, tu n'as plus qu'une chose à en faire : l'afficher. Le type concret est effacé (`dyn`). On peut le récupérer avec `err.downcast_ref::<ParseError>()`, mais c'est du rattrapage laid.

La règle de l'écosystème :
- **binaire / application** → `Box<dyn Error>` ou `anyhow::Error`. L'erreur remonte au `main`, on l'affiche, terminé
- **bibliothèque** → **enum concret**. Tes utilisateurs doivent pouvoir décider quoi faire. C'est ce que fait `thiserror`

</details>

---

### Q4. Ce code compile-t-il ?

```rust
let value: u32 = parts[2]
    .trim()
    .parse()
    .map_err(|e| ParseError::InvalidNumber(e.to_string()))?;
```

dans une fonction qui retourne `Result<Vec<Item>, Box<dyn Error>>`.

- [ ] **A.** Non : `map_err` ne s'applique qu'aux `Option`
- [ ] **B.** Oui — `map_err` transforme l'erreur, puis `?` la boxe
- [ ] **C.** Non : il faudrait `map` et non `map_err`
- [ ] **D.** Non : `e.to_string()` n'existe pas sur une erreur

<details><summary>Réponse</summary>

**B.** Deux conversions s'enchaînent :
1. `map_err` : `Result<u32, ParseIntError>` → `Result<u32, ParseError>` (ton code)
2. `?` : `ParseError` → `Box<dyn Error>` (via le `From` de Q2)

`.to_string()` marche sur toute erreur grâce à l'impl générique `impl<T: Display> ToString for T` — et `Error: Display`.

Le tableau à retenir :

| Méthode | Agit sur |
|---|---|
| `.map(f)` | la valeur `Ok` |
| `.map_err(f)` | l'erreur `Err` |
| `.and_then(f)` | la valeur `Ok`, avec `f` qui rend elle-même un `Result` |

</details>

---

### Q5. 🔥 Que se passe-t-il ?

```rust
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
```

- [ ] **A.** Compile et fonctionne
- [ ] **B.** Erreur de compilation : récursion détectée
- [ ] **C.** Compile, puis **stack overflow** à l'exécution
- [ ] **D.** Affiche une chaîne vide

<details><summary>Réponse</summary>

**C.** `to_string()` vient de l'impl générique `impl<T: Display> ToString for T`, qui appelle… `Display::fmt`. Récursion infinie → `thread 'main' has overflowed its stack`.

Le compilateur ne le voit pas : rien de syntaxiquement circulaire, juste deux traits qui se renvoient la balle. Il n'émet même pas de warning.

Le même piège existe avec `Debug` : `write!(f, "{:?}", self)` dans un `impl Debug` manuel. Dans un `impl Display`, on ne formate **que les champs**, jamais `self`.

</details>

---

### Q6. 🔥 Que produit ce programme, et pourquoi la sortie surprend-elle ?

```rust
#[derive(Debug)]
enum ParseError { InvalidFormat(String) }
// + impl Display { "Format invalide : {s}" } + impl Error

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Err(ParseError::InvalidFormat("BadLine".into()))?;
    Ok(())
}
```

- [ ] **A.** `Format invalide : BadLine`
- [ ] **B.** `Error: InvalidFormat("BadLine")`
- [ ] **C.** Rien, code de sortie 0
- [ ] **D.** Panique

<details><summary>Réponse</summary>

**B.** Quand `main` rend un `Err`, le runtime l'affiche via **`Debug`**, pas `Display` — d'où le `InvalidFormat("BadLine")` brut au lieu de ton joli message. Code de sortie : `1`.

C'est un choix délibéré de la stdlib : `Debug` est plus informatif pour un diagnostic (il montre la variante et la structure).

Si tu veux ton message `Display` — et l'énoncé de ex08 l'attend explicitement — il faut gérer l'erreur toi-même :

```rust
fn main() {
    match load_items("items.csv") {
        Ok(items) => { /* ... */ }
        Err(e) => eprintln!("Erreur de parsing : {e}"),  // {} → Display ✅
    }
}
```

Note aussi `eprintln!` plutôt que `println!` : les erreurs vont sur `stderr`.

</details>

---

### Q7. À quoi sert `source()` que tu n'as pas implémentée ?

- [ ] **A.** À rien, c'est déprécié
- [ ] **B.** À exposer l'erreur **sous-jacente**, pour reconstituer une chaîne de causes
- [ ] **C.** À donner le fichier source où l'erreur est née
- [ ] **D.** À sérialiser l'erreur

<details><summary>Réponse</summary>

**B.** Signature : `fn source(&self) -> Option<&(dyn Error + 'static)>`. Par défaut elle rend `None` — ton erreur est une racine.

Elle prend son sens quand tu **enveloppes** une erreur :

```rust
enum ParseError {
    InvalidNumber { field: String, source: std::num::ParseIntError },
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::InvalidNumber { source, .. } => Some(source),
        }
    }
}
```

Un outil de logging peut alors dérouler toute la chaîne : *`échec du chargement` → `champ 'value' invalide` → `invalid digit found in string`*. C'est ce que fait `anyhow` avec `{:#}` et ce que `thiserror` génère via `#[source]`.

</details>

---

### Q8. 🔥 Ces deux enums d'erreur sont-ils équivalents ?

```rust
enum ShopError { NotEnoughStock(String, u32, u32) }              // (1)
enum ShopError { NotEnoughStock { item: String, have: u32, want: u32 } }  // (2)
```

- [ ] **A.** Oui, seule la syntaxe change
- [ ] **B.** Techniquement oui, mais (2) élimine une classe entière de bugs
- [ ] **C.** (1) est plus performante
- [ ] **D.** (2) ne peut pas être matchée

<details><summary>Réponse</summary>

**B.** Même taille mémoire, même performance. Mais avec (1) :

```rust
ShopError::NotEnoughStock(name, want, have)   // 😱 have et want inversés
```

Le compilateur accepte : deux `u32`, il ne peut rien distinguer. Ton message d'erreur affichera « have 10, want 4 » et tu chercheras le bug ailleurs.

Avec (2), l'ordre des champs nommés est libre et l'inversion est **impossible**. C'est exactement pour ça que ex11 et ex16 imposent la forme struct pour les variantes à plusieurs champs.

La règle générale (déjà énoncée dans le README de ex14) : **variante tuple pour 1 champ dont le rôle est évident, variante struct dès 2 champs**.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | Tu sais concevoir des erreurs, pas seulement les propager |
| 6-7/8 | Bien. Retiens Q3 (le vrai coût de `Box<dyn Error>`) et Q6 (`Debug` vs `Display` dans `main`) |
| < 6 | Reprends Q2 : tout l'exercice découle de cette impl `From` générique |
