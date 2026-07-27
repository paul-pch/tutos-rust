# QUIZ — ex07 : Option, Result et `?`

> 8 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Ce code compile-t-il ?

```rust
fn main() {
    let content = std::fs::read_to_string("items.csv")?;
    println!("{content}");
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `?` ne peut être utilisé que dans une fonction qui retourne `Result` ou `Option`
- [ ] **C.** Non : `read_to_string` ne retourne pas de `Result`
- [ ] **D.** Oui, `main` gère le `?` automatiquement

<details><summary>Réponse</summary>

**B.** `?` est du sucre pour « si `Err`, `return` l'erreur immédiatement ». Il faut donc que la fonction ait un type de retour compatible. Ici `main` rend `()`.

Mais — et c'est utile à savoir — `main` **peut** rendre un `Result` :

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("items.csv")?;
    println!("{content}");
    Ok(())
}
```

En cas d'`Err`, le processus affiche l'erreur via `Debug` sur `stderr` et sort avec le code `1`. Le `Ok(())` final est obligatoire.

</details>

---

### Q2. 🔥 Ce code compile-t-il ?

```rust
fn load(path: &str) -> Result<Vec<Item>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    let n: f32 = content.trim().parse()?;
    Ok(vec![])
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : le `?` sur `parse()` produit un `ParseFloatError`, pas un `io::Error`
- [ ] **C.** Non : `parse` ne retourne pas de `Result`
- [ ] **D.** Oui, `?` convertit automatiquement

<details><summary>Réponse</summary>

**B.** `?` fait bien une conversion automatique — mais via le trait `From` :

```rust
// ce que ? fait vraiment
match expr {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
}
```

Il faut donc qu'un `impl From<ParseFloatError> for io::Error` existe. Ce n'est pas le cas → `E0277: ? couldn't convert the error to std::io::Error`.

C'est **exactement** le problème que ex08 résout : soit tu élargis le type de retour à `Box<dyn Error>` (qui accepte tout ce qui implémente `Error`), soit tu définis ton propre enum d'erreur avec les `impl From` qui vont bien.

C'est aussi pourquoi l'énoncé de ex07 te dit d'utiliser `.unwrap()` sur le parse « pour l'instant » : c'est une dette technique assumée, pas une bonne pratique.

</details>

---

### Q3. 🔥 Ce code compile-t-il ?

```rust
fn first_name(items: &[Item]) -> Result<String, std::io::Error> {
    let first = items.first()?;
    Ok(first.name.clone())
}
```

- [ ] **A.** Oui
- [ ] **B.** Non : `the ? operator can only be used on Results, not Options, in a function that returns Result`
- [ ] **C.** Non : `first()` ne retourne pas d'`Option`
- [ ] **D.** Oui, `None` devient `Err`

<details><summary>Réponse</summary>

**B.** `?` ne traverse pas la frontière `Option` / `Result` : pas de conversion implicite de `None` vers une erreur — Rust ne saurait pas *laquelle*.

Les ponts explicites, à connaître :

```rust
opt.ok_or(MonErreur::Vide)?          // Option<T> → Result<T, E>
opt.ok_or_else(|| coûteux())?        // idem, erreur construite paresseusement
res.ok()                             // Result<T, E> → Option<T>, jette l'erreur
```

`ok_or_else` est préférable dès que construire l'erreur alloue (typiquement `format!(...)`).

</details>

---

### Q4. Que valent ces expressions ?

```rust
"a\nb\n".lines().count()    // (1)
"a\nb".lines().count()      // (2)
"a\n\n".lines().count()     // (3)
"".lines().count()          // (4)
```

- [ ] **A.** 3, 2, 3, 1
- [ ] **B.** 2, 2, 2, 0
- [ ] **C.** 2, 2, 3, 1
- [ ] **D.** 3, 2, 2, 0

<details><summary>Réponse</summary>

**B — 2, 2, 2, 0.**

`.lines()` traite le `\n` comme un **terminateur**, pas un séparateur : un `\n` final ne crée pas de ligne vide supplémentaire. Elle gère aussi `\r\n` (le `\r` est retiré).

Pourquoi ça compte pour ton parser CSV : tu peux boucler sur `.lines()` sans filtrer la dernière ligne vide — c'est déjà fait. En revanche, (3) montre qu'une ligne **réellement** vide au milieu du fichier est bien renvoyée, et fera échouer ton `split(',')` avec un `InvalidFormat`. D'où l'idiome défensif :

```rust
for line in content.lines().filter(|l| !l.trim().is_empty()) { ... }
```

Piège voisin : `"a,b,".split(',')` rend **3** éléments (`"a"`, `"b"`, `""`), car là le `,` est un vrai séparateur.

</details>

---

### Q5. 🔥 Quelle est la sortie ?

```rust
let v: Vec<i32> = vec![];
let a = v.first().copied().unwrap_or(0);
let b = v.first().copied().unwrap_or_else(|| { println!("calcul"); 0 });
let c = v.iter().sum::<i32>();
println!("{a} {b} {c}");
```

- [ ] **A.** `calcul` puis `0 0 0`
- [ ] **B.** `0 0 0` sans `calcul`
- [ ] **C.** Panique sur `first()`
- [ ] **D.** `calcul` puis `0 0` puis panique sur `sum`

<details><summary>Réponse</summary>

**A.** Le `Vec` est vide donc `first()` rend `None`, et `unwrap_or_else` **exécute** sa closure → `calcul` s'affiche. `sum()` sur un itérateur vide rend l'élément neutre, `0` — jamais de panique.

La famille complète, par ordre de préférence :

| Méthode | Comportement sur `None`/`Err` |
|---|---|
| `unwrap_or(v)` | rend `v` — **v est évalué systématiquement** |
| `unwrap_or_else(\|\| v)` | rend `v` — évalué **seulement** si besoin |
| `unwrap_or_default()` | rend `Default::default()` |
| `expect("msg")` | **panique** avec ton message |
| `unwrap()` | **panique** avec un message générique |

`expect` bat toujours `unwrap` : quand ça pète en prod à 3 h du matin, tu veux savoir *pourquoi*, pas juste `called Option::unwrap() on a None value`.

</details>

---

### Q6. 🔥 Combien de warnings ?

```rust
fn save(path: &str) -> Result<(), std::io::Error> { Ok(()) }

fn main() {
    save("out.txt");
}
```

- [ ] **A.** 0
- [ ] **B.** 1 — `unused Result that must be used`
- [ ] **C.** 1 — variable inutilisée
- [ ] **D.** C'est une erreur, pas un warning

<details><summary>Réponse</summary>

**B.** `Result` est annoté `#[must_use]` : ignorer un `Result` déclenche *`unused Result that must be used — this Result may be an Err variant, which should be handled`*.

C'est un **warning**, donc ça compile — d'où l'importance de ne pas les ignorer. Pour dire explicitement « je m'en fiche » : `let _ = save("out.txt");`.

Même mécanisme sur les itérateurs (`.map()` sans consommateur, cf. ex11) et sur `Option`.

Astuce projet : `#![deny(unused_must_use)]` en tête de `main.rs`, ou dans `[lints]` du `Cargo.toml`, transforme ce warning en erreur.

</details>

---

### Q7. Dans ton parser, quelle version est correcte ?

```rust
// (A)
let value: u32 = parts[2].parse().unwrap();

// (B)
let value: u32 = parts.get(2).unwrap().parse().unwrap();

// (C)
let value: u32 = parts[2].trim().parse().unwrap();
```

Le fichier contient `Sword, 5.5, 120` (avec des espaces après les virgules).

- [ ] **A.** (A) et (B) marchent
- [ ] **B.** Seule (C) marche
- [ ] **C.** Les trois marchent
- [ ] **D.** Aucune

<details><summary>Réponse</summary>

**B.** `" 120".parse::<u32>()` échoue : `ParseIntError { kind: InvalidDigit }`. **`parse` ne tolère aucun espace**, ni avant, ni après.

(A) et (B) paniquent donc au premier `.unwrap()`. Et (B) ne protège de rien de plus que (A) — elle remplace juste un panic d'indexation par un panic d'`unwrap`.

Le réflexe : **`.trim()` systématiquement** avant tout `parse` sur de la donnée externe. Idem pour `"5.5 ".parse::<f32>()`.

</details>

---

### Q8. 🔥 Quelle est la différence sémantique entre ces deux signatures ?

```rust
fn find_item(&self, name: &str) -> Option<&Item>;    // (1)
fn find_item(&self, name: &str) -> Result<&Item, ShopError>;  // (2)
```

- [ ] **A.** Aucune, `Option` est un `Result` sans erreur
- [ ] **B.** (1) : l'absence est **normale** ; (2) : l'absence est un **échec** qu'il faut expliquer
- [ ] **C.** (2) est toujours meilleure
- [ ] **D.** (1) est plus rapide

<details><summary>Réponse</summary>

**B.** C'est un choix de **conception**, pas de technique.

- `Option<T>` : « il se peut qu'il n'y en ait pas, et c'est prévu ». `HashMap::get`, `Iterator::next`, `Vec::first`
- `Result<T, E>` : « ça aurait dû marcher, voici pourquoi ça a échoué ». Lecture de fichier, parsing, transaction

Le test qui tranche : **l'appelant a-t-il besoin de savoir *pourquoi* ?** Si oui → `Result`. S'il n'y a qu'une seule raison possible d'échouer, `Option` suffit et évite un type d'erreur inutile.

Tu retrouveras ce choix en ex11 : `most_valuable()` rend `Option<&Item>` (une boutique vide n'est pas une erreur), alors que `sell()` rend `Result` (l'appelant doit distinguer « stock insuffisant » de « item inexistant »).

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | Prêt pour ex08 — tu as déjà compris pourquoi il existe |
| 6-7/8 | Bien. Q2 est le point clé : `?` convertit via `From`, pas par magie |
| < 6 | Relis la section `?` du README et reproduis Q2 dans ton code : le message d'erreur est très pédagogique |
