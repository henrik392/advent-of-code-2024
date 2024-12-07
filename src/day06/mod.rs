use crate::day_trait::Day;
mod part1;
mod part2;

pub struct Day06;
impl Day for Day06 {
    fn part1(&self) -> String {
        part1::solve()
    }

    fn part2(&self) -> String {
        part2::solve()
    }
}

type InputData = (Vec<Vec<bool>>, (usize, usize));

#[allow(dead_code)]
pub fn get_input() -> InputData {
    let input = include_str!("input.txt");
    parse_input(input)
}

#[allow(dead_code)]
pub fn test_input() -> InputData {
    let input = include_str!("test.txt");
    parse_input(input)
}

fn parse_input(str_input: &str) -> InputData {
    let mut guard_position = (0, 0);

    for (i, row) in str_input.lines().enumerate() {
        for (j, cell) in row.chars().enumerate() {
            if cell == '^' {
                guard_position = (i, j);
            }
        }
    }

    let map = str_input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    (map, guard_position)
}
