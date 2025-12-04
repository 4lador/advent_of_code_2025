use std::collections::HashSet;

use crate::utils::math::{digits, is_pair};

pub struct Range {
    pub min: u64,
    pub max: u64,
}

impl Range {
    pub const fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }

    pub fn find_invalids_part_1(&self) -> Option<Vec<u64>> {
        let mut invalids: Vec<u64> = Vec::new();
        for i in self.min..=self.max {
            let digits = digits(i);
            let is_pair = is_pair(digits);

            if !is_pair {
                continue;
            }

            let i_str = i.to_string();
            let middle = i_str.len() / 2;
            let (left, right) = i_str.split_at(middle);

            if left == right {
                invalids.push(i);
            }
        }

        if invalids.len() > 0 {
            Some(invalids)
        } else {
            None
        }
    }

    // For each possible length of numbers, construct possible repeatable patterns and check if this is in range
    pub fn find_invalids_part_2(&self) -> Option<Vec<u64>> {
        let mut invalids: HashSet<u64> = HashSet::new();

        let min_digits = digits(self.min);
        let max_digits = digits(self.max);

        for l in min_digits..=max_digits {
            for k in 1..=l / 2 {
                if l % k != 0 {
                    continue;
                }

                let start = 10_u64.pow((k - 1) as u32);
                let end = 10_u64.pow(k as u32) - 1;

                for m in start..=end {
                    let s = m.to_string().repeat(l / k);
                    let number = s.parse::<u64>().unwrap();

                    if number >= self.min && number <= self.max {
                        invalids.insert(number);
                    }
                }
            }
        }

        let mut invalid_vec: Vec<u64> = invalids.into_iter().collect();
        invalid_vec.sort_unstable();
        Some(invalid_vec)
    }
}
