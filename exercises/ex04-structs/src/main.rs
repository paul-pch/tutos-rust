fn main() {
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
                weight,
                value,
            }
        }

        fn describe(&self) {
            println!(
                "Item: {} | Weight: {:.1}kg | Value: {} gold",
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
