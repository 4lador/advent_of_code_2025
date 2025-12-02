use std::{path::PathBuf, u32};

use crate::puzzles::day1::sequence::Sequence;

pub fn run(rotations_file_path: PathBuf) -> Option<u32> {
    let mut sequence = Sequence::new(50, 0, 99, rotations_file_path);
    let pwd_result = sequence.find_password();

    match pwd_result {
        Ok(password) => Some(password),
        Err(e) => {
            println!(
                "An error occured while looking for challenge password: {}",
                e
            );

            None
        }
    }
}
