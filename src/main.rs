use std::path::PathBuf;

mod puzzles;

fn main() {
    start_day_1();
}

fn start_day_1() {
    let rotations_file = PathBuf::from("assets/puzzle1/input.txt");
    let password = puzzles::day1::challenge::run(rotations_file);

    match password {
        Some(password) => {
            println!("Password is {}", password);
        }
        None => {
            panic!("An error occured while looking for day1 challenge result: ");
        }
    }
}
