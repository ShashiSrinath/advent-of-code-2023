use std::time::Instant;

use crate::days::day_8::day_8_haunted_wasteland;

mod days;
mod util;

fn main() {
    let now = Instant::now();

    println!("{}", day_8_haunted_wasteland());

    println!("Time: {:?}", now.elapsed());
}
