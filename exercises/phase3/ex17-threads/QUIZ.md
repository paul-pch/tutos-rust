# QUIZ — ex17 : Threads, `join` et portées

> 9 questions. 🔥 = cas tricky.
> Coche `- [x]` ta réponse **sans déplier la correction**, puis demande la relecture du quiz.

---

### Q1. 🔥 Quelle est la durée totale de cette boucle, avec 4 sondes de 120, 80, 50 et 200 ms ?

```rust
for probe in probes {
    let handle = thread::spawn(move || probe.run());
    results.push(handle.join().unwrap());
}
```

- [ ] **A.** ~200 ms — les threads tournent en parallèle
- [ ] **B.** ~450 ms — **séquentiel déguisé**
- [ ] **C.** ~120 ms
- [ ] **D.** Indéterminé

<details><summary>Réponse</summary>

**B — ~450 ms**, soit exactement la somme. Aucun gain.

`join()` **bloque** jusqu'à la fin du thread. En le plaçant à l'intérieur de la boucle, tu attends chaque thread avant de lancer le suivant : à aucun instant deux threads ne tournent ensemble. Tu as payé le coût de création de 4 threads (~50 µs chacun) pour rien.

Le pattern correct, en **deux temps** :

```rust
// 1. tout lancer
let handles: Vec<_> = probes.iter()
    .map(|p| thread::spawn(move || p.run()))
    .collect();                                   // ← le collect est ESSENTIEL

// 2. tout attendre
let results: Vec<_> = handles.into_iter()
    .map(|h| h.join().unwrap())
    .collect();
```

⚠️ Le `.collect()` de l'étape 1 n'est pas cosmétique. Sans lui, l'itérateur reste **paresseux** (ex11/Q1) et le `map` de l'étape 2 tirerait un thread à la fois — tu reproduirais exactement le bug d'origine, en plus subtil.

</details>

---

### Q2. Pourquoi `move` est-il nécessaire ici ?

```rust
let jobs = vec![1, 2, 3];
thread::spawn(|| println!("{}", jobs.len()));
```

- [ ] **A.** Pour rendre `jobs` mutable
- [ ] **B.** Parce que `spawn` exige `F: 'static` : le compilateur ne sait pas quand le thread finira, il refuse tout emprunt de durée limitée
- [ ] **C.** Pour copier `jobs`
- [ ] **D.** `move` est optionnel

<details><summary>Réponse</summary>

**B.** La signature réelle :

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where F: FnOnce() -> T + Send + 'static, T: Send + 'static
```

Trois contraintes, chacune pour une raison :
- **`'static`** — le thread peut survivre à la fonction qui l'a lancé (rien n'oblige à `join`). Toute référence vers une variable locale pourrait donc devenir pendouillante
- **`Send`** — la valeur doit pouvoir traverser la frontière de thread. C'est ce qui bloque `Rc` (ex15/Q7)
- **`T: Send`** — la valeur de retour repasse la frontière dans l'autre sens, via `join()`

Rappel de ex10/Q5 : **`F: 'static` ne veut pas dire « vit pour toujours »**, mais « ne contient aucune référence de durée limitée ». Une `String` owned satisfait `'static`. C'est précisément ce que `move` accomplit : capturer par valeur au lieu d'emprunter.

</details>

---

### Q3. 🔥 Ce code compile-t-il ?

```rust
let probes = vec![p1, p2, p3];
let results: Vec<_> = probes.iter()
    .map(|p| thread::spawn(move || p.run()))
    .collect();
```

- [ ] **A.** Oui
- [ ] **B.** Non : `p` est un `&Probe` emprunté à `probes`, qui ne vit pas `'static` — `move` ne sauve rien
- [ ] **C.** Oui, `move` rend la référence `'static`
- [ ] **D.** Non : il manque `.into_iter()`

<details><summary>Réponse</summary>

**B.** Piège classique : `move` capture le `&Probe` **par valeur**, mais une référence copiée reste une référence. Sa durée de vie est celle de `probes`, pas `'static`.

```
error[E0521]: borrowed data escapes outside of function
              argument requires that `probes` is borrowed for `'static`
```

C'est exactement l'indice que donne le README : *« lis le message du compilateur en entier, il nomme la contrainte en cause »*.

Trois sorties possibles :

| Solution | Coût | Quand |
|---|---|---|
| `probes.into_iter()` + `move` | consomme `probes` | tu n'en as plus besoin |
| `Arc<Vec<Probe>>` + clone par thread | une allocation atomique | partage long, threads détachés |
| **`thread::scope`** | **zéro** | 👈 la réponse de cet exercice |

</details>

---

### Q4. 🔥 Pourquoi `thread::scope` autorise-t-il l'emprunt que Q3 refuse ?

```rust
thread::scope(|s| {
    for probe in probes {                     // &Probe, sans move ni clone
        handles.push(s.spawn(|| probe.run()));
    }
});
```

- [ ] **A.** `scope` désactive le borrow checker
- [ ] **B.** Parce que `scope` **garantit** que tous ses threads sont joints avant de rendre la main — le compilateur peut donc prouver que les emprunts ne survivent pas
- [ ] **C.** `scope` clone les données
- [ ] **D.** `s.spawn` est `unsafe`

<details><summary>Réponse</summary>

**B.** C'est une garantie **structurelle**, pas un contournement.

`thread::scope` (stable depuis Rust 1.63) joint tous les threads non joints à la sortie du bloc — y compris en cas de `panic!`. Le compilateur a donc une borne supérieure sur leur durée de vie, et `s.spawn` peut relâcher la contrainte `'static` en `'scope` :

```rust
pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
where F: FnOnce() -> T + Send + 'scope       // ← 'scope, pas 'static
```

Ni `clone`, ni `Arc`, ni allocation. C'est la manière moderne de faire du parallélisme de données en Rust standard, et le cœur de cet exercice.

Détail : `s.spawn` rend un `ScopedJoinHandle` que tu peux joindre **toi-même** pour récupérer les valeurs dans l'ordre que tu veux — ce dont tu as besoin pour l'étape 3.

</details>

---

### Q5. Que rend `join()` exactement ?

- [ ] **A.** `T`
- [ ] **B.** `Result<T, Box<dyn Any + Send>>` — `Err` si le thread a **paniqué**
- [ ] **C.** `Option<T>`
- [ ] **D.** `()`

<details><summary>Réponse</summary>

**B.** Un panic dans un thread enfant **ne tue pas** le processus : il termine ce thread, et `join()` te le rapporte en `Err`.

C'est une propriété d'isolation utile : un thread de travail qui plante ne fait pas tomber le serveur. Mais ça veut aussi dire qu'un `.unwrap()` sur `join()` **relaie** le panic dans le thread principal — souvent ce qu'on veut, parfois non.

L'erreur est un `Box<dyn Any + Send>` : le *payload* du panic, tel que passé à `panic!`. Pour le lire :

```rust
match handle.join() {
    Ok(v)  => v,
    Err(e) => {
        if let Some(s) = e.downcast_ref::<&str>()   { eprintln!("panic: {s}"); }
        if let Some(s) = e.downcast_ref::<String>() { eprintln!("panic: {s}"); }
    }
}
```

Note aussi : par défaut, le message de panic du thread enfant est déjà imprimé sur `stderr` au moment où il survient.

</details>

---

### Q6. 🔥 Que produit ce programme ?

```rust
fn main() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        println!("terminé");
    });
    println!("main fini");
}
```

- [ ] **A.** `main fini` puis `terminé` 500 ms après
- [ ] **B.** `main fini` seulement — le processus s'arrête, le thread est **tué**
- [ ] **C.** `terminé` puis `main fini`
- [ ] **D.** Erreur de compilation

<details><summary>Réponse</summary>

**B.** Quand `main` retourne, le runtime appelle `exit()` : le processus meurt et **tous** les threads non joints sont détruits sans avertissement, sans dérouler leur pile, sans exécuter leurs `Drop`.

Le README le dit bien : *« `join()` n'est pas une politesse, c'est ce qui garantit que le travail a eu lieu »*.

Et le compilateur ne t'aide pas ici : `JoinHandle` n'est **pas** `#[must_use]`, parce que détacher un thread est parfois volontaire (un thread de log, un serveur en arrière-plan).

Corollaire pour ta version parallèle : sans `join`, tu ne mesures rien et tu n'obtiens aucun résultat — pas seulement un résultat approximatif.

</details>

---

### Q7. Comment garantir que les résultats sortent **dans l'ordre des sondes en entrée** ?

- [ ] **A.** En joignant les threads dans l'ordre de lancement
- [ ] **B.** En triant les résultats à la fin
- [ ] **C.** En utilisant un `Mutex` partagé
- [ ] **D.** En lançant les threads dans l'ordre

<details><summary>Réponse</summary>

**A.** L'ordre d'**exécution** est non déterministe et hors de ton contrôle. L'ordre de **collecte**, lui, t'appartient entièrement :

```rust
let handles: Vec<_> = probes.iter().map(|p| s.spawn(|| p.run())).collect();
handles.into_iter().map(|h| h.join().unwrap()).collect()
//     ↑ le Vec préserve l'ordre de lancement, join() ne fait qu'attendre
```

`join()` attend que *ce* thread précis ait fini. Si le thread 0 termine en dernier, tu attends — mais le thread 3 a déjà fini pendant ce temps, son `join()` sera instantané. La durée totale reste celle du **plus lent**, et l'ordre est déterministe.

C'est la piste du README : *« ce n'est pas l'ordre d'exécution qu'il faut contrôler mais l'ordre de collecte »*. Et c'est ce qui rend un test fiable sur du code concurrent.

(B) fonctionnerait aussi mais suppose une clé de tri ; (C) est du travail inutile et sérialise l'écriture.

</details>

---

### Q8. 🔥 Pourquoi le speedup vaut-il ~2.26 et non 4, avec 4 sondes ?

Latences : 120, 80, 50, 200 ms.

- [ ] **A.** Le coût de création des threads
- [ ] **B.** Parce que la campagne parallèle dure le temps de la sonde **la plus lente** (200 ms), pas la moyenne — donc 450/200 ≈ 2.25
- [ ] **C.** Il n'y a que 2 cœurs
- [ ] **D.** `thread::sleep` est imprécis

<details><summary>Réponse</summary>

**B.** Séquentiel : 120+80+50+200 = **450 ms**. Parallèle : `max(120, 80, 50, 200)` = **200 ms**. Rapport ≈ **2.25** — l'output attendu montre 2.26, l'écart venant du surcoût réel mesuré.

Le facteur 4 ne serait atteint que si les 4 sondes avaient **exactement** la même latence. C'est la loi d'Amdahl appliquée au cas le plus simple : **ton chemin critique est ta tâche la plus lente**.

Conséquence pratique : pour améliorer ce système, inutile d'ajouter des threads — il faut découper ou accélérer la sonde `queue` (200 ms).

Note que (A) et (C) sont fausses **ici** parce que les sondes sont bloquées en I/O (`sleep`), pas en calcul : les 4 threads dorment simultanément même sur un seul cœur. Le nombre de cœurs ne limiterait que du travail **CPU-bound**.

</details>

---

### Q9. Comment écrire un test qui vérifie réellement le gain de parallélisme ?

```rust
#[test]
fn parallel_is_faster() {
    let probes = vec![p(100), p(100), p(100)];
    // ?
}
```

- [ ] **A.** `assert!(parallel_duration < sequential_duration)`
- [ ] **B.** `assert!(parallel_duration < Duration::from_millis(250))` — une borne que **seule** l'exécution parallèle peut tenir
- [ ] **C.** Mesurer le nombre de threads créés
- [ ] **D.** On ne peut pas tester du code concurrent

<details><summary>Réponse</summary>

**B.** C'est la piste du README : *« un test qui affirme "le parallèle est plus rapide" sans chiffre ne teste rien »*.

(A) est un faux test : sur une machine chargée, `parallel` peut ponctuellement dépasser `sequential` → **flaky**. Et il passerait aussi si les deux implémentations étaient séquentielles mais que l'une est marginalement plus rapide.

(B) est un vrai test : 3 sondes de 100 ms font 300 ms en séquentiel. Une borne à 250 ms est **structurellement** hors d'atteinte pour une implémentation séquentielle, tout en laissant une marge confortable pour le surcoût des threads (~1 ms) et le bruit de l'ordonnanceur.

Le principe général du test de concurrence : **choisis un seuil que seule l'implémentation correcte peut franchir**, avec une marge généreuse. Pas de comparaison relative, pas de `sleep` arbitraire dans le test lui-même.

Les autres tests demandés par l'énoncé suivent la même logique, et ils sont **déterministes** — c'est pour ça que Q7 comptait :
- l'ordre des résultats correspond à l'ordre des entrées
- séquentiel et parallèle s'accordent sur l'état de chaque service
- `count_up(&[])` vaut `0` (le cas vide, toujours à tester)

</details>

---

## Auto-évaluation

| Score | Verdict |
|---|---|
| 9/9 | Tu as compris la concurrence Rust — Q3/Q4 (`'static` vs `scope`) sont le cœur du sujet |
| 7-8/9 | Solide. Retiens Q1 : le `collect()` intermédiaire n'est pas optionnel |
| < 7 | Reprends Q2 puis Q4 : tant que la contrainte `'static` de `spawn` n'est pas claire, `scope` ressemble à de la magie |
