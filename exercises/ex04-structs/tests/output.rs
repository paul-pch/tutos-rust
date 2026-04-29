use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex04-structs")
        .unwrap()
        .assert()
        .success()
        .stdout("Item: Sword | Weight: 5.5kg | Value: 120 gold\n  -> Valuable: true\nItem: Rope | Weight: 1.0kg | Value: 10 gold\n  -> Valuable: false
");
}
