#[allow(dead_code)]
pub fn greet(name: &str) -> String {
    format!("Hello, {} how are you doing today?", name)
}

use either::Either;

#[allow(dead_code)]
pub fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    let mut sum: i32 = 0;

    for num in arr.iter() {
        match num {
            Either::Left(x) => sum += x,
            Either::Right(x) => sum += x.parse::<i32>().unwrap(),
        }
    }

    sum
}

#[allow(dead_code)]
pub fn switch_it_up(n: usize) -> &'static str {
    match n {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "Invalid",
    }
}

#[allow(dead_code)]
pub fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    distance_to_pump <= (mpg * gallons)
}

#[allow(dead_code)]
pub fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}
