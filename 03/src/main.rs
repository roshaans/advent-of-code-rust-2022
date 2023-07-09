mod common;

pub fn main() {
    // let input = include_str!("../input.txt").to_string();
    // let answer = find_common_items(input.clone());
    // println!("answer: {answer}");
}

pub fn calc_score(c: char) -> u32 {
    let score = match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("hello"),
    };
    eprintln!("score for {}: {}", c, score);
    score
}

pub fn find_common_items(rucksack: String) -> u32 {
    let mut counts = vec![0; 52];

    for n in 0..rucksack.len() / 2 {
        let char = rucksack.as_bytes()[n] as char;
        let char_score = calc_score(char);
        // println!("char_score: {char_score}");
        counts[char_score as usize] += 1;
        // total_score += char_score;
        // println!("total_score: {total_score}");
    }

    let mut total_score = 0;
    for n in (rucksack.len() / 2)..rucksack.len() {
        let char = rucksack.as_bytes()[n] as char;
        let char_score = calc_score(char);
        match counts.get(char_score as usize) {
            Some(_) => total_score += char_score,
            None => counts[char_score as usize] += 1,
        };
        // println!("char_score: {char_score}");
        counts[char_score as usize] += 1;
    }
    eprintln!("{:#?}", counts.clone());
    total_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score_conversion_uppercase() {
        assert_eq!(calc_score('A'), 27);
        assert_eq!(calc_score('p'), 16);
        // assert_eq!(calc_score('B'), 28);
        // assert_eq!(calc_score('Z'), 52);
    }

    //
    // #[test]
    // fn score_conversion_lower() {
    //     assert_eq!(calc_score('a'), 1);
    //     assert_eq!(calc_score('b'), 2);
    //     assert_eq!(calc_score('z'), 26);
    // }

    #[test]
    fn ruddersack_contains_one_common_item() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let actual = find_common_items(rucksack.to_string());
        assert_eq!(actual, 16)
    }

    // #[test]
    // fn ruddersack_contain_two_common_items() {
    //     let rucksack = "vJLrwpWtwJgWrhcsFMMfFLFhFp";
    //     let actual = find_common_items(rucksack.to_string());
    //     assert_eq!(actual, 16 + 38)
    // }
    // #[test]
    // fn ruddersack_contain_no_common_items() {
    //     let rucksack = "vJrwWtwJgWrhcsFMMfFFhF";
    //     let actual = find_common_items(rucksack.to_string());
    //     assert_eq!(actual, 0)
    // }
}
