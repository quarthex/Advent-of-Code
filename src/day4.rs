use std::str::FromStr;

const INPUT: &str = include_str!("day4.txt");

/// Invalid input error.
#[derive(Debug)]
struct InvalidInput;

/// A game card.
#[derive(Debug, PartialEq)]
struct Card {
    /// ID of the card.
    id: u32,
    /// Winning numbers.
    winning: Vec<u32>,
    /// Got numbers.
    got: Vec<u32>,
}

impl Card {
    /// Count the amount of matching numbers.
    fn matching(&self) -> usize {
        self.got.iter().filter(|n| self.winning.contains(n)).count()
    }

    /// Compute the amount of points of a card.
    fn points(&self) -> u32 {
        let count = self.matching();

        // If none match, returns 0.
        // If at least one matches, return 2^n.
        count.checked_sub(1).map_or(0, |n| 1 << n)
    }
}

/// Convert a line of the input into a card.
impl FromStr for Card {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip the prefix.
        let s = s.strip_prefix("Card ").ok_or(InvalidInput)?;
        // Split the ID from the rest.
        let (id, s) = s.split_once(": ").ok_or(InvalidInput)?;
        let id = id.trim_start().parse().map_err(|_| InvalidInput)?;
        // Split the winning numbers from the got numbers.
        let (winning, got) = s.split_once(" | ").ok_or(InvalidInput)?;
        // Convert winning and got numbers into a list of integers.
        let winning = winning
            .split_ascii_whitespace()
            .map(|s| s.parse().map_err(|_| InvalidInput))
            .collect::<Result<_, _>>()?;
        let got = got
            .split_ascii_whitespace()
            .map(|s| s.parse().map_err(|_| InvalidInput))
            .collect::<Result<_, _>>()?;
        // Return the resulting card.
        Ok(Self { id, winning, got })
    }
}

pub fn first_part() -> u32 {
    // Parse the input.
    INPUT
        .trim()
        .lines()
        .map(|s| s.parse::<Card>().expect("Invalid input"))
        // Compute the points of each card.
        .map(|card| card.points())
        // Sum it.
        .sum()
}

pub fn second_part() -> u32 {
    // Count the amount of cards
    let count = INPUT.trim().lines().count();

    // Create a vector of cards count.
    let mut counts = vec![1; count];

    // Parse the input.
    let cards = INPUT
        .trim()
        .lines()
        .map(|s| s.parse::<Card>().expect("Invalid input"));

    // Iterate from the top-most card to the bottom.
    for (index, card) in cards.enumerate() {
        // Get the matching count.
        let matching = card.matching();

        // Increase the counts of the following next cards.
        for i in 1..=matching {
            counts[index + i] += counts[index];
        }
    }

    // Total count.
    counts.into_iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn test() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        // Check the parsing.
        let mut cards = INPUT.lines().map(|s| s.parse::<Card>().unwrap());
        assert_eq!(
            cards.next(),
            Some(Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                got: vec![83, 86, 6, 31, 17, 9, 48, 53]
            })
        );
        assert_eq!(
            cards.next(),
            Some(Card {
                id: 2,
                winning: vec![13, 32, 20, 16, 61],
                got: vec![61, 30, 68, 82, 17, 32, 24, 19]
            })
        );
        assert_eq!(
            cards.next(),
            Some(Card {
                id: 3,
                winning: vec![1, 21, 53, 59, 44],
                got: vec![69, 82, 63, 72, 16, 21, 14, 1]
            })
        );
        assert_eq!(
            cards.next(),
            Some(Card {
                id: 4,
                winning: vec![41, 92, 73, 84, 69],
                got: vec![59, 84, 76, 51, 58, 5, 54, 83]
            })
        );
        assert_eq!(
            cards.next(),
            Some(Card {
                id: 5,
                winning: vec![87, 83, 26, 28, 32],
                got: vec![88, 30, 70, 12, 93, 22, 82, 36]
            })
        );
        assert_eq!(
            cards.next(),
            Some(Card {
                id: 6,
                winning: vec![31, 18, 13, 56, 72],
                got: vec![74, 77, 10, 23, 35, 67, 36, 11]
            })
        );
        assert_eq!(cards.next(), None);

        // Check the points computation rule.
        let cards = INPUT.lines().map(|s| s.parse::<Card>().unwrap());
        let points: Vec<_> = cards.map(|card| card.points()).collect();
        assert_eq!(points, [8, 2, 2, 1, 0, 0]);
    }
}
