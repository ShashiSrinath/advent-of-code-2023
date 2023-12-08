use crate::util::fs_util::read_lines;
use std::collections::HashMap;

fn get_card_numbers(str: &str) -> Vec<i32> {
    str.trim()
        .split(" ")
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
        .collect()
}

pub fn day_4_scratchcards() -> i32 {
    let lines = read_lines("./inputs/day_4/values.txt").unwrap();

    let mut total_points = -0;

    for line in lines {
        let line = line.unwrap();

        let (winning, owned) = line.split_once(":").unwrap().1.split_once("|").unwrap();

        let mut current_card_points = 0;

        let winning: Vec<i32> = get_card_numbers(&winning);
        let owned: Vec<i32> = get_card_numbers(&owned);

        for n in &owned {
            if winning.contains(&n) {
                if current_card_points == 0 {
                    current_card_points = 1;
                } else {
                    current_card_points = current_card_points * 2;
                }
            }
        }

        total_points = total_points + current_card_points;
    }

    total_points
}

pub fn day_4_scratchcards_part2() -> i32 {
    let lines = read_lines("./inputs/day_4/values.txt").unwrap();

    let mut total_points = 0;
    let mut card_count: HashMap<i32, i32> = HashMap::new();

    for (index, line) in lines.enumerate() {
        let line = line.unwrap();

        match card_count.get(&(index as i32)) {
            None => card_count.insert(index as i32, 1),
            Some(count) => card_count.insert(index as i32, count + 1),
        };

        let (winning, owned) = line.split_once(":").unwrap().1.split_once("|").unwrap();

        let winning: Vec<i32> = get_card_numbers(&winning);
        let owned: Vec<i32> = get_card_numbers(&owned);

        let mut no_of_wins = 0;

        for _ in 0..*card_count.get(&(index as i32)).unwrap() {
            let mut next_card_index = index as i32 + 1;
            for n in &owned {
                if winning.contains(&n) {
                    match card_count.get(&next_card_index) {
                        None => card_count.insert(next_card_index, 1),
                        Some(count) => card_count.insert(next_card_index, count + 1),
                    };

                    no_of_wins = no_of_wins + 1;

                    next_card_index = next_card_index + 1;
                }
            }
        }

        total_points = total_points + card_count.get(&(index as i32)).unwrap();
    }

    total_points
}
