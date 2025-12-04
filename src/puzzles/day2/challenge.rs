use std::{
    error::Error,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::puzzles::day2::range::Range;

pub struct Challenge {
    ranges: Vec<Range>,
    invalid_ids: Vec<u64>,
}

impl Challenge {
    fn parse_ranges(content: String) -> Result<Vec<Range>, Box<dyn Error>> {
        let ranges: Result<Vec<Range>, Box<dyn Error>> = content
            .split(',')
            .map(|part| part.trim())
            .filter_map(|part| part.split_once('-'))
            .map(|(min_s, max_s)| -> Result<Range, Box<dyn Error>> {
                let min = min_s.trim().parse::<u64>()?;
                let max = max_s.trim().parse::<u64>()?;

                Ok(Range::new(min, max))
            })
            .collect::<Result<Vec<Range>, Box<dyn Error>>>();

        ranges
    }

    pub fn new<P: AsRef<Path>>(input_path: P) -> Self {
        let path: PathBuf = input_path.as_ref().to_path_buf();

        let ranges = match read_to_string(&path) {
            Ok(file_content) => Challenge::parse_ranges(file_content),
            Err(e) => Err(Box::new(e) as Box<dyn Error>),
        };

        Self {
            ranges: ranges.unwrap_or(vec![]),
            invalid_ids: vec![],
        }
    }

    pub fn solve(&mut self) -> Option<u64> {
        for range in &self.ranges {
            println!("Range: min={}, max={}", range.min, range.max);

            if let Some(invalids) = range.find_invalids() {
                println!("Invalid IDs: {:?}", invalids);
                self.invalid_ids.extend(invalids)
            }
        }

        let total: u64 = self.invalid_ids.iter().sum::<u64>();

        Some(total)
    }
}
