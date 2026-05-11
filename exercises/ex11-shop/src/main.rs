use std::fmt;

#[derive(PartialEq)]
enum ItemCategory {
    Armor,
    Consumable,
    Misc,
    Weapon,
}

impl fmt::Display for ItemCategory  {
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
    quantity: u32
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} | {:1}kg | {} gold | x{}", self.category.to_string(), self.name, self.weight, self.value, self.quantity)
    }
}

impl Item {
    fn new(name: String, category: ItemCategory, weight: f32, value: u32, quantity: u32) -> Self {
        Item { name, category, weight, value, quantity }
    }
}

enum ShopError {
    ItemNotFound(String),
    NotEnoughStock { item: String, have: u32, want: u32}
}

impl fmt::Display for ShopError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopError::ItemNotFound(s) => write!(f, "item not found: {}", s),
            ShopError::NotEnoughStock { item, have, want } => write!(f, "not enough stock for {} (have {}, want {}", item, have, want),
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
        Shop { name, inventory, gold }
    }

    fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    fn total_weight(&self) -> f32 {
        self.inventory.iter()
            .map(|item| item.weight)
            .sum()
    }

    fn total_value(&self) -> u32 {
        self.inventory.iter()
            .map(|item| item.value)
            .sum()
    }

    fn most_valuable(&self) -> Option<&Item> {
        self.inventory.iter()
            .max_by_key(|item| item.value)
    }

    fn count_by_category(&self, cat: &ItemCategory) -> usize {
        self.inventory.iter()
            .filter(|item| item.category == *cat)
            .count()
    }

    fn sell(&mut self, name: &str, qty: u32) -> Result<u32, ShopError> {
        let index = self.inventory.iter()
            .find(|item| item.name == name)
            .get
    }
}






fn main() {
    let sword = Item::new(String::from("Sword"), ItemCategory::Weapon, 5.5, 120, 2);
    println!("{}", ItemCategory::Armor.to_string());
    println!("{}", sword.to_string());
}
