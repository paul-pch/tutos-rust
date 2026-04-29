fn main() {
    println!("Hello, world!");

    let x = 2;

    let y: i32 = "4".parse().unwrap();

    fn add(a: i32, b: i32) {
        println!("{}", a + b);
    }

    add(x, y);

    let point = (10, 20);
    let (_x, _y) = point;

    println!("{}", point.0);

    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[0]);
    println!("{}", arr.len());

    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    println!("{}", multiply(x, y));

    // ================================

    let name = "Alice";
    let score = 100;
    let niveau: u8 = 1;
    let player: (&str, i32, u8) = (name, score, niveau);

    let bonuses: [i32; 3] = [5, 7, 3];

    fn total_score(score: i32, bonuses: [i32; 3]) -> i32 {
        score + bonuses.iter().sum::<i32>()
    }

    println!("{}", total_score(player.1, bonuses));
}
