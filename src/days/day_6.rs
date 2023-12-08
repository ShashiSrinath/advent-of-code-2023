use crate::util::fs_util::read_lines;

#[derive(Debug)]
struct Race {
    time: i64,
    record_distance: i64,
}

impl Race {
    pub fn get_winning_chances(&self) -> i64 {
        let mut winning_chances = 0;

        for i in 0..self.time {
            // i is same as speed
            let remaining_time = self.time - i;

            let total_distance = i * remaining_time;

            if total_distance > self.record_distance {
                winning_chances = winning_chances + 1;
            }
        }

        winning_chances
    }
}

pub fn day_6_boat_race() -> i64 {
    let lines = read_lines("./inputs/day_6/values.txt").unwrap();

    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();

        if i == 0 {
            times = line
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| v.parse().unwrap())
                .collect();
        }

        if i == 1 {
            distances = line
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| v.parse().unwrap())
                .collect();
        }
    }

    let mut winning_chance_product = 1;

    for i in 0..times.len() {
        let race = Race {
            time: times[i],
            record_distance: distances[i],
        };

        winning_chance_product = winning_chance_product * race.get_winning_chances();
    }

    winning_chance_product
}

pub fn day_6_boat_race_part_2() -> i64 {
    let lines = read_lines("./inputs/day_6/values.txt").unwrap();

    let mut time: String = "".to_string();
    let mut distance: String = "".to_string();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();

        if i == 0 {
            time = line.split_once(":").unwrap().1.trim().replace(" ", "");
        }

        if i == 1 {
            distance = line.split_once(":").unwrap().1.trim().replace(" ", "");
        }
    }

    let race = Race {
        time: time.parse().unwrap(),
        record_distance: distance.parse().unwrap(),
    };

    return race.get_winning_chances();
}
