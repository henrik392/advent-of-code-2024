use crate::day_trait::Day;
mod part1;
mod part2;

pub struct Day04;
impl Day for Day04 {
    fn part1(&self) -> String {
        part1::solve()
    }

    fn part2(&self) -> String {
        part2::solve()
    }
}

pub fn get_input() -> Vec<Vec<char>> {
    let str_input = include_str!("input.txt");

    str_input.lines().map(|x| x.chars().collect()).collect()
}
