use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn base_score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn parse(c: &char) -> Shape {
        match c {
            'A'|'X' => Shape::Rock,
            'B'|'Y' => Shape::Paper,
            'C'|'Z' => Shape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }
}

#[derive(PartialEq)]
enum Strategy {
    Win,
    Lose,
    Draw,
}

impl Strategy {
    fn parse(c: &char) -> Strategy {
        match c {
            'X' => Strategy::Lose,
            'Y' => Strategy::Draw,
            'Z' => Strategy::Win,
            _ => panic!("Invalid strategy"),
        }
    }
}


struct Round {
    opponent: Shape,
    me: Shape,
}

impl Round {
    fn score(&self) -> i32 {
        let base = self.me.base_score();
        if self.opponent == self.me {
            base + 3
        } else if self.opponent == Shape::Rock && self.me == Shape::Scissors ||
                self.opponent == Shape::Paper && self.me == Shape::Rock ||
                self.opponent == Shape::Scissors && self.me == Shape::Paper  {
            base
        } else {
            base + 6
        }
    }
}

fn assign_shape_from_strat(opponents_shape: Shape, strat: Strategy) -> Shape {
    match strat {
        Strategy::Draw => opponents_shape,
        Strategy::Win => {
            match opponents_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            }
        }
        Strategy::Lose => {
            match opponents_shape{
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            }
        }
    }
}

fn main() {
    let input:Vec<(char, char)> = fs::read_to_string("src/day02/input.txt")
        .unwrap()
        .lines()
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .collect();

    let part_1_rounds = input
        .iter()
        .map(|(opponent, me)| Round {
            opponent: Shape::parse(&opponent),
            me: Shape::parse(&me),
        });

    let part_1_score: i32 = part_1_rounds
        .map(|round| round.score())
        .sum();

    println!("Part 1");
    println!("Score: {}", part_1_score);

    let part_2_rounds = input
        .iter()
        .map(|(opponent, me)| {
            let opponent_shape = Shape::parse(&opponent); 
            let my_strategy = Strategy::parse(&me);
            Round {
                opponent: opponent_shape,
                me: assign_shape_from_strat(opponent_shape, my_strategy),
            }
        });

    let part_2_score:i32 = part_2_rounds
        .map(|round| round.score())
        .sum();

    println!("Part 2");
    println!("Score: {}", part_2_score);
}