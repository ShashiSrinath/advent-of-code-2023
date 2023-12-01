use std::ops::Add;
use crate::util::fs_util::read_lines;

mod util;

fn main() {
    println!("{}", day_1_trebuchet());
}

fn day_1_trebuchet() -> i32 {
    let lines = read_lines("./inputs/day_1/values.txt").unwrap();

    let mut  calibration_sum = 0;

    for line in lines {
        let line = line.unwrap();
        let digits_vec: Vec<&str> = line.matches(char::is_numeric).collect();
        let calibration_value: i32 = format!("{}{}", digits_vec.first().unwrap(), digits_vec.last().unwrap()).parse().unwrap();
        calibration_sum =  calibration_sum + calibration_value;
    }

    return calibration_sum;
}
