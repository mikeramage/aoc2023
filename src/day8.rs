use crate::utils;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

//Day 8 solution
pub fn day8() -> (usize, usize) {
    let input: Vec<String> = utils::parse_input("input/day8.txt");
    let instructions = &input[0];
    let re_node = Regex::new(r"(?<key>\w+)\s+=\s+\((?<lval>\w+), (?<rval>\w+)\)").unwrap();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for node in input[2..].iter() {
        let captures = re_node.captures(node).unwrap();
        nodes.insert(
            captures["key"].to_string(),
            (captures["lval"].to_string(), captures["rval"].to_string()),
        );
    }

    // Part 1.
    let part1: usize = calc_num_steps(instructions, &nodes, &vec![&"AAA".to_string()], true);

    // Part 2
    let mut initial_keys: Vec<&String> = vec![];
    for key in nodes.keys() {
        if key.ends_with('A') {
            initial_keys.push(key);
        }
    }

    let part2: usize = calc_num_steps(instructions, &nodes, &initial_keys, false);

    (part1, part2)
}

fn calc_num_steps(
    instructions: &str,
    nodes: &HashMap<String, (String, String)>,
    initial_keys: &Vec<&String>,
    part1: bool,
) -> usize {
    let mut num_steps: usize = 0;
    let mut current_keys: Vec<&String> = initial_keys.clone();
    let mut new_keys: Vec<&String> = vec![];
    let mut zcounts: HashMap<usize, usize> = HashMap::new();

    'outer: loop {
        for instruction in instructions.chars() {
            num_steps += 1;
            for key in current_keys {
                match instruction {
                    'L' => {
                        new_keys.push(&nodes.get(key).unwrap().0);
                    }
                    'R' => {
                        new_keys.push(&nodes.get(key).unwrap().1);
                    }
                    _ => unreachable!("Oops"),
                }
            }
            if part1 && new_keys[0] == "ZZZ"
            // || (!part1 && new_keys.iter().all(|x| x.ends_with('Z'))) - lifetime of universe!
            {
                break 'outer;
            }

            // Debug
            if !part1 {
                for (i, item) in new_keys.iter().enumerate() {
                    if item.ends_with('Z') {
                        // Big assumption that this is cyclic. It is, but that had to be
                        // determined empirically.
                        zcounts.entry(i).or_insert(num_steps);
                    }
                }

                if zcounts.len() == initial_keys.len() {
                    break 'outer;
                }
            }

            current_keys = new_keys;
            new_keys = vec![];
        }
    }

    if part1 {
        num_steps
    } else {
        let mut lowest_common_multiple = 1;
        for x in zcounts.values() {
            lowest_common_multiple = lcm(lowest_common_multiple, *x);
        }
        lowest_common_multiple
    }
}
