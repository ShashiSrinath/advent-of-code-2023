use crate::util::fs_util::read_lines;

mod util;

fn main() {
    println!("{}", day_2_cube_conundrum());
}

fn day_1_trebuchet() -> i32 {
    let lines = read_lines("./inputs/day_1/values.txt").unwrap();

    let mut calibration_sum = 0;

    for line in lines {
        let line = line.unwrap();
        let digits_vec: Vec<&str> = line.matches(char::is_numeric).collect();
        let calibration_value: i32 = format!(
            "{}{}",
            digits_vec.first().unwrap(),
            digits_vec.last().unwrap()
        )
        .parse()
        .unwrap();
        calibration_sum = calibration_sum + calibration_value;
    }

    return calibration_sum;
}

fn day_1_trebuchet_part2() -> i32 {
    let lines = read_lines("./inputs/day_1/values.txt").unwrap();
    let number_texts = vec![
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ];
    let mut calibration_sum = 0;

    for line in lines {
        let line = line.unwrap();
        let mut digits_vec = line
            .match_indices(char::is_numeric)
            .collect::<Vec<(usize, &str)>>();

        for (number, number_text) in &number_texts {
            line.match_indices(number_text)
                .collect::<Vec<(usize, &str)>>()
                .iter()
                .for_each(|(idx, txt)| digits_vec.push((idx.clone(), number.clone())));
        }

        digits_vec.sort();

        let calibration_value: i32 = format!(
            "{}{}",
            digits_vec.first().unwrap().1,
            digits_vec.last().unwrap().1
        )
        .parse()
        .unwrap();
        calibration_sum = calibration_sum + calibration_value;
    }

    return calibration_sum;
}

fn day_2_cube_conundrum() -> i32 {
    let lines = read_lines("./inputs/day_2/values.txt").unwrap();

    const MAX_RED_CUBES: i32 = 12;
    const MAX_GREEN_CUBES: i32 = 13;
    const MAX_BLUE_CUBES: i32 = 14;

    let mut sum_of_ids = 0;

    for line in lines {
        let line = line.unwrap();

        let (game, inputs) = line.split_once(":").unwrap();

        let (_, game_id) = game.trim().split_once(" ").unwrap();
        let attempts: Vec<&str> = inputs.split(";").collect();

        let mut game_is_possible = true;

        for attempt in attempts {
            let inputs: Vec<&str> = attempt.split(",").collect();

            for input in inputs {
                let (count, color) = input.trim().split_once(" ").unwrap();

                let count = count.parse::<i32>().unwrap();

                let is_valid = match color {
                    "red" => count <= MAX_RED_CUBES,
                    "green" => count <= MAX_GREEN_CUBES,
                    "blue" => count <= MAX_BLUE_CUBES,
                    _ => false,
                };

                if !is_valid {
                    game_is_possible = false;
                    break;
                }
            }

            if !game_is_possible {
                break;
            }
        }

        if game_is_possible {
            sum_of_ids = sum_of_ids + game_id.parse::<i32>().unwrap();
        }
    }

    sum_of_ids
}
