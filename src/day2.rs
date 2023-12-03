use std::str::FromStr;

const INPUT: &str = include_str!("day2.txt");

#[derive(Debug)]
/// Error for invalid inputs.
struct InvalidInput;

/// A cube set.
#[derive(Default)]
struct CubeSet {
    /// The amount of red cubes.
    red: u32,
    /// The amount of green cubes.
    green: u32,
    /// The amount of blue cubes.
    blue: u32,
}

/// Parse a cube set.
impl FromStr for CubeSet {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Create a default cube set.
        let cube_set = CubeSet::default();
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

struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
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
            .collect::<Result<Vec<CubeSet>, _>>()
            .map_err(|_| InvalidInput)?;
        Ok(Self { id, cube_sets })
    }
}

pub(crate) fn first_part() -> u32 {
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
        .filter(|game| {
            game.cube_sets.iter().all(|cube_set| {
                cube_set.red <= MAX_RED && cube_set.green <= MAX_GREEN && cube_set.blue <= MAX_BLUE
            })
        })
        // Get the ID.
        .map(|game| game.id)
        // Sum.
        .sum()
}

pub(crate) fn second_part() -> u32 {
    // Parse the input.
    INPUT
        .trim()
        .lines()
        .map(Game::from_str)
        .map(|game| game.expect("Invalid input"))
        // Get the required cube set for each game.
        .map(|game| {
            game.cube_sets
                .iter()
                .fold(CubeSet::default(), |first, second| CubeSet {
                    red: first.red.max(second.red),
                    green: first.green.max(second.green),
                    blue: first.blue.max(second.blue),
                })
        })
        // Compute the power.
        .map(|cube_set| cube_set.red * cube_set.green * cube_set.blue)
        // Sum.
        .sum()
}
