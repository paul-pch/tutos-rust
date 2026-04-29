fn main() {
    println!("Hello, world!");

    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);

    let x = 5;
    let _y = x;

    // Les types scalaires c'est ok.
    println!("{}", x);

    let s1 = String::from("Hello");
    let _len = calculate_len(&s1);

    fn calculate_len(x: &String) -> usize {
        x.len()
    }

    println!("{}", s1);

    fn add_world(s: &mut String) {
        s.push_str(" world");
    }

    let mut s = String::from("hello...");
    add_world(&mut s);

    println!("{}", s);

    // =====================

    println!("Exercice :");

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
