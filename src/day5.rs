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

#[cfg(test)]
mod tests {
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
}
