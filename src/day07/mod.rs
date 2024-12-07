use crate::day_trait::Day;
mod part1;
mod part2;

pub struct Day07;
impl Day for Day07 {
    fn part1(&self) -> String {
        part1::solve()
    }

    fn part2(&self) -> String {
        part2::solve()
    }
}

type InputData = Vec<(i64, Vec<i64>)>;

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
    str_input
        .lines()
        .map(|line| {
            // first split ':', then split space
            let mut parts = line.split(':').collect::<Vec<&str>>();
            let sum = parts[0].parse::<i64>().unwrap();
            let mut nums = vec![];
            if parts.len() > 1 {
                let nums_str = parts[1].trim().split(' ').collect::<Vec<&str>>();
                for num_str in nums_str {
                    nums.push(num_str.parse::<i64>().unwrap());
                }
            }
            (sum, nums)
        })
        .collect()
}
