use std::error::Error;
use std::fs;

pub fn check_outcome(player: char, opponent: char) -> i32 {
    let outcome_score: i32 = match (opponent, player) {
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        ('C', 'X') | ('A', 'Y') | ('B', 'Z') => 6,
        _ => 3,
    };

    let choice_score: i32 = match player {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    return outcome_score + choice_score;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?.parse()?;

    let mut score: i32 = 0;

    for play in input.lines() {
        let opponent: char;
        let player: char;
        let moves: Vec<char> = play
            .split(" ")
            .map(|x| x.parse::<char>().unwrap())
            .collect::<Vec<char>>();

        opponent = moves[0];
        player = moves[1];

        let outcome: i32 = check_outcome(player, opponent);

        score += outcome;
    }

    println!("{score}");
    Ok(())
}
