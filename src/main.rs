use std::time::Instant;

use crate::days::day_6::day_6_boat_race_part_2;

mod days;
mod util;

fn main() {
    let now = Instant::now();

    println!("{}", day_6_boat_race_part_2());

    println!("Time: {:?}", now.elapsed());
}
