use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex01-variables")
        .unwrap()
        .assert()
        .success()
        .stdout("Score: 10\nLives: 2\n");
}
