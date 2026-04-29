use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex02-types")
        .unwrap()
        .assert()
        .success()
        .stdout("Player: Alice\nTotal score: 115\n");
}
