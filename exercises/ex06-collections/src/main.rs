use std::collections::HashMap;

fn main() {
    #[derive(Clone)]
    enum Rarity {
        Common,
        Rare,
        Legendary,
    }

    #[derive(Clone)]
    struct Item {
        name: String,
        rarity: Rarity,
        value: u32,
    }

    impl Item {
        fn new(name: String, rarity: Rarity, value: u32) -> Self {
            Item {
                name,
                rarity,
                value,
            }
        }

        fn display(&self) -> String {
            match self.rarity {
                Rarity::Common => format!("{} (Common) - {} gold", self.name, self.value),
                Rarity::Rare => format!("{} (Rare) - {} gold", self.name, self.value),
                Rarity::Legendary => format!("{} (Legendary) - {} gold", self.name, self.value),
            }
        }
    }

    let mut total_value: u32 = 0;

    println!("=== Inventaire ===");
    let mut inventaire: Vec<Item> = Vec::new();
    inventaire.push(Item::new(String::from("Épée de fer"), Rarity::Common, 10));
    inventaire.push(Item::new(String::from("Potion de soin"), Rarity::Common, 5));
    inventaire.push(Item::new(
        String::from("Bouclier d'acier"),
        Rarity::Rare,
        50,
    ));
    inventaire.push(Item::new(
        String::from("Amulette dragon"),
        Rarity::Legendary,
        500,
    ));

    for item in inventaire.iter() {
        println!("{}", item.display());
        total_value += item.value;
    }

    println!("");
    println!("=== Par catégorie ===");
    let mut par_categorie: HashMap<String, Vec<Item>> = HashMap::new();
    par_categorie.insert(String::from("Armes"), vec![inventaire[0].clone()]);
    par_categorie.insert(String::from("Armures"), vec![inventaire[2].clone()]);
    par_categorie.insert(
        String::from("Potions"),
        vec![inventaire[1].clone(), inventaire[3].clone()],
    );

    // Proper way
    // for categorie in par_categorie.iter()  {
    //     println!("{} :", categorie.0);
    //     for item in categorie.1.iter()  {
    //         println!("  {}", item.display());
    //     }
    // }

    if let Some(items) = par_categorie.get("Armes") {
        println!("Armes :");
        for item in items {
            println!("  {}", item.display());
        }
    }
    if let Some(items) = par_categorie.get("Armures") {
        println!("Armures :");
        for item in items {
            println!("  {}", item.display());
        }
    }
    if let Some(items) = par_categorie.get("Potions") {
        println!("Potions :");
        for item in items {
            println!("  {}", item.display());
        }
    }

    println!("");
    println!("Valeur totale de l'inventaire : {} gold", total_value);
}
