use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex15-rc-refcell")
        .unwrap()
        .assert()
        .success()
        .stdout("=== Infra Status ===\ndisk: up\ndatabase: up\napi: up\nweb: up\n\napi healthy? true\nweb healthy? true\n\n-- disk goes down --\n\napi healthy? false\nweb healthy? false\n\nreferences to disk: 2\nreferences to database: 3\n");
}
