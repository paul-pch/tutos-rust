# QUIZ — ex12 : Generics

> 8 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. Combien de fonctions `highest` existent dans le binaire final ?

```rust
fn highest<T: PartialOrd>(a: T, b: T) -> T { if a >= b { a } else { b } }

fn main() {
    println!("{}", highest(3_u32, 7_u32));
    println!("{}", highest(1.5_f64, 0.2_f64));
    println!("{}", highest("a", "b"));
}
```

- [ ] **A.** 1 — les generics sont résolus à l'exécution
- [ ] **B.** 3 — une par type concret utilisé (**monomorphisation**)
- [ ] **C.** 0 — tout est inliné
- [ ] **D.** Indéterminé

<details><summary>Réponse</summary>

**B.** À la compilation, Rust génère une copie spécialisée par type instancié : `highest_u32`, `highest_f64`, `highest_str`. Chacune est ensuite optimisée et inlinée indépendamment.

Conséquences :
- ✅ **zéro coût d'abstraction** — aussi rapide que trois fonctions écrites à la main
- ✅ appels **statiques**, inlinables, spécialisables par LLVM
- ❌ le binaire **gonfle** (*code bloat*) et les temps de compilation grimpent

C'est le contre-modèle exact de `dyn Trait` (ex09/Q5) : une seule copie, mais un saut indirect par appel.

Le compromis pratique quand une fonction générique est énorme : extraire le gros du corps dans une fonction non générique prenant un `&dyn Trait`, et ne garder que la fine enveloppe en générique.

</details>

---

### Q2. 🔥 Pourquoi le README impose-t-il `PartialOrd` et non `Ord` ?

- [ ] **A.** `Ord` n'existe pas pour les types numériques
- [ ] **B.** Parce que `f64` n'implémente **pas** `Ord` : `NaN` n'est comparable à rien, pas même à lui-même
- [ ] **C.** `PartialOrd` est plus rapide
- [ ] **D.** Pour accepter les `String`

<details><summary>Réponse</summary>

**B.** IEEE-754 impose que `NaN != NaN`. Un ordre **total** (`Ord`) exige la trichotomie : pour tout `a`, `b`, exactement un de `a < b`, `a == b`, `a > b` est vrai. `NaN` viole ça pour toute valeur. Donc :

```rust
f64: PartialOrd ✅    f64: Ord ❌
f64: PartialEq ✅     f64: Eq  ❌
```

Ce qui te ferme des portes concrètes :

```rust
bucket.items.sort_by_key(|m| m.value);          // ❌ exige Ord
bucket.items.iter().max_by_key(|m| m.value);    // ❌ exige Ord
bucket.items.iter().max();                      // ❌ exige Ord
HashMap<f64, T>                                 // ❌ exige Eq + Hash
```

D'où les contournements du README :

```rust
.reduce(|acc, m| if m.value >= acc.value { m } else { acc })     // PartialOrd suffit
.max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())          // ⚠️ panique sur NaN
.max_by(|a, b| a.value.total_cmp(&b.value))                     // ✅ Rust 1.62+, ordre total sur f64
```

`total_cmp` est la bonne réponse moderne : elle définit un ordre total sur `f64` (les `NaN` sont rangés aux extrémités) sans jamais paniquer.

</details>

---

### Q3. Ce code compile-t-il ?

```rust
struct Bucket<T> { name: String, items: Vec<Measurement<T>> }

impl<T> Bucket<T> {
    fn new(name: &str) -> Self { ... }
    fn len(&self) -> usize { self.items.len() }
}

impl<T> Bucket<T> where T: PartialOrd {
    fn max_value(&self) -> Option<&Measurement<T>> { ... }
}
```

- [ ] **A.** Non : un type ne peut avoir qu'un seul bloc `impl`
- [ ] **B.** Non : les contraintes doivent être sur la définition de la struct
- [ ] **C.** Oui — et c'est **idiomatique**
- [ ] **D.** Oui, mais `new` sera indisponible si `T: PartialOrd`

<details><summary>Réponse</summary>

**C.** Les blocs `impl` conditionnés par des bornes différentes sont non seulement légaux, mais la bonne pratique.

Le sens exact : `new` et `len` existent pour **tout** `T` ; `max_value` n'existe **que** si `T: PartialOrd`. Un `Bucket<MonTypeSansOrdre>` est parfaitement constructible, il n'aura simplement pas la méthode.

C'est ce que fait la stdlib partout — `Option<T>` a des dizaines de blocs conditionnels, dont `impl<T: Default> Option<T> { fn unwrap_or_default() }`.

Le pattern à retenir : **mets la borne au plus près de l'endroit qui en a besoin.**

</details>

---

### Q4. 🔥 Pourquoi ne faut-il **pas** écrire ceci ?

```rust
struct Measurement<T: Display> { label: String, value: T }
```

- [ ] **A.** C'est une erreur de compilation
- [ ] **B.** Parce que la borne devra être **répétée sur chaque `impl`**, chaque fonction et chaque struct qui contient un `Measurement<T>` — sans rien apporter
- [ ] **C.** `Display` ne peut pas contraindre un champ
- [ ] **D.** Ça empêche la monomorphisation

<details><summary>Réponse</summary>

**B.** C'est légal, mais c'est un anti-pattern documenté (l'API Guidelines Rust le déconseillent explicitement).

Le problème : la borne devient **virale**.

```rust
struct Measurement<T: Display> { ... }
impl<T: Display> Measurement<T> { ... }           // obligé de la répéter
struct Bucket<T: Display> { items: Vec<Measurement<T>> }   // contaminé
fn summary<T: Display>(b: &Bucket<T>) { ... }              // contaminé
```

Et elle n'apporte **aucune sécurité** : elle ne fait qu'interdire de *construire* un `Measurement<T>` pour un `T` non-`Display`, alors que ça ne pose aucun problème tant qu'on ne l'affiche pas.

La règle : **jamais de borne sur la définition d'une struct**. Mets-les sur les `impl` qui en ont besoin — c'est exactement ce que fait l'énoncé :

```rust
struct Measurement<T> { ... }                                    // aucune borne
impl<T: Display> Display for Measurement<T> { ... }              // borne locale
```

Seule exception légitime : quand la borne est nécessaire au **type** lui-même, typiquement `struct Foo<T: ?Sized>` ou un paramètre de type associé.

</details>

---

### Q5. 🔥 Ce code compile-t-il ?

```rust
let b = Bucket::new("CPU");
println!("{}", b.len());
```

- [ ] **A.** Oui, `T` est inféré à `()`
- [ ] **B.** Non : `type annotations needed — cannot infer type for T`
- [ ] **C.** Oui, `T` vaut `i32` par défaut
- [ ] **D.** Non : `new` prend deux arguments

<details><summary>Réponse</summary>

**B.** Rien dans ce code ne contraint `T` : `new` ne le mentionne pas dans ses arguments, et `len()` ne renseigne rien. Il n'existe **pas** de type générique par défaut (le *fallback* `i32` ne vaut que pour les littéraux entiers).

Les trois façons de fixer `T` :

```rust
let b: Bucket<f64> = Bucket::new("CPU");        // annotation
let b = Bucket::<f64>::new("CPU");              // turbofish sur le type
let mut b = Bucket::new("CPU");
b.add(Measurement::new("web", 87.3));           // inférence par l'usage ultérieur
```

La troisième est celle que tu utiliseras naturellement dans `main` — Rust infère à l'échelle de la **fonction entière**, pas ligne par ligne.

</details>

---

### Q6. 🔥 Ce code compile-t-il ?

```rust
let p = Pair::new("web-server", 87.3_f64);
println!("{p}");
let s = p.swap();
println!("{s}");
println!("{p}");     // ← ici
```

avec `fn swap(self) -> Pair<B, A>`.

- [ ] **A.** Oui, affiche 3 lignes
- [ ] **B.** Non : `use of moved value: p` — `swap` prend `self` par valeur
- [ ] **C.** Non : `Pair<B, A>` n'est pas un type valide
- [ ] **D.** Non : il faut `p.swap()` deux fois

<details><summary>Réponse</summary>

**B.** `fn swap(self)` **consomme** la paire (cf. ex04/Q5). Après `let s = p.swap()`, `p` est déplacée.

Deux points de conception intéressants ici :

1. **Pourquoi `self` et pas `&self` ?** Parce que `swap` doit *déplacer* `first` et `second` dans la nouvelle paire. Avec `&self`, il faudrait cloner — donc exiger `A: Clone + B: Clone`. Consommer est plus général et gratuit.

2. **`Pair<A, B>` et `Pair<B, A>` sont deux types distincts.** `Pair<&str, f64>` ≠ `Pair<f64, &str>`, aussi différents que `String` et `u32`. D'où : `p.swap().swap()` retrouve bien le type `Pair<A, B>` de départ.

Si tu veux garder l'original, dérive `Clone` : `let s = p.clone().swap();`.

</details>

---

### Q7. Pourquoi `count_above` prend-elle `&T` et non `T` ?

```rust
fn count_above(&self, threshold: &T) -> usize
```

- [ ] **A.** Par convention
- [ ] **B.** Parce que `T` n'est contraint que par `PartialOrd` — rien ne garantit qu'il soit `Copy`, donc le prendre par valeur le **consommerait**
- [ ] **C.** Pour éviter une allocation
- [ ] **D.** Parce que `filter` exige une référence

<details><summary>Réponse</summary>

**B.** C'est le réflexe clé du code générique : **tu ne sais rien de `T` au-delà de ses bornes.**

Avec `threshold: T`, la fonction consommerait le seuil — un appel dans une boucle deviendrait impossible pour un `T` non-`Copy` (une `String`, un `Vec`…). Alors qu'avec `&T` ça marche pour tous les `T`, y compris `f64`.

Tu pourrais ajouter `T: Copy` et prendre `T` par valeur, mais tu restreindrais inutilement l'API. La hiérarchie de préférence en générique :

```
&T  >  T: Copy  >  T: Clone (+ .clone())  >  T par valeur
```

Dans le corps, `.filter(|m| m.value > *threshold)` déréférence le seuil. La comparaison `PartialOrd` fonctionne aussi directement entre `&T` et `&T` (`impl PartialOrd<&B> for &A`), donc `m.value > *threshold` et `&m.value > threshold` sont tous deux valides.

</details>

---

### Q8. 🔥 Quelle est l'erreur ici ?

```rust
impl<T> std::fmt::Display for Measurement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.label, self.value)
    }
}
```

- [ ] **A.** Aucune
- [ ] **B.** `T doesn't implement Display` — il faut `impl<T: Display> Display for Measurement<T>`
- [ ] **C.** Il manque le `?`
- [ ] **D.** `write!` n'accepte pas deux arguments

<details><summary>Réponse</summary>

**B.** Dans un `impl` générique, tu ne peux utiliser de `T` **que** ce que ses bornes garantissent. Ici `T` est totalement libre, donc `{}` sur `self.value` est refusé :

```
error[E0277]: `T` doesn't implement `std::fmt::Display`
help: consider restricting type parameter `T`: `<T: std::fmt::Display>`
```

C'est la différence fondamentale avec les generics de Java ou les templates C++ :
- **C++** : le template est vérifié à l'*instanciation* → erreurs illisibles de 300 lignes
- **Rust** : la **définition** est vérifiée une fois, contre ses bornes. Si elle compile, toute instanciation valide compile

Corollaire pratique : quand le compilateur te dit `consider restricting type parameter`, il a presque toujours raison — copie sa suggestion.

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 8/8 | Generics maîtrisés. Q2 (`f64`/`Ord`) et Q4 (bornes virales) sont du niveau code de prod |
| 6-7/8 | Bien. Retiens Q3 et Q4 : **la borne au plus près de son usage**, jamais sur la struct |
| < 6 | Reprends Q8 puis Q1 : comprendre la monomorphisation explique tout le reste |
