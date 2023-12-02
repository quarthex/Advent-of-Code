const INPUT: &str = include_str!("day1.txt");

pub fn result() -> u32 {
    // Split the input into lines.
    INPUT
        .trim()
        .lines()
        // For each line, get only the digits.
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .map(|mut digits| {
            // For each list of digits, get the first one...
            let first = digits
                .next()
                .expect("There should always be at least one digit");
            // ...and the last one (if it exists)
            let last = digits.last().unwrap_or(first);
            // Convert to a single number.
            first * 10 + last
        })
        // Returns the sum.
        .sum()
}
