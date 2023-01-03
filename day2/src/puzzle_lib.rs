use std::error::Error;
use std::fs;

use day2::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("FilePath: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
    //println!("Contents {contents}");

    let mut rounds: Vec<Round> = Vec::new();
    let lines = contents.lines();
    let mut rnd_number = 1;

    for s in lines {
        let rnd = Round::build_from_strat_line(&s).unwrap();
        rounds.push(rnd);
        rnd_number = rnd_number + 1;
    }

    let mut total = 0;
    for r in rounds {
        total = total + r.round_score();
    }

    println!("Final Score: {}", total);

    Ok(())
}

#[derive(Debug)]
enum Shape {
    ROCK,
    PAPER,
    SCISSORS,
}


#[derive(Debug)]
enum RoundResult {
    IWon,
    ILost,
    TIE,
}

#[derive(Debug)]
struct Round {
    //round_number: i32,
    opponent_choice: Shape,
    my_choice: Shape,
}

impl Round {
    pub fn build_from_strat_line(strat_line: &str) -> Result<Round, &'static str> {
        let v: Vec<&str> = strat_line.split_whitespace().collect();

        let o_choice = Round::letter_to_shape(v[0].clone());
        let o2_choice = Round::letter_to_shape(v[0].clone());
        let m_choice = Round::result_to_shape(v[1], o2_choice);

        Round::build(o_choice, m_choice)
    }

    pub fn build(
        //round_number: i32,
        op_choice: Shape,
        my_choice: Shape,
    ) -> Result<Round, &'static str> {
        Ok(Round {
            //round_number: round_number,
            opponent_choice: op_choice,
            my_choice: my_choice,
        })
    }

    fn letter_to_shape(letter_code: &str) -> Shape {
        match letter_code {
            "A" => return Shape::ROCK,
            "B" => return Shape::PAPER,
            "C" => return Shape::SCISSORS,
            _ => panic!("Unknown shape code"),
        }
    }

    fn result_to_shape(letter_code: &str, op_shape: Shape) -> Shape {
        match letter_code {
            // Loose
            "X" => {
                match op_shape {
                    Shape::ROCK => return Shape::SCISSORS,
                    Shape::PAPER => return Shape::ROCK,
                    Shape::SCISSORS => return Shape::PAPER,
                }
            },
            // draw
            "Y" => {
                match op_shape {
                    Shape::ROCK => return Shape::ROCK,
                    Shape::PAPER => return Shape::PAPER,
                    Shape::SCISSORS => return Shape::SCISSORS,
                }

            },
            // win
            "Z" => {
                match op_shape {
                    Shape::ROCK => return Shape::PAPER,
                    Shape::PAPER => return Shape::SCISSORS,
                    Shape::SCISSORS => return Shape::ROCK,
                }
            },
            _ => panic!("Invalid win loss code letter")
        }
    }

    //fn debug(&self) {
    //    print!("{:#?} / result {:?} / score: {}", self, self.round_result(), self.round_score());
    //}

    fn shape_score(&self) -> i32 {
        match self.my_choice {
            Shape::ROCK => 1,
            Shape:: PAPER => 2,
            Shape::SCISSORS => 3
        }
    }

    fn outcome_score(res: RoundResult) -> i32 {
        match res {
            RoundResult::TIE => 3,
            RoundResult::ILost => 0,
            RoundResult::IWon => 6,
        }
    }

    fn round_score(&self) -> i32 {
        let mut total = 0;
        total = total + self.shape_score();
        total = total + Round::outcome_score(self.round_result());

        return total;
    }



    fn round_result(&self) -> RoundResult {
        match self.my_choice {
            Shape::ROCK => {
                return match self.opponent_choice {
                    Shape::ROCK => RoundResult::TIE,
                    Shape::PAPER => RoundResult::ILost,
                    Shape::SCISSORS => RoundResult::IWon,
                }
            }
            Shape::PAPER => {
                return match self.opponent_choice {
                    Shape::ROCK => RoundResult::IWon,
                    Shape::PAPER => RoundResult::TIE,
                    Shape::SCISSORS => RoundResult::ILost,
                }
            }
            Shape::SCISSORS => {
                return match self.opponent_choice {
                    Shape::ROCK => RoundResult::ILost,
                    Shape::PAPER => RoundResult::IWon,
                    Shape::SCISSORS => RoundResult::TIE,
                }
            }
        }
    }
}
