use std::collections::HashMap;

use crate::util::fs_util::read_lines;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    name: String,
    left_name: String,
    right_name: String,
    is_end: bool,
    is_start: bool,
}

pub fn day_8_haunted_wasteland() -> i32 {
    let lines = read_lines("./inputs/day_8/values.txt").unwrap();
    let mut directions: Vec<Direction> = Vec::new();
    let mut node_map: HashMap<String, Node> = HashMap::new();

    let mut starting_node_name: Option<String> = None;

    for (line) in lines {
        let line = line.unwrap();

        if directions.is_empty() {
            line.split("").for_each(|s| {
                if s == "L" {
                    directions.push(Direction::Left)
                }

                if s == "R" {
                    directions.push(Direction::Right)
                }
            });

            continue;
        }

        if line.is_empty() {
            continue;
        }

        let (node_str, node_data) = line.split_once("=").unwrap();
        let (left, right) = node_data.split_once(",").unwrap();

        let node_str = node_str.trim();
        let left = left.trim().strip_prefix("(").unwrap().trim();
        let right = right.trim().strip_suffix(")").unwrap().trim();

        let node = Node {
            left_name: left.to_string(),
            right_name: right.to_string(),
            is_end: node_str == "ZZZ",
            is_start: node_str == "AAA",
            name: node_str.to_string(),
        };

        if node.is_start {
            starting_node_name = Some(node_str.to_string());
        }

        node_map.insert(node_str.to_string(), node);
    }

    let mut current_node = node_map.get(&starting_node_name.unwrap()).unwrap();
    let mut direction_index = 0;
    let mut step_counter = 0;
    loop {
        let next_direction = directions.get(direction_index).unwrap();
        match next_direction {
            Direction::Left => {
                current_node = node_map.get(&current_node.left_name).unwrap();
            }
            Direction::Right => {
                current_node = node_map.get(&current_node.right_name).unwrap();
            }
        }
        step_counter = step_counter + 1;

        if current_node.is_end {
            break;
        }

        if directions.len() > direction_index + 1 {
            direction_index = direction_index + 1;
        } else {
            direction_index = 0;
        }
    }

    step_counter
}
