use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex08-error-customs")
        .unwrap()
        .assert()
        .success()
        .stdout("Loaded 3 items:\n- Sword | 5.5kg | 120 gold\n- Rope | 1.0kg | 10 gold\n- Potion | 0.5kg | 30 gold\n");
}
