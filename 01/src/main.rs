mod common;
use itertools::Itertools;

pub fn main() {
    // let content = common::get_file_contents("inputs/test.txt".to_string()d).unwrap();
    let input = include_str!("../input.txt").to_string();
    let answer = calculate_max_group(input.clone());
    println!("answer: {answer}");

    let answer_top_3 = calculate_top_3_max(input.clone());

    println!("answer top 3: {answer_top_3}");
}

pub fn calculate_max_group(input: String) -> u64 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
}

pub fn calculate_top_3_max(input: String) -> u64 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn creates_of_cal_counts() {
        let input = include_str!("../test.txt");
        let actual = calculate_max_group(input.to_string());
        let answer = 24000;
        assert_eq!(answer, actual)
    }

    #[test]
    pub fn calculate_top_3() {
        let input = include_str!("../test.txt");
        let actual = calculate_top_3_max(input.to_string());
        let answer = 45000;
        assert_eq!(answer, actual)
    }
}
