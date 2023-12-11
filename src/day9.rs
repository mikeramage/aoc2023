use crate::utils;

//Day 8 solution
pub fn day9() -> (usize, usize) {
    let input: Vec<String> = utils::parse_input("input/day9.txt");
    let sequences: Vec<Vec<isize>> = input
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|d| d.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    let mut next_values: Vec<isize> = vec![];
    let mut prev_values: Vec<isize> = vec![];
    for sequence in sequences {
        let mut temp = sequence.clone();
        let mut last_numbers: Vec<isize> = vec![];
        let mut first_numbers: Vec<isize> = vec![];
        last_numbers.push(temp[temp.len() - 1]);
        first_numbers.push(temp[0]);

        while !(temp.iter().all(|x| *x == 0)) {
            temp = temp.windows(2).map(|x| x[1] - x[0]).collect();
            last_numbers.push(temp[temp.len() - 1]);
            first_numbers.push(temp[0]);
        }
        next_values.push(last_numbers.iter().sum());
        let mut prev_value: isize = 0;
        for num in first_numbers.iter().rev() {
            prev_value = *num - prev_value;
        }
        prev_values.push(prev_value);
    }
    (
        next_values.iter().sum::<isize>() as usize,
        prev_values.iter().sum::<isize>() as usize,
    )
}
