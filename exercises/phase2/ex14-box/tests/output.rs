use assert_cmd::Command;
#[test]
fn test_output() {
    Command::cargo_bin("ex14-box")
        .unwrap()
        .assert()
        .success()
        .stdout("[dir] /\n  [dir] src\n    [file] main.rs (1024 bytes)\n    [file] lib.rs (2048 bytes)\n  [file] README.md (512 bytes)\n  [dir] target\n    [dir] debug\n      [file] app (1048576 bytes)\nTotal size: 1052160 bytes\nDepth: 3\n");
}
