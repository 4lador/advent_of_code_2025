use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

enum TurnDirection {
    Left,
    Right,
}

pub struct Sequence {
    current_dial: u32,
    min_dial: u32,
    max_dial: u32,
    rotations_file_path: PathBuf,
}

impl Sequence {
    pub const fn new(
        initial_dial: u32,
        min_dial: u32,
        max_dial: u32,
        rotations_file_path: PathBuf,
    ) -> Self {
        Self {
            current_dial: initial_dial,
            min_dial,
            max_dial,
            rotations_file_path,
        }
    }

    fn rotate_dial(&mut self, direction: TurnDirection, amount: u32) -> u32 {
        let range = self.max_dial - self.min_dial + 1;
        let current = self.current_dial as i32;
        let range = range as i32;
        let min = self.min_dial as i32;

        let new = match direction {
            TurnDirection::Left => current - amount as i32,
            TurnDirection::Right => current + amount as i32,
        };

        (((new - min).rem_euclid(range)) + min) as u32
    }

    fn parse_instruction(instr: &str) -> Option<(TurnDirection, u32)> {
        if instr.is_empty() {
            return None;
        }

        let (dir_char, value_str) = instr.split_at(1);

        let direction = match dir_char {
            "L" => TurnDirection::Left,
            "R" => TurnDirection::Right,
            _ => return None,
        };

        let value = match value_str.parse::<u32>() {
            Ok(v) => v,
            Err(_) => return None,
        };

        Some((direction, value))
    }

    fn apply_instruction(&mut self, instr: &str) {
        if let Some((direction, amount)) = Sequence::parse_instruction(instr) {
            self.current_dial = self.rotate_dial(direction, amount);
        } else {
            println!("Instruction invalide: {}", instr);
        }
    }

    pub fn find_password(&mut self) -> io::Result<u32> {
        let file = File::open(&self.rotations_file_path)?;
        let reader = io::BufReader::new(file);
        let mut result: u32 = 0;

        for line_result in reader.lines() {
            let instr = line_result?;
            self.apply_instruction(&instr);

            if self.current_dial == 0 {
                result += 1;
            }
        }

        Ok(result)
    }
}
