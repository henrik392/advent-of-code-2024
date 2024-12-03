use crate::day_trait::Day;
mod part1;
mod part2;

pub struct Day03;
impl Day for Day03 {
    fn part1(&self) -> String {
        part1::solve()
    }

    fn part2(&self) -> String {
        part2::solve()
    }
}

pub fn get_input() -> String {
    let str_input = include_str!("input.txt");

    str_input.to_owned()
}