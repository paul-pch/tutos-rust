fn main() {
    enum ItemKind {
        Armor { defense: u32 },
        Consumable { heal: u32 },
        Quest,
        Weapon { attack: u32 },
    }

    struct Item {
        name: String,
        weight: f32,
        kind: ItemKind,
    }

    impl Item {
        fn new(name: String, weight: f32, kind: ItemKind) -> Self {
            Item { name, weight, kind }
        }

        fn describe(&self) {
            match self.kind {
                ItemKind::Armor { defense } => println!(
                    "[Armor] {} | {:.1}kg | defense bonus: {}",
                    self.name, self.weight, defense
                ),
                ItemKind::Consumable { heal } => println!(
                    "[Consumable] {} | {:.1}kg | heals {} hp",
                    self.name, self.weight, heal
                ),
                ItemKind::Quest => {
                    println!("[Quest] {} | {:.1}kg | quest item", self.name, self.weight)
                }
                ItemKind::Weapon { attack } => println!(
                    "[Weapon] {} | {:.1}kg | attack bonus: {}",
                    self.name, self.weight, attack
                ),
            }
        }
    }

    Item::new(String::from("Sword"), 5.5, ItemKind::Weapon { attack: 15 }).describe();
    Item::new(String::from("Shield"), 8.0, ItemKind::Armor { defense: 10 }).describe();
    Item::new(
        String::from("Health Potion"),
        0.5,
        ItemKind::Consumable { heal: 50 },
    )
    .describe();
    Item::new(String::from("Ancient Key"), 0.1, ItemKind::Quest).describe();
}
