use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

const INPUT: &str = include_str!("day5.txt");

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
    /// Destination range.
    destination: RangeInclusive<u32>,
    /// Source range.
    source: RangeInclusive<u32>,
}

impl Map {
    /// Try to map a source into a destination.
    fn map(&self, source: u32) -> Option<u32> {
        if self.source.contains(&source) {
            Some(source - self.source.start() + self.destination.start())
        } else {
            None
        }
    }

    /// Try to map a range of sources into a range of destinations.
    /// Also return is was not mapped.
    fn map_range(
        &self,
        source: RangeInclusive<u32>,
    ) -> (Option<RangeInclusive<u32>>, Vec<RangeInclusive<u32>>) {
        // Reject what is before the source range.
        let before = if source.start() < self.source.start() {
            let end = (*source.end()).min(self.source.start() - 1);
            Some(*source.start()..=end)
        } else {
            None
        };

        // Reject what is after the source range.
        let after = if self.source.end() < source.end() {
            let start = (self.source.end() + 1).max(*source.start());
            Some(start..=*source.end())
        } else {
            None
        };

        // Get intersection.
        let start = (*source.start()).max(*self.source.start());
        let end = (*source.end()).min(*self.source.end());
        // Map to destination.
        let destination = if start <= end {
            let start = start - self.source.start() + self.destination.start();
            let end = end - self.source.start() + self.destination.start();
            Some(start..=end)
        } else {
            None
        };

        let rejected = [before, after].into_iter().flatten().collect();

        (destination, rejected)
    }
}

impl FromStr for Map {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string.
        let mut parts = s.split_ascii_whitespace();
        // Convert the values.
        let destination_start = parts.next().ok_or(InvalidInput::Other)?.parse()?;
        let source_start = parts.next().ok_or(InvalidInput::Other)?.parse()?;
        let range_length: u32 = parts.next().ok_or(InvalidInput::Other)?.parse()?;
        // Check for additional values.
        if parts.next().is_none() && range_length > 0 {
            let r = range_length - 1;
            Ok(Self {
                destination: destination_start..=destination_start + r,
                source: source_start..=source_start + r,
            })
        } else {
            Err(InvalidInput::Other)
        }
    }
}

/// Collection of maps.
#[derive(Debug, PartialEq)]
struct Maps(Vec<Map>);

impl Maps {
    /// Map a range of sources into a collecton of destination ranges.
    fn map(&self, source: RangeInclusive<u32>) -> Vec<RangeInclusive<u32>> {
        let mut destination = Vec::new();

        let source = self.0.iter().fold(vec![source], |source, map| {
            source
                .into_iter()
                .flat_map(|s| {
                    let (d, r) = map.map_range(s);
                    destination.extend(d);
                    r
                })
                .collect()
        });

        destination.into_iter().chain(source).collect()
    }
}

/// The input.
struct Input {
    /// List of seeds.
    seeds: Vec<u32>,
    /// Seed to soil maps.
    seed_to_soil: Maps,
    /// Soil to fertilizer maps.
    soil_to_fertilizer: Maps,
    /// Fertilizer to water maps.
    fertilizer_to_water: Maps,
    /// Water to light maps.
    water_to_light: Maps,
    /// Ligth to temperature maps.
    light_to_temperature: Maps,
    /// Temperature to humidity maps.
    temperature_to_humidity: Maps,
    /// Humidity to location maps.
    humidity_to_location: Maps,
}

impl Input {
    /// Map a seed to a location.
    fn seed_to_location(&self, seed: u32) -> u32 {
        // Fold the maps.
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
        .into_iter()
        .fold(seed, |source, maps| {
            // Find a map matching a source.
            maps.0
                .iter()
                .find_map(|map| map.map(source))
                .unwrap_or(source)
        })
    }

    fn seeds_to_locations(&self, seeds: RangeInclusive<u32>) -> Vec<RangeInclusive<u32>> {
        // Fold the maps.
        [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
        .into_iter()
        .fold(vec![seeds], |sources, maps| {
            // Convert a set of sources into a set of locations.
            sources
                .into_iter()
                .flat_map(|sources| maps.map(sources))
                .collect()
        })
    }
}

/// Input parser.
impl FromStr for Input {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /// Parse a list of maps.
        fn parse_maps<'a, I>(lines: &mut I, name: &str) -> Result<Maps, InvalidInput>
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
                .map(str::parse)
                .collect::<Result<_, _>>()
                .map(Maps)
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
        if lines.next().is_some() {
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

pub fn first_part() -> u32 {
    // Parse the input.
    let input: Input = INPUT.parse().expect("Invalid input");
    // Map each seed into a location.
    input
        .seeds
        .iter()
        .map(|seed| input.seed_to_location(*seed))
        // Find the minimum value.
        .min()
        .expect("Empty seed list")
}

pub fn second_part() -> u32 {
    // Parse the input.
    let input: Input = INPUT.parse().expect("Invalid input");
    // Convert the seeds to range of seeds.
    input
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let start = chunk[0];
            let end = start + chunk[1] - 1;
            start..=end
        })
        // Convert seeds to locations.
        .flat_map(|seeds| input.seeds_to_locations(seeds))
        // Get the minimum location
        .map(|range| *range.start())
        .min()
        .expect("Empty location list")
}

#[cfg(test)]
mod tests {
    use super::{Input, Map};

    impl Map {
        const fn new(destination_start: u32, source_start: u32, range_length: u32) -> Self {
            Self {
                destination: destination_start..=destination_start + range_length - 1,
                source: source_start..=source_start + range_length - 1,
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
            input.seed_to_soil.0,
            vec![Map::new(50, 98, 2), Map::new(52, 50, 48)]
        );
        assert_eq!(
            input.soil_to_fertilizer.0,
            [
                Map::new(0, 15, 37),
                Map::new(37, 52, 2),
                Map::new(39, 0, 15),
            ]
        );
        assert_eq!(
            input.fertilizer_to_water.0,
            [
                Map::new(49, 53, 8),
                Map::new(0, 11, 42),
                Map::new(42, 0, 7),
                Map::new(57, 7, 4)
            ]
        );
        assert_eq!(
            input.water_to_light.0,
            [Map::new(88, 18, 7), Map::new(18, 25, 70)]
        );
        assert_eq!(
            input.light_to_temperature.0,
            [
                Map::new(45, 77, 23),
                Map::new(81, 45, 19),
                Map::new(68, 64, 13),
            ]
        );
        assert_eq!(
            input.temperature_to_humidity.0,
            [Map::new(0, 69, 1), Map::new(1, 0, 69)]
        );
        assert_eq!(
            input.humidity_to_location.0,
            [Map::new(60, 56, 37), Map::new(56, 93, 4)]
        );
    }
}
