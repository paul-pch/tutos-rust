fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    struct Player {
        name: String,
        health: u32,
    }

    let p = Player {
        name: String::from("Alice"),
        health: 100,
    };

    println!("{}", p.name);

    let mut p2 = Player {
        name: String::from("Bob"),
        health: 100,
    };

    p2.health = 80;

    impl Player {
        fn _is_alive(&self) -> bool {
            self.health > 0
        }

        fn _take_damage(&mut self, damage: u32) {
            self.health -= damage;
        }

        fn new(name: &str) -> Player {
            Player {
                name: name.to_string(),
                health: 100,
            }
        }
    }

    let p = Player::new("Alice");
    println!("{:?}", p);
    println!("{:#?}", p);

    // =================================

    #[derive(Debug)]
    struct Item {
        name: String,
        weight: f32,
        value: u32,
    }

    impl Item {
        fn new(name: &str, weight: f32, value: u32) -> Item {
            Item {
                name: name.to_string(),
                weight: weight,
                value: value,
            }
        }

        fn describe(&self) {
            println!(
                "Item: {} | Weight: {}kg | Value: {} gold",
                self.name, self.weight, self.value
            );
            println!("  -> Valuable: {}", self.is_valuable());
        }

        fn is_valuable(&self) -> bool {
            self.value > 50
        }
    }

    let it1 = Item::new("Sword", 5.5, 120);
    let it2 = Item::new("Rope", 1.0, 10);

    it1.describe();
    it2.describe();
}
