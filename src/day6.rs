use crate::utils;

///Day 6 solution
pub fn day6() -> (usize, usize) {
    // Note that the input is only set up to get day1 right. If you want to get
    // day2 right, you need to manually remove the whitespace between the digits.
    // I am of course far too lazy to write code to smoosh the numbers together.
    let input: Vec<String> = utils::parse_input("input/day6.txt");
    let times: Vec<usize> = input[0].split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|time| time.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = input[1].split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|distance| distance.parse::<usize>().unwrap())
        .collect();

    let ts_ds = times
        .iter()
        .zip(distances)
        .collect::<Vec<(&usize, usize)>>();

    let numbers_of_ways: Vec<_> = ts_ds
        .iter()
        .map(|x| {
            let (time, distance) = *x;

            // d = t_charge * (t - t_charge) because v = t_charge and boat moves for time t - t_charge.
            // d is maximized when t_charge = t / 2.
            // Integer division rounds down, so we step back from t/2 till the distance is below the target distance
            // and drop out the loop. Multiply by 2 (for odd time) and by 2 then subtract 1 for even time.
            let start = time / 2;
            let mut num_ways: usize = 0;
            for t_charge in (0..start + 1).rev() {
                if distance < (t_charge * (time - t_charge)) {
                    num_ways += 1;
                } else {
                    break;
                }
            }
            if time % 2 == 0 {
                // For even time, the max falls exactly on t/2, which is included.
                // Multiplying by 2 would overcount by one (counting the central point twice)
                (num_ways * 2) - 1
            } else {
                num_ways * 2
            }
        })
        .collect();

    // I only realised afterwards that for the smooshed big number input, product
    // gives the same as sum, so I only needed to change the input. Sigh.
    (
        numbers_of_ways.iter().product(),
        numbers_of_ways.iter().sum(),
    )
}
