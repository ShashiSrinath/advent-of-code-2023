use crate::util::fs_util::read_lines;

pub fn day_4_scratchcards() -> i32 {
    let lines = read_lines("./inputs/day_4/values.txt").unwrap();

    let mut totalPoints = -0;

    for line in lines {
        let line = line.unwrap();

        let (_, inputs) = line.split_once(":").unwrap();

        let (winning, owned) = inputs.split_once("|").unwrap();

        let mut currentCardPoints = 0;

        let winning: Vec<i32> = winning
            .trim()
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap())
            .collect();

        let owned: Vec<i32> = owned
            .trim()
            .split(" ")
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap())
            .collect();

        for n in &owned {
            if winning.contains(&n) {
                if currentCardPoints == 0 {
                    currentCardPoints = 1;
                } else {
                    currentCardPoints = currentCardPoints * 2;
                }
            }
        }

        totalPoints = totalPoints + currentCardPoints;
    }

    totalPoints
}
