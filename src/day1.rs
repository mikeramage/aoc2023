use crate::utils;
use regex::Regex;

///Day 1 solution
pub fn day1() -> (usize, usize) {
    let obfuscated_calibration_values: Vec<String> = utils::parse_input("input/day1.txt");
    (
        sum_calibration_values(&obfuscated_calibration_values, extract_calibration_value),
        sum_calibration_values(
            &obfuscated_calibration_values,
            extract_calibration_value_incl_words,
        ),
    )
}

/// Extracts the calibration value from a string assuming the characters of interest are ascii digits
/// The value is obtained by finding the first digit in the string, multiplying by 10, and adding the last digit in the string.
fn extract_calibration_value(calibration_value: &str) -> usize {
    (calibration_value
        .chars()
        .find(|x| x.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap() as usize)
        * 10
        + calibration_value
            .chars()
            .rev()
            .find(|x| x.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap() as usize
}

/// Extracts the calibration value from a string assuming the characters of interest are ascii digits or words "one", "two", through "nine"
/// The value is obtained by finding the first digit in the string, multiplying by 10, and adding the last digit in the string.
fn extract_calibration_value_incl_words(calibration_value: &str) -> usize {
    let digit_re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut match_iter = digit_re.find_iter(calibration_value);
    let first_match = match_iter.next().unwrap().as_str();
    let last_match = match match_iter.last() {
        Some(x) => x.as_str(),
        None => first_match,
    };
    let first_digit: usize = digit_from_match(first_match);
    let last_digit: usize = digit_from_match(last_match);

    first_digit * 10 + last_digit
}

/// Converts the digit or string match to a usize digit
fn digit_from_match(match_str: &str) -> usize {
    match match_str {
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => match_str.parse().unwrap(),
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!(),
    }
}

fn sum_calibration_values(obfuscated_calibration_values: &[String], f: fn(&str) -> usize) -> usize {
    obfuscated_calibration_values.iter().map(|x| f(x)).sum()
}
