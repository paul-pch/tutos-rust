fn main() {
    struct Item {
        name: String,
        weight: f32,
        value: u32,
    }

    impl Item {
        fn new(name: String, weight: f32, value: u32) -> Self {
            Item {
                name,
                weight,
                value,
            }
        }

        fn display(&self) {
            println!(
                "- {} | {:.1}kg | {} gold",
                self.name, self.weight, self.value
            );
        }
    }

    fn load_items(path: &str) -> Result<Vec<Item>, std::io::Error> {
        let mut items: Vec<Item> = Vec::new();
        let content = std::fs::read_to_string(path)?;
        for line in content.lines() {
            let parts: Vec<&str> = line.split(",").collect();
            items.push(Item::new(
                parts[0].to_string(),
                parts[1].parse::<f32>().unwrap(),
                parts[2].parse::<u32>().unwrap(),
            ));
        }
        Ok(items)
    }

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/items.csv");

    match load_items(path) {
        Ok(items) => {
            println!("Loaded {} items:", items.len());
            for item in items {
                item.display();
            }
        }
        Err(e) => println!("Erreur : {}", e),
    }
}
