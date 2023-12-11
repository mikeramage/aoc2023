use itertools::Itertools;

use crate::utils;
use core::cmp::Ordering;
use itertools::enumerate;
use std::collections::HashMap;

///Day 7 solution
pub fn day7() -> (usize, usize) {
    let input: Vec<String> = utils::parse_input("input/day7.txt");
    let handmap: HashMap<String, usize> = input
        .iter()
        .map(|x| {
            let handbid = x.split_whitespace().collect::<Vec<_>>();
            let hand: String = handbid[0].to_string();
            let bid: usize = handbid[1].parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect::<HashMap<String, usize>>();

    let sorted_keys = handmap.keys().sorted_by(|a, b| compare_hands(a, b, false));
    let sorted_keys_alt = handmap.keys().sorted_by(|a, b| compare_hands(a, b, true));

    let mut winnings: usize = 0;
    for (i, key) in enumerate(sorted_keys) {
        winnings += (i + 1) * handmap.get(key).unwrap();
    }

    let mut winnings_alt: usize = 0;
    for (i, key) in enumerate(sorted_keys_alt) {
        winnings_alt += (i + 1) * handmap.get(key).unwrap();
    }

    (winnings, winnings_alt)
}

fn compare_hands(a: &str, b: &str, alt: bool) -> Ordering {
    let mut ordering = hand_type_strength(a, alt).cmp(&hand_type_strength(b, alt));
    if ordering == Ordering::Equal {
        //Need to compare cards in order
        for i in 0..a.len() {
            ordering = card_val(a.chars().nth(i).unwrap(), alt)
                .cmp(&card_val(b.chars().nth(i).unwrap(), alt));
            if ordering != Ordering::Equal {
                break;
            }
        }
    }
    ordering
}

fn hand_type_strength(hand: &str, alt: bool) -> usize {
    // 5 of a kind = 6
    // 4 of a kind = 5
    // full house = 4
    // three of a kind = 3
    // 2 pair = 2
    // pair = 1
    // high card = 0
    let mut counts: HashMap<char, usize> = HashMap::new();
    for char in hand.chars() {
        counts
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let num_jacks: usize = match counts.get(&'J') {
        Some(x) => *x,
        None => 0,
    };

    match counts.values().max().unwrap() {
        5 => 6, //five of a kind
        4 => {
            if alt {
                match num_jacks {
                    0 => 5,
                    1 | 4 => 6,
                    _ => unreachable!("Oops"),
                }
            } else {
                5
            }
        } //four of a kind
        3 => {
            //full house or three of a kind
            if counts.keys().len() == 2 {
                //Full house
                if alt {
                    match num_jacks {
                        0 => 4,
                        2 | 3 => 6,
                        _ => unreachable!("By gum"),
                    }
                } else {
                    4
                }
            } else {
                // three of a kind
                assert!(counts.keys().len() == 3);
                if alt {
                    match num_jacks {
                        0 => 3,
                        1 | 3 => 5,
                        _ => unreachable!("Load o' nonsense!"),
                    }
                } else {
                    3
                }
            }
        }
        2 => {
            //pair or two pairs
            if counts.keys().len() == 3 {
                //Two pairs
                if alt {
                    match num_jacks {
                        0 => 2,
                        1 => 4,
                        2 => 5,
                        _ => unreachable!("Ey up!"),
                    }
                } else {
                    2
                }
            } else {
                // pair
                assert!(counts.keys().len() == 4);
                if alt {
                    match num_jacks {
                        0 => 1,
                        1 | 2 => 3,
                        _ => unreachable!("Gi o'er!"),
                    }
                } else {
                    1
                }
            }
        }
        _ => {
            assert!(counts.keys().len() == 5);
            if alt {
                match num_jacks {
                    0 => 0,
                    1 => 1,
                    _ => unreachable!("Nay, lad!"),
                }
            } else {
                0
            }
        }
    }
}

fn card_val(card: char, alt: bool) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if alt {
                0
            } else {
                11
            }
        }
        'T' => 10,
        _ => card.to_digit(10).unwrap() as usize,
    }
}
