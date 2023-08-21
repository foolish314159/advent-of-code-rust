use std::fs::read_to_string;

/// Return the total points of the rock paper scissors tournament if you are following
/// the given set of moves.
///
/// Points for each round are awarded as following:
/// Win = 6 points, Draw = 3 points, Loss = 0 points
/// Rock = 1 point, Paper = 2 points, Scissors = 3 points
///
/// # Arguments
///
/// * `filename` - Path to input file containing a list of rock paper scissors moves.
///                First column is what your opponent is (apparently) going to play:
///                A = Rock, B = Paper, C = Scissors
///
///                (Part 1)
///                Second column is what player 2 (you) is supposed to play:
///                X = Rock, Y = Paper, Z = Scissors
///
///                (Part 2)
///                Second column is the desired outcome for player 2 (you):
///                X = Loss, Y = Draw, Z = Win
///
/// * `part1`    - Treat second column like (Part 1) or (Part 2)
pub fn total_points(filename: &str, part1: bool) -> u32 {
    match part1 {
        true => parse_rounds_part1(filename),
        false => parse_rounds_part2(filename),
    }
    .iter()
    .fold(0u32, |points, round| points + round.points())
}

fn parse_rounds_part1(filename: &str) -> Vec<Round> {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    content.lines().fold(vec![], |mut rounds, line| {
        let mut shapes = line.split_whitespace();
        if shapes.clone().count() == 2 {
            if let (Some(str1), Some(str2)) = (shapes.next(), shapes.next()) {
                rounds.push(Round {
                    p1_shape: Shape::from_str(str1),
                    p2_shape: Shape::from_str(str2),
                });
            }
        }
        rounds
    })
}

fn parse_rounds_part2(filename: &str) -> Vec<Round> {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    content.lines().fold(vec![], |mut rounds, line| {
        let mut shapes = line.split_whitespace();
        if shapes.clone().count() == 2 {
            if let (Some(str1), Some(str2)) = (shapes.next(), shapes.next()) {
                let p1_shape = Shape::from_str(str1);
                let p2_shape = p1_shape.for_outcome(&Outcome::from_str(str2));
                rounds.push(Round { p1_shape, p2_shape });
            }
        }
        rounds
    })
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
    fn from_str(str: &str) -> Outcome {
        match str {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic! {"Invalid outcome string {str}"},
        }
    }
}

impl Shape {
    fn vs(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Rock, Shape::Paper) => Outcome::Loss,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Scissors) => Outcome::Loss,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Loss,
            (_, _) => Outcome::Draw,
        }
    }
    fn points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
    fn from_str(str: &str) -> Shape {
        match str {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic! {"Invalid shape string {str}"},
        }
    }
    /// Returns the shape the other player has to choose for desired outcome
    /// E.g. Rock.for_outcome(Win) will return Paper
    fn for_outcome(&self, outcome: &Outcome) -> Shape {
        match (self, outcome) {
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Rock, Outcome::Loss) => Shape::Scissors,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Paper, Outcome::Loss) => Shape::Rock,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
            (Shape::Scissors, Outcome::Loss) => Shape::Paper,
            (_, _) => *self,
        }
    }
}

struct Round {
    p1_shape: Shape,
    p2_shape: Shape,
}

impl Round {
    /// Points player 2 (you) will get for this round
    fn points(&self) -> u32 {
        self.p2_shape.vs(&self.p1_shape).points() + self.p2_shape.points()
    }
}
