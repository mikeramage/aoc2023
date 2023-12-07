use crate::utils;
use regex::Regex;
use std::collections::HashSet;

///Day 1 solution
pub fn day4() -> (usize, usize) {
    let scratchcards: Vec<String> = utils::parse_input("input/day4.txt");
    let re_scratchcard = Regex::new(r"Card\s+\d+:(?<winning>[\s\d]+)\|(?<mine>[\s\d]+)").unwrap();
    let mut num_matches_and_count_list: Vec<(usize, usize)> = scratchcards
        .iter()
        .map(|scratchcard| {
            let captures = re_scratchcard.captures(scratchcard).unwrap();
            let winning: HashSet<_> = captures["winning"]
                .split_whitespace()
                .map(|digit_string| digit_string.parse::<usize>().unwrap())
                .collect();
            let mine: HashSet<_> = captures["mine"]
                .split_whitespace()
                .map(|digit_string| digit_string.parse::<usize>().unwrap())
                .collect();
            (winning.intersection(&mine).count(), 1)
        })
        .collect();

    for i in 0..num_matches_and_count_list.len() {
        let number = num_matches_and_count_list[i].0;
        let count = num_matches_and_count_list[i].1;
        if number > 0 {
            for j in 0..number {
                num_matches_and_count_list[i + j + 1] = (
                    num_matches_and_count_list[i + j + 1].0,
                    num_matches_and_count_list[i + j + 1].1 + count,
                );
            }
        }
    }

    (
        num_matches_and_count_list
            .iter()
            .map(|(number, _count)| {
                let base: usize = 2;
                match number {
                    0 => 0,
                    anything_else => base.pow(*anything_else as u32 - 1),
                }
            })
            .sum(),
        num_matches_and_count_list
            .iter()
            .map(|(_number, count)| count)
            .sum(),
    )
}
