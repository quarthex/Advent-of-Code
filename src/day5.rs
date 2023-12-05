use std::{num::ParseIntError, str::FromStr};

/// Invalid input.
#[derive(Debug, PartialEq)]
enum InvalidInput {
    /// Integer convertion error.
    ParseInt(ParseIntError),
    /// Other kinds of error.
    Other,
}

/// Encapsulate a `ParseIntError` into an `InvalidInput`.
impl From<ParseIntError> for InvalidInput {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

/// A mapping.
#[derive(Debug, PartialEq)]
struct Map {
    /// Start of the destination range.
    destination_range_start: u32,
    /// Start of the source range.
    source_range_start: u32,
    /// Length of both ranges.
    range_length: usize,
}

impl FromStr for Map {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string.
        let mut parts = s.split_ascii_whitespace();
        // Convert the values.
        let destination_range_start = parts.next().ok_or(InvalidInput::Other)?.parse()?;
        let source_range_start = parts.next().ok_or(InvalidInput::Other)?.parse()?;
        let range_length = parts.next().ok_or(InvalidInput::Other)?.parse()?;
        // Check for additional values.
        if let None = parts.next() {
            Ok(Self {
                destination_range_start,
                source_range_start,
                range_length,
            })
        } else {
            Err(InvalidInput::Other)
        }
    }
}

/// The input.
struct Input {
    /// List of seeds.
    seeds: Vec<u32>,
    /// Seed to soil maps.
    seed_to_soil: Vec<Map>,
    /// Soil to fertilizer maps.
    soil_to_fertilizer: Vec<Map>,
    /// Fertilizer to water maps.
    fertilizer_to_water: Vec<Map>,
    /// Water to light maps.
    water_to_light: Vec<Map>,
    /// Ligth to temperature maps.
    light_to_temperature: Vec<Map>,
    /// Temperature to humidity maps.
    temperature_to_humidity: Vec<Map>,
    /// Humidity to location maps.
    humidity_to_location: Vec<Map>,
}

/// Input parser.
impl FromStr for Input {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /// Parse a list of maps.
        fn parse_maps<'a, I>(lines: &mut I, name: &str) -> Result<Vec<Map>, InvalidInput>
        where
            I: 'a + Iterator<Item = &'a str>,
        {
            // Get the header.
            let line = lines.next().ok_or(InvalidInput::Other)?;
            // Check the header.
            let s = line
                .strip_prefix(name)
                .and_then(|s| s.strip_prefix(" map:"))
                .ok_or(InvalidInput::Other)?;
            if !s.is_empty() {
                return Err(InvalidInput::Other);
            }
            // Parse maps until the empty line.
            lines
                .take_while(|s| !s.is_empty())
                .map(|line| line.parse())
                .collect()
        }

        // Split into lines.
        let mut lines = s.trim_end().lines();

        // Parse the seeds.
        let line = lines
            .next()
            .and_then(|s| s.strip_prefix("seeds: "))
            .ok_or(InvalidInput::Other)?;
        let seeds: Vec<u32> = line
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        if !lines.next().unwrap_or_default().is_empty() {
            return Err(InvalidInput::Other);
        }

        // Parse maps.
        let seed_to_soil = parse_maps(&mut lines, "seed-to-soil")?;
        let soil_to_fertilizer = parse_maps(&mut lines, "soil-to-fertilizer")?;
        let fertilizer_to_water = parse_maps(&mut lines, "fertilizer-to-water")?;
        let water_to_light = parse_maps(&mut lines, "water-to-light")?;
        let light_to_temperature = parse_maps(&mut lines, "light-to-temperature")?;
        let temperature_to_humidity = parse_maps(&mut lines, "temperature-to-humidity")?;
        let humidity_to_location = parse_maps(&mut lines, "humidity-to-location")?;

        // Check end-of-file.
        if let Some(..) = lines.next() {
            return Err(InvalidInput::Other);
        }

        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::Input;

    use super::Map;

    impl Map {
        const fn new(
            destination_range_start: u32,
            source_range_start: u32,
            range_length: usize,
        ) -> Self {
            Self {
                destination_range_start,
                source_range_start,
                range_length,
            }
        }
    }

    #[test]
    fn parse_map() {
        assert_eq!("50 98 1".parse(), Ok(Map::new(50, 98, 1)));
        assert_eq!("52 50 48".parse(), Ok(Map::new(52, 50, 48)));
    }

    #[test]
    fn parse_input() {
        const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let input: Input = INPUT.parse().expect("Invalid input");
        assert_eq!(input.seeds, [79, 14, 55, 13]);
        assert_eq!(
            input.seed_to_soil,
            [Map::new(50, 98, 2), Map::new(52, 50, 48)]
        );
        assert_eq!(
            input.soil_to_fertilizer,
            [
                Map::new(0, 15, 37),
                Map::new(37, 52, 2),
                Map::new(39, 0, 15),
            ]
        );
        assert_eq!(
            input.fertilizer_to_water,
            [
                Map::new(49, 53, 8),
                Map::new(0, 11, 42),
                Map::new(42, 0, 7),
                Map::new(57, 7, 4)
            ]
        );
        assert_eq!(
            input.water_to_light,
            [Map::new(88, 18, 7), Map::new(18, 25, 70)]
        );
        assert_eq!(
            input.light_to_temperature,
            [
                Map::new(45, 77, 23),
                Map::new(81, 45, 19),
                Map::new(68, 64, 13),
            ]
        );
        assert_eq!(
            input.temperature_to_humidity,
            [Map::new(0, 69, 1), Map::new(1, 0, 69)]
        );
        assert_eq!(
            input.humidity_to_location,
            [Map::new(60, 56, 37), Map::new(56, 93, 4)]
        );
    }
}
