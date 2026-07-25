use assert_cmd::Command;
use predicates::prelude::*;

/// Les durées mesurées varient d'une exécution à l'autre : on assert le format,
/// pas les valeurs.
#[test]
fn test_output() {
    Command::cargo_bin("ex17-threads")
        .unwrap()
        .assert()
        .success()
        .stdout(
            predicate::str::contains("=== Sequential ===")
                .and(predicate::str::contains("=== Parallel ==="))
                .and(predicate::str::is_match(r"(?m)^  api {10}UP {4}\s*\d+ms$").unwrap())
                .and(predicate::str::is_match(r"(?m)^  database {5}UP {4}\s*\d+ms$").unwrap())
                .and(predicate::str::is_match(r"(?m)^  cache {8}DOWN {2}\s*\d+ms$").unwrap())
                .and(predicate::str::is_match(r"(?m)^  queue {8}UP {4}\s*\d+ms$").unwrap())
                .and(predicate::str::is_match(r"(?m)^  total: \d+ms$").unwrap())
                .and(predicate::str::is_match(r"3/4 up — speedup x\d+\.\d{2}").unwrap()),
        );
}

/// Le fond de l'exercice : la campagne parallèle dure le temps de la sonde la plus
/// lente (200 ms), pas la somme des latences (450 ms).
#[test]
fn test_parallel_is_faster() {
    let output = Command::cargo_bin("ex17-threads")
        .unwrap()
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let totals: Vec<u64> = stdout
        .lines()
        .filter_map(|line| line.trim().strip_prefix("total: "))
        .filter_map(|total| total.trim_end_matches("ms").parse().ok())
        .collect();

    assert_eq!(totals.len(), 2, "deux lignes `total:` attendues");
    let (sequential, parallel) = (totals[0], totals[1]);

    assert!(
        sequential >= 450,
        "la campagne séquentielle doit cumuler les latences, mesuré : {sequential}ms"
    );
    assert!(
        (200..350).contains(&parallel),
        "la campagne parallèle doit durer le temps de la sonde la plus lente, mesuré : {parallel}ms"
    );
}
