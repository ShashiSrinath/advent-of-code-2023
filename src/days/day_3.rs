use crate::util::fs_util::read_lines;

#[derive(Debug, Clone)]
pub struct PartCoords {
    x: i32,
    y: i32,
    x2: i32,
}

#[derive(Debug, Clone)]
struct PartInput {
    value: i32,
    coords: PartCoords,
}

#[derive(Debug)]
struct GearInput {
    x: i32,
    y: i32,
}

impl PartInput {
    pub fn is_valid(&self, matrix: &Vec<Vec<char>>) -> bool {
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

    pub fn is_in_gear_coords(&self, (x, y): (i32, i32)) -> bool {
        if self.coords.y >= y - 1 && self.coords.y <= y + 1 {
            if self.coords.x >= x - 1 && self.coords.x <= x + 1 {
                return true;
            }
            if self.coords.x2 >= x - 1 && self.coords.x2 <= x + 1 {
                return true;
            }
        }

        false
    }
}

impl GearInput {
    pub fn validate(&self, parts: &Vec<PartInput>) -> (bool, Vec<PartInput>) {
        let mut adj_parts: Vec<PartInput> = Vec::new();

        for part in parts {
            if part.is_in_gear_coords((self.x, self.y)) {
                adj_parts.push(part.clone());
            }
        }

        if adj_parts.len() == 2 {
            (true, adj_parts)
        } else {
            (false, adj_parts)
        }
    }
}

pub fn day_3_gear_ratios() -> i32 {
    let lines = read_lines("./inputs/day_3/values.txt").unwrap();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut current_vec: Vec<char> = Vec::new();

        line.chars().for_each(|c| current_vec.push(c));

        matrix.push(current_vec);
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

                    // reset values.txt
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

            // reset values.txt
            start_index = -1;
            end_index = -1;
            value = Vec::new();
        }
    }

    let mut sum = 0;
    for part in parts {
        if part.is_valid(&matrix) {
            sum = sum + part.value;
        }
    }

    sum
}

pub fn day_3_gear_ratios_part_2() -> i32 {
    let lines = read_lines("./inputs/day_3/values.txt").unwrap();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut current_vec: Vec<char> = Vec::new();

        line.chars().for_each(|c| current_vec.push(c));

        matrix.push(current_vec);
    }

    let mut parts: Vec<PartInput> = Vec::new();
    let mut gears: Vec<GearInput> = Vec::new();

    for (y, vec_x) in matrix.iter().enumerate() {
        let mut start_index: i32 = -1;
        let mut end_index: i32 = -1;
        let mut value: Vec<char> = Vec::new();

        for (x, current_char) in vec_x.iter().enumerate() {
            if current_char == &'*' {
                gears.push(GearInput {
                    x: x as i32,
                    y: y as i32,
                })
            }

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

                    // reset values.txt
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

            // reset values.txt
            start_index = -1;
            end_index = -1;
            value = Vec::new();
        }
    }

    let mut valid_parts: Vec<PartInput> = Vec::new();
    for part in parts {
        if part.is_valid(&matrix) {
            valid_parts.push(part);
        }
    }

    let mut gear_ratio_sum = 0;
    for gear in gears {
        let (is_valid, adj_parts) = gear.validate(&valid_parts);
        if is_valid {
            let gear_ratio = adj_parts.first().unwrap().value * adj_parts.last().unwrap().value;

            gear_ratio_sum = gear_ratio_sum + gear_ratio;
        }
    }

    gear_ratio_sum
}
