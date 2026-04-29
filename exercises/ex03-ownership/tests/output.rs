use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex03-ownership")
        .unwrap()
        .assert()
        .success()
        .stdout("Hello, Paul!\nPaul\nPAUL\n");
}
