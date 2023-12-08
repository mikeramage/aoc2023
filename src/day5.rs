use crate::utils;
use itertools::enumerate;

///Day 5 solution
pub fn day5() -> (usize, usize) {
    let input: Vec<String> = utils::parse_input_by_blank_lines("input/day5.txt");
    assert!(input.len() == 8);
    let seeds: Vec<usize> = input[0]
        .split_whitespace()
        .filter(|x| x.starts_with(|d: char| d.is_ascii_digit()))
        .map(|d| d.parse::<usize>().unwrap())
        .collect();
    let mappings: Vec<Vec<(usize, usize, usize)>> = input[1..]
        .iter()
        .map(|x| {
            let mapping: Vec<(usize, usize, usize)> = x
                .split('\n')
                .filter(|y: &&str| y.starts_with(|z: char| z.is_ascii_digit()))
                .map(|y| {
                    let mapping_parameters: Vec<usize> = y
                        .split_whitespace()
                        .map(|d| d.parse::<usize>().unwrap())
                        .collect();
                    let mapping_parameters = (
                        mapping_parameters[0],
                        mapping_parameters[1],
                        mapping_parameters[2],
                    );
                    mapping_parameters
                })
                .collect();
            mapping
        })
        .collect();

    let locations: Vec<usize> = seeds.iter().map(|&seed| {
        let mut location = seed;
        for mapping in &mappings {
           location = map_input(location, mapping);
        }
        location
    }).collect();
    
    //This is brute force, but runs in about three and a half minutes
    let mut min_location = 100000000000; //Something giant.
    for pair in seeds.chunks(2) {
        let start = pair[0];
        let range = pair[1];
        for (i, seed) in enumerate(start..start+range) {
            let mut location = seed;
            for mapping in &mappings {
                location = map_input(location, mapping);
            }

            if location < min_location {
                min_location = location;
            }
        }
    }    

    (*locations.iter().min().unwrap(), min_location)
}

fn map_input(input: usize, mappings: &Vec<(usize, usize, usize)>) -> usize {
    for mapping in mappings {
        let (dest, src, range) = mapping;
        if (input >= *src) && (input < (src + range)) {
            return input - src + dest;
        }
    }

    input
}
