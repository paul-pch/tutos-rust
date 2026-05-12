use std::fmt;

#[derive(PartialEq)]
enum ItemCategory {
    Armor,
    Consumable,
    Misc,
    Weapon,
}

impl fmt::Display for ItemCategory {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemCategory::Armor => write!(_f, "[Armor]"),
            ItemCategory::Consumable => write!(_f, "[Consumable]"),
            ItemCategory::Misc => write!(_f, "[Misc]"),
            ItemCategory::Weapon => write!(_f, "[Weapon]"),
        }
    }
}

struct Item {
    name: String,
    category: ItemCategory,
    weight: f32,
    value: u32,
    quantity: u32,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} | {:.1} kg | {} gold | x{}",
            self.category, self.name, self.weight, self.value, self.quantity
        )
    }
}

impl Item {
    fn new(name: String, category: ItemCategory, weight: f32, value: u32, quantity: u32) -> Self {
        Item {
            name,
            category,
            weight,
            value,
            quantity,
        }
    }
}

enum ShopError {
    ItemNotFound(String),
    NotEnoughStock { item: String, have: u32, want: u32 },
}

impl fmt::Display for ShopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopError::ItemNotFound(s) => write!(f, "item not found: {}", s),
            ShopError::NotEnoughStock { item, have, want } => write!(
                f,
                "not enough stock for {} (have {}, want {})",
                item, have, want
            ),
        }
    }
}

struct Shop {
    name: String,
    inventory: Vec<Item>,
    gold: u32,
}

impl Shop {
    fn new(name: String, inventory: Vec<Item>, gold: u32) -> Self {
        Shop {
            name,
            inventory,
            gold,
        }
    }

    fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    fn total_weight(&self) -> f32 {
        self.inventory
            .iter()
            .map(|item| item.weight * item.quantity as f32)
            .sum()
    }

    fn total_value(&self) -> u32 {
        self.inventory
            .iter()
            .map(|item| item.value * item.quantity)
            .sum()
    }

    fn most_valuable(&self) -> Option<&Item> {
        self.inventory.iter().max_by_key(|item| item.value)
    }

    fn count_by_category(&self, cat: &ItemCategory) -> usize {
        self.inventory
            .iter()
            .filter(|item| item.category == *cat)
            .count()
    }

    fn sell(&mut self, name: &str, qty_needed: u32) -> Result<u32, ShopError> {
        let index = self.inventory.iter().position(|item| item.name == name);

        match index {
            Some(idx) => {
                if self.inventory[idx].quantity >= qty_needed {
                    self.inventory[idx].quantity -= qty_needed;
                    // TODO Ajouter la suppression de l'objet si qty null
                    self.gold += self.inventory[idx].value * qty_needed;
                    Ok(self.inventory[idx].value * qty_needed)
                } else {
                    Err(ShopError::NotEnoughStock {
                        item: name.to_string(),
                        have: self.inventory[idx].quantity,
                        want: qty_needed,
                    })
                }
            }
            None => Err(ShopError::ItemNotFound(name.to_string())),
        }
    }
}

fn main() {
    let mut shop = Shop::new(String::from("Merchant Guild Shop"), vec![], 500);

    shop.add_item(Item::new(
        String::from("Sword"),
        ItemCategory::Weapon,
        5.5,
        120,
        2,
    ));
    shop.add_item(Item::new(
        String::from("Iron Shield"),
        ItemCategory::Armor,
        8.0,
        85,
        1,
    ));
    shop.add_item(Item::new(
        String::from("Health Potion"),
        ItemCategory::Consumable,
        0.5,
        30,
        5,
    ));
    shop.add_item(Item::new(
        String::from("Rope"),
        ItemCategory::Misc,
        1.0,
        5,
        3,
    ));
    shop.add_item(Item::new(
        String::from("Dagger"),
        ItemCategory::Weapon,
        1.5,
        45,
        4,
    ));

    println!("=== {} ===\n", shop.name);
    for item in &shop.inventory {
        println!("{}", item);
    }

    println!("\nTotal weight: {:.1} kg", shop.total_weight());
    println!("Total value: {} gold", shop.total_value());
    println!("Shop gold: {} gold", shop.gold);

    match shop.most_valuable() {
        Some(item) => println!("\nMost valuable: {} ({} gold)", item.name, item.value),
        None => println!("empty"),
    }

    println!(
        "Weapons: {} types",
        shop.count_by_category(&ItemCategory::Weapon)
    );

    println!("\n--- Transactions ---");

    let name = "Health Potion";
    let qty = 2;
    match shop.sell(name, qty) {
        Ok(gold) => println!("Sold {}x {} for {} gold", qty, name, gold),
        Err(e) => println!("Error: {}", e),
    }

    let name = "Rope";
    let qty = 3;
    match shop.sell(name, qty) {
        Ok(gold) => println!("Sold {}x {} for {} gold", qty, name, gold),
        Err(e) => println!("Error: {}", e),
    }

    let name = "Dagger";
    let qty = 10;
    match shop.sell(name, qty) {
        Ok(gold) => println!("Sold {}x {} for {} gold", qty, name, gold),
        Err(e) => println!("Error: {}", e),
    }

    let name = "Arrows";
    let qty = 1;
    match shop.sell(name, qty) {
        Ok(gold) => println!("Sold {}x {} for {} gold", qty, name, gold),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nShop gold: {} gold", shop.gold);
}
