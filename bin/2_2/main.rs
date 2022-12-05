use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn check_outcome(player: char, opponent: char) -> i32 {
    let win_map: HashMap<char, char> = HashMap::from([
        ('A', 'Z'),
        ('B', 'X'),
        ('C', 'Y'),
        ('X', 'C'),
        ('Y', 'A'),
        ('Z', 'B'),
    ]);

    let lose_map: HashMap<char, char> = HashMap::from([
        ('Z', 'A'),
        ('X', 'B'),
        ('Y', 'C'),
        ('C', 'X'),
        ('A', 'Y'),
        ('B', 'Z'),
    ]);

    let tie_map: HashMap<char, char> = HashMap::from([
        ('A', 'X'),
        ('B', 'Y'),
        ('C', 'Z'),
        ('X', 'A'),
        ('Y', 'B'),
        ('Z', 'C'),
    ]);

    // X LOSE, Y TIE, Z WIN
    let desired_outcome_map: HashMap<char, char> =
        HashMap::from([('X', 'L'), ('Y', 'T'), ('Z', 'W')]);

    let choice_score_map: HashMap<char, i32> =
        HashMap::from([('A', 1), ('B', 2), ('C', 3), ('X', 1), ('Y', 2), ('Z', 3)]);

    let outcome_map: HashMap<char, i32> = HashMap::from([('W', 6), ('T', 3), ('L', 0)]);

    let desired_outcome = *desired_outcome_map.get(&player).unwrap();
    let mut choice: char = ' ';

    if desired_outcome == 'W' {
        choice = *lose_map.get(&opponent).unwrap();
    } else if desired_outcome == 'T' {
        choice = *tie_map.get(&opponent).unwrap();
    } else if desired_outcome == 'L' {
        choice = *win_map.get(&opponent).unwrap();
    }

    let choice_score: i32 = *choice_score_map.get(&choice).unwrap();

    return *outcome_map.get(&desired_outcome).unwrap() + choice_score;
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
