# ex17-threads — threads, `join` et portées

## Concept

### Lancer un thread

`thread::spawn` démarre un thread OS et rend un `JoinHandle<T>` où `T` est le type retourné
par la closure. `join()` attend la fin du thread et rend un `Result` : `Err` si le thread a
paniqué.

```rust
use std::thread;

let handle = thread::spawn(|| {
    // exécuté en parallèle du thread principal
    41 + 1
});

let value = handle.join().unwrap(); // 42 — bloque jusqu'à la fin du thread
```

Si le thread principal se termine, le processus s'arrête : les threads non joints sont tués
sans avertissement. `join()` n'est donc pas une politesse, c'est ce qui garantit que le travail
a eu lieu.

### `move` et la contrainte `'static`

La closure passée à `spawn` doit être `'static` : le compilateur ne sait pas quand le thread
finira, donc il refuse qu'elle emprunte quoi que ce soit de plus court que le programme.

```rust
let jobs = vec![1, 2, 3];

thread::spawn(move || {        // sans `move` : erreur, `jobs` ne vit pas assez longtemps
    println!("{}", jobs.len()); // `jobs` appartient maintenant au thread
});
// `jobs` n'est plus utilisable ici
```

D'où le réflexe habituel — cloner, ou envelopper dans un `Arc` — pour partager une donnée
entre plusieurs threads.

### `thread::scope` — emprunter sans `'static`

Depuis Rust 1.63, `thread::scope` crée une portée dont **tous les threads sont joints avant
qu'elle ne rende la main**. Le compilateur le sait, donc il autorise les emprunts de données
locales : ni `clone`, ni `Arc`.

```rust
let jobs = vec![1, 2, 3];
let mut handles = Vec::new();

thread::scope(|s| {
    for job in &jobs {              // emprunt simple, pas de move
        handles.push(s.spawn(move || job * 2));
    }
    // implicite : tous les threads sont joints à la sortie du bloc
});
```

`s.spawn` rend un `ScopedJoinHandle` : on peut le joindre soi-même pour récupérer les valeurs,
dans l'ordre où on veut.

### Le piège qui annule tout le bénéfice

```rust
// ❌ séquentiel déguisé — on attend chaque thread avant de lancer le suivant
for job in jobs {
    let handle = thread::spawn(move || work(job));
    handle.join().unwrap();
}

// ✅ on lance tout, puis on attend
let handles: Vec<_> = jobs.into_iter().map(|j| thread::spawn(move || work(j))).collect();
for handle in handles {
    handle.join().unwrap();
}
```

Joindre dans l'ordre de lancement rend les résultats **déterministes** même si l'exécution ne
l'est pas : c'est ce qui permet d'écrire un test fiable sur du code concurrent.

### Combien de threads ?

`std::thread::available_parallelism()` donne le parallélisme utilisable. Au-delà, on paie la
création de threads sans rien gagner — sauf pour des tâches bloquées en I/O, où le CPU n'est
pas le facteur limitant.

---

## Exercice

Un vérificateur de santé : N services à sonder, chaque sonde passe l'essentiel de son temps à
attendre le réseau. Le cas d'école du gain par parallélisation.

La logique va dans `src/lib.rs`, `src/main.rs` se contente d'appeler et d'afficher.

### Étape 1 — modéliser la sonde

Une struct décrivant un service à sonder : son nom, la latence qu'il met à répondre, et s'il
répond correctement. Une méthode qui exécute la sonde : elle simule l'attente réseau avec
`thread::sleep`, mesure le temps réellement écoulé avec `std::time::Instant`, et rend un
résultat.

### Étape 2 — modéliser le résultat

Une struct portant le nom du service, son état (up/down) et la durée mesurée. Elle implémente
`Display` au format :

```
  api          UP      121ms
```

Deux espaces d'indentation, le nom sur 12 colonnes cadré à gauche, un espace, l'état sur
6 colonnes cadré à gauche, un espace, la durée en millisecondes sur 4 colonnes cadrée à droite,
suivie de `ms`.

### Étape 3 — les deux stratégies

Deux fonctions prenant `&[Sonde]` et rendant `Vec<Résultat>` :

- une version **séquentielle**, qui sonde les services l'un après l'autre ;
- une version **parallèle**, qui les sonde tous en même temps.

Les deux doivent rendre les résultats **dans l'ordre des sondes en entrée**. La version
parallèle emprunte la slice sans la cloner.

### Étape 4 — le rapport

Une fonction qui, à partir des résultats, donne le nombre de services up. Une autre qui donne
la durée totale d'une campagne.

### Étape 5 — `main`

Sonder ces quatre services, dans cet ordre :

| service | latence | état |
|---|---|---|
| api | 120 ms | up |
| database | 80 ms | up |
| cache | 50 ms | down |
| queue | 200 ms | up |

Afficher la campagne séquentielle, puis la campagne parallèle, puis la ligne de synthèse avec
le rapport des deux durées totales, à deux décimales.

### Étape 6 — les tests

Dans `src/lib.rs`, écrire les tests unitaires couvrant :

- la version parallèle rend les résultats dans l'ordre des entrées ;
- les deux versions s'accordent sur l'état de chaque service ;
- la campagne parallèle est plus courte que la somme des latences ;
- le comptage des services up, y compris sur une liste vide.

---

## Output attendu

Les durées mesurées varient d'une exécution à l'autre — seul le format est figé.

```
=== Sequential ===
  api          UP      121ms
  database     UP       81ms
  cache        DOWN     51ms
  queue        UP      201ms
  total: 456ms

=== Parallel ===
  api          UP      121ms
  database     UP       81ms
  cache        DOWN     51ms
  queue        UP      201ms
  total: 202ms

3/4 up — speedup x2.26
```

## Pistes

- La lib s'importe sous le nom `ex17_threads` — les tirets deviennent des underscores.
- `Duration::as_millis()` rend un `u128`. Réfléchis au type que tu veux réellement manipuler
  et à l'endroit où faire la conversion.
- La version parallèle a besoin d'emprunter la slice de sondes. `thread::spawn` te le
  refusera ; lis le message du compilateur en entier, il nomme la contrainte en cause.
- Pour que l'ordre des résultats soit garanti, ce n'est pas l'ordre d'exécution qu'il faut
  contrôler mais l'ordre de collecte.
- Un test qui affirme « le parallèle est plus rapide » sans chiffre ne teste rien. Compare la
  durée mesurée à une borne que seule l'exécution parallèle peut tenir.
- Le speedup n'est pas exactement 4 alors qu'il y a 4 sondes : la campagne parallèle dure le
  temps de la sonde la plus lente.

## Lancer l'exercice

```sh
cargo run -p ex17-threads
cargo test -p ex17-threads --lib   # tes tests unitaires
cargo test -p ex17-threads         # tout, y compris l'output attendu
```
