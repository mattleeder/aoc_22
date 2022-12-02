use common::utils;

enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Eq, Hash, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    player: Choice,
    opponent: Choice,
    outcome: Outcome,
}

impl Round {
    fn new(s: &str) -> Result<Round, &'static str> {
        let char_vec: Vec<char> = s.chars().collect();
        if char_vec.len() != 3 {
            return Err("Expected 3 chars")
        }
        let opponent = match char_vec[0] {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => return Err("Unexpected char for opponent")
        };
        let player = match char_vec[2] {
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => return Err("Unexpected char for player")
        };

        let outcome: Outcome = Round::outcome(&player, &opponent);

        Ok(Round { player, opponent, outcome })
    }

    fn outcome(player: &Choice, opponent: &Choice) -> Outcome {
        let res = match player {
            Choice::Rock => {
                match opponent {
                    Choice::Rock => Outcome::Draw,
                    Choice::Paper => Outcome::Lose,
                    Choice::Scissors => Outcome::Win,
                }
            },
            Choice::Paper => {
                match opponent {
                    Choice::Rock => Outcome::Win,
                    Choice::Paper => Outcome::Draw,
                    Choice::Scissors => Outcome::Lose,
                }
            },
            Choice::Scissors => {
                match opponent {
                    Choice::Rock => Outcome::Lose,
                    Choice::Paper => Outcome::Win,
                    Choice::Scissors => Outcome::Draw,
                }
            },
        };

        res
    }

    fn get_score(&self) -> i64 {
        let mut score = 0;

        match self.player {
            Choice::Rock => score += 1,
            Choice::Paper => score += 2,
            Choice::Scissors => score += 3,
        }

        match self.outcome {
            Outcome::Win => score += 6,
            Outcome::Draw => score += 3,
            Outcome::Lose => score += 0,
        }

        score
    }
}

fn main() {
    let contents = utils::read_file().unwrap();
    let mut total_score: i64 = 0;

    for line in contents.lines() {
        let round = Round::new(line);
        total_score += round.unwrap().get_score();
    }

    println!("{}", total_score);
}
