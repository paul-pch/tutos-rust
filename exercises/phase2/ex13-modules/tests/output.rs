use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex13-modules")
        .unwrap()
        .assert()
        .success()
        .stdout(
            "=== DevOps Queue ===\n\
             \n\
             Pending tasks:\n\
             \x20\x20[HIGH  ] #3 Deploy to production\n\
             \x20\x20[MEDIUM] #4 Update load balancer\n\
             \x20\x20[LOW   ] #5 Archive old logs\n\
             \n\
             Completed tasks:\n\
             \x20\x20[HIGH  ] #1 Fix critical bug\n\
             \x20\x20[MEDIUM] #2 Run integration tests\n\
             \n\
             3 pending, 2 done\n",
        );
}
