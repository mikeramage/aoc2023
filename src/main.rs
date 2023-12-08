use std::env;
use std::time;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

//With thanks to CJP for the logic behind this framework.
//I tried just to understand what he'd done and reproduce something similar
//But it's basically identical :-(
//
//I'm not copying anyone's solutions though!
static DAYS: [fn() -> (usize, usize); 5] =
    [day1::day1, day2::day2, day3::day3, day4::day4, day5::day5];

fn main() {
    let mut min_day: usize = 1;
    let mut max_day: usize = DAYS.len();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Argument specified
        min_day = args[1]
            .parse()
            .expect("Bad argument - must be a day number");
        max_day = min_day;
    }

    for day in min_day..max_day + 1 {
        println!("Running day {}", day);
        let now = time::Instant::now();
        let (part1, part2): (usize, usize) = DAYS[day - 1]();
        let elapsed_time = now.elapsed();
        println!(
            "Took {}.{:03} ms",
            elapsed_time.as_micros() / 1000,
            elapsed_time.as_micros() % 1000
        );
        println!("Part1 answer: {}", part1);
        println!("Part2 answer: {}", part2);
    }
}
