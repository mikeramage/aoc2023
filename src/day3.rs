use crate::utils;
use std::collections::HashMap;

///Day 3 solution
pub fn day3() -> (usize, usize) {
    let schematic = utils::parse_input_chars("input/day3.txt");
    let mut numbers_adjacent_to_symbols: Vec<usize> = vec![];
    let mut gear_mappings: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut in_number = false;
    let mut current_digits: Vec<char> = vec![];
    let mut number_start_x = 0;
    let mut number_start_y = 0;
    for x in 0..schematic.len() {
        if in_number {
            //edge case - number ended at end of last row
            if let Some((x_sym, y_sym)) = is_adjacent_to_symbol(
                number_start_x,
                number_start_y,
                current_digits.len(),
                &schematic,
            ) {
                let number = current_digits.iter().collect::<String>().parse().unwrap();
                if schematic[x_sym][y_sym] == '*' {
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        gear_mappings.entry((x_sym, y_sym))
                    {
                        //Create new vector and add to gear_mappings
                        e.insert(vec![number]);
                    } else {
                        //Append to vector already there
                        gear_mappings.get_mut(&(x_sym, y_sym)).unwrap().push(number);
                    }
                }
                numbers_adjacent_to_symbols.push(number)
            }
        }

        in_number = false;
        current_digits = vec![];
        number_start_x = 0;
        number_start_y = 0;

        for y in 0..schematic[x].len() {
            if schematic[x][y].is_ascii_digit() {
                //number
                current_digits.push(schematic[x][y]);

                if !in_number {
                    // if we weren't in a number, we are now.
                    in_number = true;
                    number_start_x = x;
                    number_start_y = y;
                }
            } else if in_number {
                //No longer in number - check if it's adjacent to a symbol
                if let Some((x_sym, y_sym)) = is_adjacent_to_symbol(
                    number_start_x,
                    number_start_y,
                    current_digits.len(),
                    &schematic,
                ) {
                    let number = current_digits.iter().collect::<String>().parse().unwrap();
                    if schematic[x_sym][y_sym] == '*' {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            gear_mappings.entry((x_sym, y_sym))
                        {
                            //Create new vector and add to gear_mappings
                            e.insert(vec![number]);
                        } else {
                            //Append to vector already there
                            gear_mappings.get_mut(&(x_sym, y_sym)).unwrap().push(number);
                        }
                    }
                    numbers_adjacent_to_symbols.push(number)
                }
                in_number = false;
                current_digits = vec![];
            }
        }
    }
    (
        numbers_adjacent_to_symbols.iter().sum(),
        gear_mappings
            .values()
            .filter(|value| value.len() == 2)
            .map(|value| value.iter().product::<usize>())
            .sum(),
    )
}

fn is_adjacent_to_symbol(
    x: usize,
    y: usize,
    length: usize,
    schematic: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    // Got something like this, x increasing down the way, y increasing to the right.
    // o o o o o   |----y
    // o D d d o   |
    // o o o o o   x
    // D, d are digits, x and y are those of the 'D'. If any of the 'o's are symbols (not '.'s or digits), then return true, else false.
    // so x range is Dx - 1 to Dx + 1, y range is Dy - 1 to Dy + length, with min x (or y) of 0 and max x is schematic.len() - 1 and max y is schematic[0].len() - 1.
    let x_min: usize = if x == 0 { 0 } else { x - 1 };

    let y_min: usize = if y == 0 { 0 } else { y - 1 };

    for x in x_min..std::cmp::min(x + 2, schematic.len()) {
        for y in y_min..std::cmp::min(y + length + 1, schematic[0].len()) {
            if !schematic[x][y].is_ascii_digit() && schematic[x][y] != '.' {
                //It's a symbol
                return Some((x, y));
            }
        }
    }

    None
}
