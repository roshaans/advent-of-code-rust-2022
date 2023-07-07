mod common;
use std::convert::From;

pub fn main() {
    let input = include_str!("../input.txt").to_string();
    let answer = calcualte_strategy_points(input.clone());
    println!("answer: {answer}");
}

enum Score {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<Move> for Score {
    fn from(s: Move) -> Self {
        match s {
            Move::Rock => Score::Rock,
            Move::Paper => Score::Paper,
            Move::Scissors => Score::Scissors,
        }
    }
}

#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone)]
enum Outcome {
    Win,
    Loss,
    Tie,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move"),
        }
    }
}

pub fn calcualte_strategy_points(input: String) -> u64 {
    input
        .split('\n')
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .filter(|x| x.len() == 2)
        .map(|x: Vec<&str>| calc_score_player_two(x[0], x[1]))
        .sum()
}

fn calc_score_player_two(player_one_move_str: &str, outcome: &str) -> u64 {
    let player_one_move = Move::from(player_one_move_str);

    let intended_outcome = match outcome {
        "X" => Outcome::Loss,
        "Y" => Outcome::Tie,
        "Z" => Outcome::Win,
        _ => panic!("Invalid outcome"),
    };

    let player_two_move = match (player_one_move, &intended_outcome) {
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Tie) => Move::Rock,
        (Move::Rock, Outcome::Loss) => Move::Scissors,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Tie) => Move::Paper,
        (Move::Paper, Outcome::Loss) => Move::Rock,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Tie) => Move::Scissors,
        (Move::Scissors, Outcome::Loss) => Move::Paper,
    };

    let player_two_score = match intended_outcome {
        Outcome::Loss => 0,
        Outcome::Win => 6,
        Outcome::Tie => 3,
    };
    println!("player_two_score: {:?}", player_two_score);
    let player_two_move_score = Score::from(player_two_move) as u64;
    println!("player_two_move_score: {:?}", player_two_move_score);
    player_two_move_score + player_two_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn calc_strategy() {
        let input = include_str!("../test.txt");
        println!("{:?}", input);
        let actual = calcualte_strategy_points(input.to_string());
        let answer = 12;
        assert_eq!(answer, actual)
    }

    #[test]
    fn lose_with_scissors() {
        let player_one_move = "C";
        let intended_outcome = "X";
        let actual = calc_score_player_two(player_one_move, intended_outcome);
        let answer = 2;
        assert_eq!(answer, actual)
    }

    #[test]
    fn tie_with_rock() {
        let player_one_move = "A";
        let intended_outcome = "Y";
        let actual = calc_score_player_two(player_one_move, intended_outcome);
        let answer = 4;
        assert_eq!(answer, actual)
    }

    #[test]
    fn win_with_rock() {
        let player_one_move = "A";
        let intended_outcome = "Z";
        let actual = calc_score_player_two(player_one_move, intended_outcome);
        let answer = 8;
        assert_eq!(answer, actual)
    }
}
