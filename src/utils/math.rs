pub fn digits(number: u64) -> usize {
    number.to_string().len()
}

pub fn is_pair(number: usize) -> bool {
    number % 2 == 0
}
