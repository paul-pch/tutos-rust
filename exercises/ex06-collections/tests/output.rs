use assert_cmd::Command;

#[test]
fn test_output() {
    Command::cargo_bin("ex06-collections")
        .unwrap()
        .assert()
        .success()
        .stdout("=== Inventaire ===\nÉpée de fer (Common) - 10 gold\nPotion de soin (Common) - 5 gold\nBouclier d'acier (Rare) - 50 gold\nAmulette dragon (Legendary) - 500 gold\n\n=== Par catégorie ===\nArmes :\n  Épée de fer (Common) - 10 gold\nArmures :\n  Bouclier d'acier (Rare) - 50 gold\nPotions :\n  Potion de soin (Common) - 5 gold\n  Amulette dragon (Legendary) - 500 gold\n\nValeur totale de l'inventaire : 565 gold\n");
}
