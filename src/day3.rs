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
            s.chars().enumerate().filter_map(move |(column, c)| {
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

pub(crate) fn second_part() -> u32 {
    const GEAR: char = '*';

    // Split the input into lines.
    let lines: Vec<_> = INPUT.trim().lines().collect();

    // Find stars
    lines
        .iter()
        .enumerate()
        .flat_map(|(line, s)| {
            s.chars().enumerate().filter_map(move |(column, c)| {
                if c == GEAR {
                    Some((line, column))
                } else {
                    None
                }
            })
        })
        // Attach surrounding numbers.
        .map(|(line, column)| {
            let mut numbers = Vec::new();
            // Left.
            if let Some(s) = lines[line][..column]
                .split(|c: char| !c.is_ascii_digit())
                .last()
            {
                if !s.is_empty() {
                    numbers.push(s);
                }
            }
            // Right
            if let Some(s) = lines[line][column + 1..]
                .split(|c: char| !c.is_ascii_digit())
                .next()
            {
                if !s.is_empty() {
                    numbers.push(s);
                }
            }
            // Top and bottom
            let top = line.checked_sub(1);
            let bottom = line + 1;
            for line in top.into_iter().chain(Some(bottom)) {
                // If the character on the same column is a digit.
                if lines[line][column..].starts_with(|c: char| c.is_ascii_digit()) {
                    // Find the start of the number.
                    let column = lines[line][..column]
                        .rfind(|c: char| !c.is_ascii_digit())
                        .map(|column| column + 1)
                        .unwrap_or(0);
                    // Get the number.
                    if let Some(number) = lines[line][column..]
                        .split(|c: char| !c.is_ascii_digit())
                        .next()
                    {
                        numbers.push(number)
                    }
                } else {
                    // Number after the current column.
                    if let Some(s) = lines[line][column + 1..]
                        .split(|c: char| !c.is_ascii_digit())
                        .next()
                    {
                        if !s.is_empty() {
                            numbers.push(s)
                        }
                    }
                    // Number before the current column.
                    if let Some(s) = lines[line][..column]
                        .rsplit(|c: char| !c.is_ascii_digit())
                        .next()
                    {
                        if !s.is_empty() {
                            numbers.push(s)
                        }
                    }
                }
            }
            numbers
        })
        // Filter the gears with 2 adjacent numbers.
        .filter(|numbers| numbers.len() == 2)
        // Compute gear ratios
        .map(|numbers| {
            numbers
                .into_iter()
                .filter_map(|n| n.parse::<u32>().ok())
                .fold(1, |a, b| a * b)
        })
        // Sum.
        .sum()
}
