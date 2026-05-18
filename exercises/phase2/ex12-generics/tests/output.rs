use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex12-generics")
        .unwrap()
        .assert()
        .success()
        .stdout(
            "=== CPU Usage (%) ===\n\
             5 measurements\n\
             Max: web-server = 87.3\n\
             Min: cache = 12.1\n\
             Above 50.0: 3\n\
             \n\
             === Memory (bytes) ===\n\
             4 measurements\n\
             Max: database = 8589934592\n\
             Min: cache = 134217728\n\
             Above 1000000000: 1\n\
             \n\
             --- Pairs ---\n\
             (web-server, 87.3)\n\
             Swapped: (87.3, web-server)\n\
             (database, 8589934592)\n\
             Swapped: (8589934592, database)\n",
        );
}
