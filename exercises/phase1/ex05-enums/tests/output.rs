use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex05-enums")
        .unwrap()
        .assert()
        .success()
        .stdout("[Weapon] Sword | 5.5kg | attack bonus: 15\n[Armor] Shield | 8.0kg | defense bonus: 10\n[Consumable] Health Potion | 0.5kg | heals 50 hp\n[Quest] Ancient Key | 0.1kg | quest item\n");
}
