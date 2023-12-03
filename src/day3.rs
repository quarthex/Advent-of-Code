const INPUT: &str = include_str!("day3.txt");

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

pub(crate) fn first_part() -> u32 {
    // Split the input into lines.
    let lines: Vec<_> = INPUT.trim().lines().collect();

    // Find numbers.
    lines
        .iter()
        .enumerate()
        .flat_map(|(line, s)| {
            // Find the numbers in the current line.
            s.char_indices().filter_map(move |(column, c)| {
                // Get the previous character.
                let prev = s[..column].chars().last();
                // If the current character is a digit but not the previous one.
                if c.is_digit(10) && (prev.map_or(true, |c| !c.is_digit(10))) {
                    // Get the whole number.
                    let number = s[column..]
                        .split_once(|c: char| !c.is_digit(10))
                        .unwrap_or((&s[column..], ""))
                        .0;
                    Some((line, column, number))
                } else {
                    None
                }
            })
        })
        // Filter the number which are adjacent to a symbol.
        .filter_map(|(line, column, number)| {
            // Lines range.
            let from_line = line.checked_sub(1).unwrap_or(0);
            let to_line = line + 1;
            // Columns range.
            let from_col = column.checked_sub(1).unwrap_or(0);
            let to_col = column + number.len();
            // Find symbols in the ranges.
            let is_adjacent = (from_line..=to_line).any(|line| {
                (from_col..=to_col).any(|column| {
                    lines
                        .get(line)
                        .and_then(|line| line.chars().nth(column))
                        .map_or(false, |c| is_symbol(c))
                })
            });
            if is_adjacent {
                Some(number.parse::<u32>().unwrap_or(0))
            } else {
                None
            }
        })
        // Sum.
        .sum()
}
