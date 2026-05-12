use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex09-traits")
        .unwrap()
        .assert()
        .success()
        .stdout("[Item] Sword — 5.5kg, 120 gold\n[Item] Potion — 0.5kg, 30 gold\n[Player] Alice — 100 HP\n");
}
