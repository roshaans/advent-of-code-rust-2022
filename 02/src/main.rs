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

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid move"),
        }
    }
}

pub fn calcualte_strategy_points(input: String) -> u64 {
    input
        .split('\n')
        .map(|line| {
            let whit_space = line.split_whitespace().collect::<Vec<&str>>();

            println!("whit_space: {:?}", whit_space);
            whit_space
        })
        .filter(|x| x.len() == 2)
        .map(|x: Vec<&str>| {
            let score = calc_score_player_two(x[0], x[1]);
            println!("score: {:?}", score);
            score
        })
        .sum()
}

fn calc_score_player_two(player_one_move_str: &str, player_two_move_str: &str) -> u64 {
    let player_one_move = Move::from(player_one_move_str);
    let player_two_move = Move::from(player_two_move_str);

    let outcome = match (player_one_move, player_two_move.clone()) {
        (Move::Rock, Move::Paper) => (0, 1),
        (Move::Rock, Move::Scissors) => (1, 0),
        (Move::Rock, Move::Rock) => (1, 1),
        (Move::Paper, Move::Rock) => (1, 0),
        (Move::Paper, Move::Scissors) => (0, 1),
        (Move::Paper, Move::Paper) => (1, 1),
        (Move::Scissors, Move::Rock) => (0, 1),
        (Move::Scissors, Move::Paper) => (1, 0),
        (Move::Scissors, Move::Scissors) => (1, 1),
    };

    let (player_one_outcome, player_two_outcome) = outcome;

    let player_two_score = match (player_one_outcome, player_two_outcome) {
        (1, 1) => 3,
        (1, 0) => 0,
        (0, 1) => 6,
        _ => panic!("Invalid outcome"),
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
        let answer = 15;
        assert_eq!(answer, actual)
    }

    #[test]
    fn rock_beats_scissors() {
        let player_one_move = "C";
        let player_two_move = "X";
        let actual = calc_score_player_two(player_one_move, player_two_move);
        let answer = 1 + 6;
        assert_eq!(answer, actual)
    }

    #[test]
    fn paper_beats_rock() {
        let player_one_move = "A";
        let player_two_move = "Y";
        let actual = calc_score_player_two(player_one_move, player_two_move);
        let answer = 8;
        assert_eq!(answer, actual)
    }

    #[test]
    fn tie_score() {
        let player_one_move = "A";
        let player_two_move = "X";
        let actual = calc_score_player_two(player_one_move, player_two_move);
        let answer = 4;
        assert_eq!(answer, actual)
    }
}
