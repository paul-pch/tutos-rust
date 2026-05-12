fn main() {
    let mut name = String::from("Paul");

    fn greet(name: &String) {
        println!("Hello, {}!", name);
    }

    greet(&name);
    println!("{}", name);

    fn shout(name: &mut String) {
        *name = name.to_uppercase();
    }

    shout(&mut name);

    println!("{}", name);
}
