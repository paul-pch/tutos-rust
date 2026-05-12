fn main() {
    let _score: i32 = 10;
    let mut lives: u8 = 3;

    lives -= 1;

    let score = String::from("Score: 10");

    println!("{}", score);
    println!("Lives: {}", lives);
}
