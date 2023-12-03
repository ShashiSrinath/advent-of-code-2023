use crate::util::fs_util::read_lines;

pub fn day_1_trebuchet() -> i32 {
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

pub fn day_1_trebuchet_part2() -> i32 {
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
