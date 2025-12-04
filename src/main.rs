use std::path::PathBuf;

mod puzzles;
mod utils;

fn main() {
    start_day_2();
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

fn start_day_2() {
    let ranges_file = PathBuf::from("assets/puzzle2/input.txt");
    let mut challenge = puzzles::day2::challenge::Challenge::new(ranges_file);
    match challenge.solve() {
        Some(answer) => println!("Challenge answer is {}", answer),
        None => println!("Nothing found ! :("),
    };
}
