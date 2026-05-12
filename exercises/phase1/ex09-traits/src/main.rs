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
    }

    struct Player {
        name: String,
        health: u32,
    }

    impl Player {
        fn new(name: String, health: u32) -> Self {
            Player { name, health }
        }
    }

    trait Summarize {
        fn summarize(&self) -> String;
        fn print_summary(&self) {
            println!("{}", self.summarize())
        }
    }

    impl Summarize for Player {
        fn summarize(&self) -> String {
            format!("[Player] {} — {} HP", self.name, self.health)
        }
    }

    impl Summarize for Item {
        fn summarize(&self) -> String {
            format!(
                "[Item] {} — {:1}kg, {} gold",
                self.name, self.weight, self.value
            )
        }
    }

    fn print_all(summarizables: &[impl Summarize]) {
        for summarizable in summarizables {
            summarizable.print_summary();
        }
    }

    let arr = [
        Item::new(String::from("Sword"), 5.5, 120),
        Item::new(String::from("Potion"), 0.5, 30),
    ];
    let arr2 = [Player::new(String::from("Alice"), 100)];

    print_all(&arr);
    print_all(&arr2);
}
