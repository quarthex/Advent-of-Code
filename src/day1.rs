const INPUT: &str = include_str!("day1.txt");

fn first_part_digits<'a>(s: &'a str) -> impl 'a + Iterator<Item = u32> {
    s.chars().filter_map(|c| c.to_digit(10))
}

pub fn first_part() -> u32 {
    common_part(first_part_digits)
}

fn second_part_digits<'a>(s: &'a str) -> impl 'a + Iterator<Item = u32> {
    const DIGITS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    s.chars().enumerate().filter_map(|(index, c)| {
        // Try to convert from a single digit...
        c.to_digit(10).or_else(|| {
            // ...or try to convert from a string.
            DIGITS
                .iter()
                .enumerate()
                .find(|(_digit, &name)| s[index..].starts_with(name))
                .and_then(|(digit, _name)| u32::try_from(digit).ok())
        })
    })
}

pub fn second_part() -> u32 {
    common_part(second_part_digits)
}

fn common_part<'a, I>(f: fn(&'a str) -> I) -> u32
where
    I: Iterator<Item = u32>,
{
    // Split the input into lines.
    INPUT
        .trim()
        .lines()
        // Convert each line into an list of digits.
        .map(f)
        .map(|mut digits| {
            // For each list of digits, get the first one...
            let first = digits
                .next()
                .expect("There should always be at least one digit");
            // ...and the last one (if it exists)
            let second = digits.last().unwrap_or(first);
            // Convert to a single number.
            first * 10 + second
        })
        // Returns the sum.
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{first_part_digits, second_part_digits};

    #[test]
    fn test_1() {
        assert_eq!(first_part_digits("1abc2").collect::<Vec<_>>(), [1, 2]);
        assert_eq!(first_part_digits("pqr3stu8vwx").collect::<Vec<_>>(), [3, 8]);
        assert_eq!(
            first_part_digits("a1b2c3d4e5f").collect::<Vec<_>>(),
            [1, 2, 3, 4, 5]
        );
        assert_eq!(first_part_digits("treb7uchet").collect::<Vec<_>>(), [7]);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            second_part_digits("two1nine").collect::<Vec<_>>(),
            [2, 1, 9]
        );
        assert_eq!(
            second_part_digits("eightwothree").collect::<Vec<_>>(),
            [8, 2, 3]
        );
        assert_eq!(
            second_part_digits("abcone2threexyz").collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert_eq!(
            second_part_digits("xtwone3four").collect::<Vec<_>>(),
            [2, 1, 3, 4]
        );
        assert_eq!(
            second_part_digits("4nineeightseven2").collect::<Vec<_>>(),
            [4, 9, 8, 7, 2]
        );
        assert_eq!(
            second_part_digits("zoneight234").collect::<Vec<_>>(),
            [1, 8, 2, 3, 4]
        );
        assert_eq!(
            second_part_digits("7pqrstsixteen").collect::<Vec<_>>(),
            [7, 6]
        );
    }
}
