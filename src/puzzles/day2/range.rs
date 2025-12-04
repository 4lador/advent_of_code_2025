use crate::utils::math::{digits, is_pair};

pub struct Range {
    pub min: u64,
    pub max: u64,
}

impl Range {
    pub const fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }

    pub fn find_invalids(&self) -> Option<Vec<u64>> {
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
}
