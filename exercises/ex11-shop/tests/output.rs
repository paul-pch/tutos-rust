use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex11-shop")
        .unwrap()
        .assert()
        .success()
        .stdout(
            "=== Merchant Guild Shop ===\n\
             \n\
             [Weapon] Sword | 5.5 kg | 120 gold | x2\n\
             [Armor] Iron Shield | 8.0 kg | 85 gold | x1\n\
             [Consumable] Health Potion | 0.5 kg | 30 gold | x5\n\
             [Misc] Rope | 1.0 kg | 5 gold | x3\n\
             [Weapon] Dagger | 1.5 kg | 45 gold | x4\n\
             \n\
             Total weight: 30.5 kg\n\
             Total value: 670 gold\n\
             Shop gold: 500 gold\n\
             \n\
             Most valuable: Sword (120 gold)\n\
             Weapons: 2 types\n\
             \n\
             --- Transactions ---\n\
             Sold 2x Health Potion for 60 gold\n\
             Sold 3x Rope for 15 gold\n\
             Error: not enough stock for Dagger (have 4, want 10)\n\
             Error: item not found: Arrows\n\
             \n\
             Shop gold: 575 gold\n",
        );
}
