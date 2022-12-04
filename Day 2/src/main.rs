use std::fs;
use std::io::{BufReader, BufRead};

#[derive(Debug, PartialEq)]
enum Sign {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
struct Guide {
    enemy: Sign,
    my: Sign,
}

fn parse_data() -> Vec<Guide> {
    let filename = "src/rawdata.txt";
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut guides: Vec<Guide> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors because I'm lazy
        let mut data = line.split_whitespace();
        if data.clone().count() == 2 { // has to be cloned lest it be consumed
            let enemy = data.next().unwrap();
            let my = data.next().unwrap();
            let enemy = match enemy {
                "A" => Sign::Rock,
                "B" => Sign::Paper,
                "C" => Sign::Scissors,
                _ => panic!("Invalid enemy sign"),
            };
            let my = match my {
                "X" => Sign::Rock,
                "Y" => Sign::Paper,
                "Z" => Sign::Scissors,
                _ => panic!("Invalid my sign"),
            };
            let guide = Guide { enemy, my };
            guides.push(guide);
        }
    }

    return guides;
}

// determine who wins in rock paper scissors and calculate the round score
fn eval_round(guide: Guide) -> i32 {
    let enemy = guide.enemy;
    let my = guide.my;

    let mut score = 0;
    if enemy == Sign::Rock {
        if my == Sign::Paper {
            score = 6;
        } else if my == Sign::Rock {
            score = 3;
        }
    } else if enemy == Sign::Paper {
        if my == Sign::Scissors {
            score = 6;
        } else if my == Sign::Paper {
            score = 3;
        }
    } else if enemy == Sign::Scissors {
        if my == Sign::Rock {
            score = 6;
        } else if my == Sign::Scissors {
            score = 3;
        }
    }
    match my {
        Sign::Rock => score += 1,
        Sign::Paper => score += 2,
        Sign::Scissors => score += 3,
    }

    return score;
}

fn main() {
    let guides = parse_data();

    let mut total_score = 0;
    for guide in guides {
        total_score += eval_round(guide);
    }

    println!("Total score: {}", total_score);
    
}
