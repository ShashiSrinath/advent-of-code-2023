use crate::util::fs_util::read_lines;

pub fn day_2_cube_conundrum() -> i32 {
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

pub fn day_2_cube_conundrum_part_2() -> i32 {
    let lines = read_lines("./inputs/day_2/values.txt").unwrap();

    let mut sum_of_powers = 0;

    for line in lines {
        let line = line.unwrap();

        let (_, inputs) = line.split_once(":").unwrap();

        let attempts: Vec<&str> = inputs.split(";").collect();

        let mut max_red_cubes = 0;
        let mut max_green_cubes = 0;
        let mut max_blue_cubes = 0;

        for attempt in attempts {
            let inputs: Vec<&str> = attempt.split(",").collect();

            for input in inputs {
                let (count, color) = input.trim().split_once(" ").unwrap();

                let count = count.parse::<i32>().unwrap();

                match color {
                    "red" => {
                        if count > max_red_cubes {
                            max_red_cubes = count.clone();
                        }
                    }
                    "green" => {
                        if count > max_green_cubes {
                            max_green_cubes = count.clone();
                        }
                    }
                    "blue" => {
                        if count > max_blue_cubes {
                            max_blue_cubes = count.clone();
                        }
                    }
                    _ => {}
                };
            }
        }

        let game_power = max_red_cubes * max_green_cubes * max_blue_cubes;

        sum_of_powers = sum_of_powers + game_power;
    }

    sum_of_powers
}
