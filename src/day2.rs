use std::str::FromStr;

const INPUT: &str = include_str!("day2.txt");

#[derive(Debug, PartialEq)]
/// Error for invalid inputs.
struct InvalidInput;

/// A cube set.
#[derive(Debug, Default, PartialEq)]
struct CubeSet {
    /// The amount of red cubes.
    red: u32,
    /// The amount of green cubes.
    green: u32,
    /// The amount of blue cubes.
    blue: u32,
}

impl CubeSet {
    const fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }

    const fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

/// Parse a cube set.
impl FromStr for CubeSet {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Create a default cube set.
        let cube_set = Self::default();
        // Use the commas to split the string.
        s.split(", ")
            .try_fold(cube_set, |mut cube_set, part| {
                // Split the count from the color.
                let (n, color) = part.split_once(' ')?;
                // Parse the count.
                let n = n.parse().ok()?;
                // Parse the color.
                match color {
                    "red" => cube_set.red = n,
                    "green" => cube_set.green = n,
                    "blue" => cube_set.blue = n,
                    _ => return None,
                }
                Some(cube_set)
            })
            .ok_or(InvalidInput)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.cube_sets
            .iter()
            .all(|cube_set| cube_set.is_possible(max_red, max_green, max_blue))
    }

    fn required_cube_set(self) -> CubeSet {
        self.cube_sets
            .iter()
            .fold(CubeSet::default(), |first, second| CubeSet {
                red: first.red.max(second.red),
                green: first.green.max(second.green),
                blue: first.blue.max(second.blue),
            })
    }
}

/// Parse a game.
impl FromStr for Game {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip the prefix.
        let s = s.strip_prefix("Game ").ok_or(InvalidInput)?;
        // Split the ID from the cube sets.
        let (id, cube_sets) = s.split_once(": ").ok_or(InvalidInput)?;
        // Parse the ID.
        let id = id.parse().map_err(|_| InvalidInput)?;
        // Split the games.
        let cube_sets = cube_sets
            .split("; ")
            // Parse the games.
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(|_| InvalidInput)?;
        Ok(Self { id, cube_sets })
    }
}

pub fn first_part() -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    // Parse the input.
    INPUT
        .trim()
        .lines()
        .map(Game::from_str)
        .map(|game| game.expect("Invalid input"))
        // Filter possible games.
        .filter(|game| game.is_possible(MAX_RED, MAX_GREEN, MAX_BLUE))
        // Get the ID.
        .map(|game| game.id)
        // Sum.
        .sum()
}

pub fn second_part() -> u32 {
    // Parse the input.
    INPUT
        .trim()
        .lines()
        .map(Game::from_str)
        .map(|game| game.expect("Invalid input"))
        // Get the required cube set for each game.
        .map(Game::required_cube_set)
        // Compute the power.
        .map(CubeSet::power)
        // Sum.
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{CubeSet, Game};

    impl CubeSet {
        const fn new(red: u32, green: u32, blue: u32) -> Self {
            Self { red, green, blue }
        }
    }

    #[test]
    fn test() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse(),
            Ok(Game {
                id: 1,
                cube_sets: vec![
                    CubeSet::new(4, 0, 3),
                    CubeSet::new(1, 2, 6),
                    CubeSet::new(0, 2, 0),
                ]
            })
        );
        assert_eq!(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".parse(),
            Ok(Game {
                id: 2,
                cube_sets: vec![
                    CubeSet::new(0, 2, 1),
                    CubeSet::new(1, 3, 4),
                    CubeSet::new(0, 1, 1),
                ]
            })
        );
        assert_eq!(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".parse(),
            Ok(Game {
                id: 3,
                cube_sets: vec![
                    CubeSet::new(20, 8, 6),
                    CubeSet::new(4, 13, 5),
                    CubeSet::new(1, 5, 0),
                ]
            })
        );
        assert_eq!(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".parse(),
            Ok(Game {
                id: 4,
                cube_sets: vec![
                    CubeSet::new(3, 1, 6),
                    CubeSet::new(6, 3, 0),
                    CubeSet::new(14, 3, 15),
                ]
            })
        );
        assert_eq!(
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".parse(),
            Ok(Game {
                id: 5,
                cube_sets: vec![CubeSet::new(6, 3, 1), CubeSet::new(1, 2, 2),]
            })
        );
    }
}
