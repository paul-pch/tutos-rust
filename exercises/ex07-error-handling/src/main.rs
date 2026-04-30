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
            println!("- {} | {}kg | {} gold", self.name, self.weight, self.value);
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

    match load_items("exercises/ex07-error-handling/items.csv") {
        Ok(items) => {
            println!("Loaded {} items:", items.len());
            for item in items {
                item.display();
            }
        }
        Err(e) => println!("Erreur : {}", e),
    }
}
