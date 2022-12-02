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
        let outcome = match char_vec[2] {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => return Err("Unexpected char for player")
        };

        let player: Choice = Round::player_choice(&outcome, &opponent);

        Ok(Round { player, opponent, outcome })
    }

    fn player_choice(outcome: &Outcome, opponent: &Choice) -> Choice {
        let res = match outcome {
            Outcome::Lose => {
                match opponent {
                    Choice::Rock => Choice::Scissors,
                    Choice::Paper => Choice::Rock,
                    Choice::Scissors => Choice::Paper,
                }
            },
            Outcome::Draw => {
                match opponent {
                    Choice::Rock => Choice::Rock,
                    Choice::Paper => Choice::Paper,
                    Choice::Scissors => Choice::Scissors,
                }
            },
            Outcome::Win => {
                match opponent {
                    Choice::Rock => Choice::Paper,
                    Choice::Paper => Choice::Scissors,
                    Choice::Scissors => Choice::Rock,
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
