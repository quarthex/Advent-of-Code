const INPUT: &str = include_str!("day1.txt");

pub(crate) fn first_part() -> u32 {
    // Split the input into lines.
    let list = INPUT
        .trim()
        .lines()
        // For each line, get only the digits.
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)));
    common_part(list)
}

pub(crate) fn second_part() -> u32 {
    const DIGITS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // Split the input into lines.
    let list = INPUT
        .trim()
        .lines()
        // Convert each line into an list of digits.
        .map(|line| {
            line.char_indices().filter_map(|(index, c)| {
                // Try to convert from a single digit...
                c.to_digit(10)
                    // ...or try to convert from a string.
                    .or_else(|| {
                        DIGITS
                            .iter()
                            .enumerate()
                            .find(|(_digit, &name)| line[index..].starts_with(name))
                            .and_then(|(digit, _name)| u32::try_from(digit).ok())
                    })
            })
        });
    common_part(list)
}

fn common_part<I>(list: I) -> u32
where
    I: Iterator,
    I::Item: Iterator<Item = u32>,
{
    list.map(|mut digits| {
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
