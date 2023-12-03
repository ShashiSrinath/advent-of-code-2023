use crate::util::fs_util::read_lines;

mod util;

fn main() {
    println!("{}", day_3_gear_ratios());
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

fn day_2_cube_conundrum_part_2() -> i32 {
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

fn day_3_gear_ratios() -> i32 {
    let lines = read_lines("./inputs/day_3/values.txt").unwrap();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut current_vec: Vec<char> = Vec::new();

        line.chars().for_each(|c| current_vec.push(c));

        matrix.push(current_vec);
    }

    #[derive(Debug)]
    pub struct PartCoords {
        x: i32,
        y: i32,
        x2: i32,
    }

    #[derive(Debug)]
    struct PartInput {
        value: i32,
        coords: PartCoords,
    }

    impl PartInput {
        pub fn is_a_part(&self, matrix: &Vec<Vec<char>>) -> bool {
            let max_y = matrix.len() - 1;
            let max_x = matrix.get(0).expect("Invalid input matrix").len() - 1;

            let mut coordinate_matrix: Vec<(i32, i32)> = Vec::new();

            let top_row = self.coords.y - 1;
            let bottom_row = self.coords.y + 1;

            if top_row >= 0 {
                for x in (self.coords.x - 1)..(self.coords.x2 + 2) {
                    if x >= 0 && x <= max_x as i32 {
                        coordinate_matrix.push((x, top_row));
                    }
                }
            }

            if self.coords.y >= 0 {
                if self.coords.x - 1 >= 0 {
                    coordinate_matrix.push((self.coords.x - 1, self.coords.y))
                }

                if self.coords.x2 + 1 <= max_x as i32 {
                    coordinate_matrix.push((self.coords.x2 + 1, self.coords.y))
                }
            }

            if bottom_row <= max_y as i32 {
                for x in (self.coords.x - 1)..(self.coords.x2 + 2) {
                    if x >= 0 && x <= max_x as i32 {
                        coordinate_matrix.push((x, bottom_row));
                    }
                }
            }

            for (coord_x, coord_y) in coordinate_matrix {
                let element = matrix
                    .get(coord_y as usize)
                    .unwrap()
                    .get(coord_x as usize)
                    .unwrap();

                if !element.is_numeric() && !element.eq(&'.') {
                    return true;
                }
            }

            return false;
        }
    }

    let mut parts: Vec<PartInput> = Vec::new();

    for (y, vec_x) in matrix.iter().enumerate() {
        let mut start_index: i32 = -1;
        let mut end_index: i32 = -1;
        let mut value: Vec<char> = Vec::new();

        for (x, current_char) in vec_x.iter().enumerate() {
            if current_char.is_numeric() {
                if start_index < 0 {
                    start_index = x as i32;
                }
                value.push(current_char.clone());
                end_index = x as i32;
            } else if !current_char.is_numeric() || x >= vec_x.len() - 1 {
                if start_index >= 0 {
                    parts.push(PartInput {
                        coords: PartCoords {
                            x: start_index,
                            x2: end_index,
                            y: y as i32,
                        },
                        value: value.iter().collect::<String>().parse().unwrap(),
                    });

                    // reset values
                    start_index = -1;
                    end_index = -1;
                    value = Vec::new();
                }
            }
        }

        if start_index >= 0 {
            parts.push(PartInput {
                coords: PartCoords {
                    x: start_index,
                    x2: end_index,
                    y: y as i32,
                },
                value: value.iter().collect::<String>().parse().unwrap(),
            });

            // reset values
            start_index = -1;
            end_index = -1;
            value = Vec::new();
        }
    }

    let mut sum = 0;
    for part in parts {
        if part.is_a_part(&matrix) {
            sum = sum + part.value;
        }
    }

    sum
}
