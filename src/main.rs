use std::time::Instant;

use crate::days::day_7::day_7_camel_cards_part2;

mod days;
mod util;

fn main() {
    let now = Instant::now();

    println!("{}", day_7_camel_cards_part2());

    println!("Time: {:?}", now.elapsed());
}
