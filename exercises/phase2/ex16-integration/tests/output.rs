use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex16-integration")
        .unwrap()
        .assert()
        .success()
        .stdout("=== Cluster Scheduler (first-fit) ===\n\nscheduled p1 (3 cpu) -> node-a\nscheduled p2 (5 cpu) -> node-b\nscheduled p3 (2 cpu) -> node-b\nfailed to schedule p4: no node available\n\n--- Cluster state ---\nnode-a: 3/4 cpu\nnode-b: 7/8 cpu\n\ntotal capacity: 12 cpu\ntotal used: 10 cpu\n\n--- Error handling ---\nlookup ghost: node not found: ghost\nover-allocate node-a by 5: not enough resources on node-a (requested 5, available 1)\n");
}
