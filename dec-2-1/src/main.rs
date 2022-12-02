use std::env;
use std::fs;
use std::fmt;
use std::cmp;
use std::cmp::Ordering;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect("Cannot find file {}");

    //println!("{contents}");

    let mut total = 0;
    for line in contents.lines() {

        let round = Round::next(&line);

        println!("Elf played {}, I played {}. Score for the round is: {}", round.elf, round.me, round.score());
        total += round.score();
    }

    println!("My total score for the tournament is: {}", total);
    //let rps3 = RPS::elf_rps('X');
    //let rps4 = RPS::me_rps('A');

}

#[derive(PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn elf_rps(input: char) -> RPS {
        if input == 'A' { 
            return RPS::Rock
        } else if input == 'B'{
            return RPS::Paper
        } else if input == 'C' {
            return RPS::Scissors
        } else {
            panic!("Invalid Input for Elf.")
        }
    }

    // used in problem 1, removed in problem 2
    /* fn me_rps(input: char) -> RPS {
        if input == 'X' { 
            return RPS::Rock
        } else if input == 'Y' {
            return RPS::Paper
        } else if input == 'Z' {
            return RPS::Scissors
        } else {
            panic!("Invalid Input for me.")
        }
    }
 */
    fn score(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }
}

impl std::fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RPS::Rock => write!(f, "Rock"),
            RPS::Scissors => write!(f, "Scissors"),
            RPS::Paper => write!(f, "Paper")
        }
    }
}

impl cmp::PartialOrd for RPS {
    fn partial_cmp(&self, other: &RPS) -> Option<Ordering>
    {
        match self {
            RPS::Rock => {
                match other {
                    //rock ties with rock
                    RPS::Rock => Some(Ordering::Equal),
                    //rock breaks scissors
                    RPS::Scissors => Some(Ordering::Greater),
                    //paper wraps rock
                    RPS::Paper => Some(Ordering::Less)
                }
            },
            RPS::Paper => {
                match other {
                    //paper ties with paper
                    RPS::Paper => Some(Ordering::Equal),
                    //paper wraps rock
                    RPS::Rock => Some(Ordering::Greater),
                    //Scissors cut paper
                    RPS::Scissors => Some(Ordering::Less)
                }
            },
            RPS::Scissors => {
                match other {
                    //scissors ties with scissors
                    RPS::Scissors => Some(Ordering::Equal),
                    //scissors cuts paper
                    RPS::Paper => Some(Ordering::Greater),
                    //rock breaks scissors
                    RPS::Rock => Some(Ordering::Less)
                }
            }
        }
    }
}

//introduced in Problem 2
enum Outcome {
    Win,
    Draw,
    Lose
}

impl Outcome {
    fn new (key: char) -> Outcome {
        if key == 'X' { 
            return Outcome::Lose
        } else if key == 'Y' {
            return Outcome::Draw
        } else if key == 'Z' {
            return Outcome::Win
        } else {
            panic!("Invalid Input for Outcome.")
        }
    }
}

struct Round {
    elf: RPS,
    me: RPS
}

impl Round {
    //problem 1 version
    /* fn next (line: &str) -> Round {
        Round {
            elf: RPS::elf_rps(line.chars().nth(0).unwrap()),
            me: RPS::me_rps(line.chars().nth(2).unwrap())
        }
    } */

    //problem 2 version
    fn next (line: &str) -> Round {
        let elf_play = RPS::elf_rps(line.chars().nth(0).unwrap());
        let outcome = Outcome::new(line.chars().nth(2).unwrap());
        let me_play = Round::get_my_play(&elf_play, &outcome);
        Round { elf: elf_play, me: me_play}
    }

    fn get_my_play(elf_play: &RPS, outcome: &Outcome) -> RPS {
        match outcome {
            Outcome::Draw => {
                match elf_play {
                    RPS::Rock => RPS::Rock,
                    RPS::Paper => RPS::Paper,
                    RPS::Scissors => RPS::Scissors
                }
            }
            Outcome::Lose => {
                match elf_play {
                    RPS::Rock => RPS::Scissors,
                    RPS::Paper => RPS::Rock,
                    RPS::Scissors => RPS::Paper
                }
            }
            Outcome::Win => {
                match elf_play {
                    RPS::Rock => RPS::Paper,
                    RPS::Paper => RPS::Scissors,
                    RPS::Scissors => RPS::Rock
                }
            }
        }
    }

    fn score (&self) -> i32 {
        let me_score = self.me.score();

        if self.me > self.elf {
            return me_score + 6;
        } else if self.me == self.elf {
            return me_score + 3;
        } else {
            return me_score;
        }
    }
}
