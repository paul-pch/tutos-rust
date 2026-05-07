use std::fmt::{self};

fn main() {
    #[derive(Debug)]
    enum ParseError {
        InvalidFormat(String),
        InvalidNumber(String),
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ParseError::InvalidFormat(s) => write!(f, "Format invalide : {}", s),
                ParseError::InvalidNumber(s) => write!(f, "Nombre invalide : {}", s),
            }
        }
    }

    impl std::error::Error for ParseError {}
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

    fn load_items(path: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let mut items: Vec<Item> = Vec::new();

        let content =
            std::fs::read_to_string(path).map_err(|e| ParseError::InvalidFormat(e.to_string()))?;

        for line in content.lines() {
            let parts: Vec<&str> = line.split(",").collect();

            if parts.len() != 3 {
                return Err(ParseError::InvalidFormat(line.to_string()).into());
            }

            items.push(Item::new(
                parts[0].to_string(),
                parts[1]
                    .parse::<f32>()
                    .map_err(|e| ParseError::InvalidNumber(e.to_string()))?,
                parts[2]
                    .parse::<u32>()
                    .map_err(|e| ParseError::InvalidNumber(e.to_string()))?,
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
