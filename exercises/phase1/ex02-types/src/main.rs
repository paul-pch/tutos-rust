fn main() {
    let name = "Alice";
    let score = 100;
    let niveau: u8 = 1;
    let player: (&str, i32, u8) = (name, score, niveau);

    let bonuses: [i32; 3] = [5, 7, 3];

    fn total_score(score: i32, bonuses: [i32; 3]) -> i32 {
        score + bonuses.iter().sum::<i32>()
    }
    println!("Player: {}", player.0);
    println!("Total score: {}", total_score(player.1, bonuses));
}
