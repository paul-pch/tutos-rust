use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex10-lifetimes")
        .unwrap()
        .assert()
        .success()
        .stdout("Longest: Greatsword\nCategory: Weapons | 2 items\n");
}
