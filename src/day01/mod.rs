use crate::day_trait::Day;
mod part1;
mod part2;

pub struct Day01;
impl Day for Day01 {
    fn part1(&self) -> String {
        part1::solve()
    }

    fn part2(&self) -> String {
        part2::solve()
    }
}

pub fn get_input() -> (Vec<i32>, Vec<i32>) {
    let str_input = include_str!("input.txt");
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in str_input.lines() {
        // There are two numbers for each line
        let mut iter = line.split_whitespace();
        vec1.push(iter.next().unwrap().parse().unwrap());
        vec2.push(iter.next().unwrap().parse().unwrap());
    }

    (vec1, vec2)
}