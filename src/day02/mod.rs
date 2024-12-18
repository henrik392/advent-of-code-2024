use crate::day_trait::Day;
mod part1;
mod part2;

pub struct Day02;
impl Day for Day02 {
    fn part1(&self) -> String {
        part1::solve()
    }

    fn part2(&self) -> String {
        part2::solve()
    }
}

pub fn get_input() -> Vec<Vec<i32>> {
    let str_input = include_str!("input.txt");
    let mut data = Vec::new();

    for line in str_input.lines() {
        // There are two numbers for each line
        data.push(line.split_whitespace().map(|x| x.parse().unwrap()).collect());
    }

    data
}