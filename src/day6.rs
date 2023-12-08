use std::{num::ParseIntError, ops::Range, str::FromStr};

const INPUT: &str = include_str!("day6.txt");

/// Invalid input.
#[derive(Debug)]
enum InvalidInput {
    /// Invalid integer.
    ParseInt(ParseIntError),
    /// Another parsing error.
    Other,
}

impl From<ParseIntError> for InvalidInput {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

/// A race.
struct Race {
    /// Duration of the race.
    time: u32,
    /// Record distance.
    distance: u32,
}

impl Race {
    /// Compute a distance in function of an hold time.
    const fn score(&self, hold_time: u32) -> u32 {
        (self.time - hold_time) * hold_time
    }

    /// Get the range of hold times that can beat the record.
    fn beatable_range(self) -> Range<u32> {
        (0..self.time)
            .find(|time| self.score(*time) > self.distance)
            .map_or(0..0, |start| {
                let end = self.time - start + 1;
                start..end
            })
    }
}

// A list of races.
struct Races(Vec<Race>);

/// Parse the input.
impl FromStr for Races {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim_end().lines();

        // Parse the times.
        let times = lines
            .next()
            .ok_or(InvalidInput::Other)?
            .strip_prefix("Time:")
            .ok_or(InvalidInput::Other)?
            .trim_start()
            .split_ascii_whitespace()
            .map(str::parse);

        // Parse the distances.
        let distances = lines
            .next()
            .ok_or(InvalidInput::Other)?
            .strip_prefix("Distance:")
            .ok_or(InvalidInput::Other)?
            .trim_start()
            .split_ascii_whitespace()
            .map(str::parse);

        // Zip lists together.
        times
            .zip(distances)
            .map(|(time, distance)| {
                Ok(Race {
                    time: time?,
                    distance: distance?,
                })
            })
            .collect::<Result<_, _>>()
            .map(Races)
    }
}

pub fn first_part() -> u32 {
    // Parse the input.
    INPUT
        .parse::<Races>()
        .expect("Invalid input")
        .0
        // Get beating range.
        .into_iter()
        .map(Race::beatable_range)
        // Convert to count.
        .map(|r| r.end - r.start)
        // Multiply.
        .product()
}
