#[allow(dead_code)]
pub fn find_next_perfect_square(sq: u64) -> Option<u64> {
    let sqrt = (sq as f64).sqrt();

    if sqrt % 1.0 == 0.0 {
        return Some((sqrt as u64 + 1).pow(2));
    }

    None
}

#[allow(dead_code)]
pub fn high_and_low(numbers: &str) -> String {
    let arr: Vec<i32> = numbers
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let max: i32 = *arr.iter().max().unwrap();
    let min: i32 = *arr.iter().min().unwrap();

    format!("{} {}", max, min)
}
