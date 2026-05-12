fn longest_name<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

struct Item {
    name: String,
}

struct InventoryRef<'a> {
    name: &'a str,
    inventaire: Vec<Item>,
}

impl InventoryRef<'_> {
    fn describe(&self) {
        println!("Category: {} | {} items", self.name, self.inventaire.len())
    }
}

fn main() {
    let category = String::from("Weapons");
    // drop(category);

    let inventory_ref = InventoryRef {
        name: &category,
        inventaire: vec![
            Item {
                name: String::from("Greatsword"),
            },
            Item {
                name: String::from("Sword"),
            },
        ],
    };

    let mut plus_long = "";

    for item in inventory_ref.inventaire.iter() {
        plus_long = longest_name(plus_long, &item.name);
    }

    println!("Longest: {}", plus_long);
    inventory_ref.describe();
}
