use crate::day_trait::Day;
mod part1;
mod part2;

pub struct Day05;
impl Day for Day05 {
    fn part1(&self) -> String {
        part1::solve()
    }

    fn part2(&self) -> String {
        part2::solve()
    }
}

type InputData = (Vec<(i32, i32)>, Vec<Vec<i32>>);

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
    let lines: Vec<&str> = str_input.lines().collect();
    let mut page_rules: Vec<(i32, i32)> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];

    let mut index = 0;

    // First loop to process page rules
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            index = i + 1;
            break;
        }

        let mut parts = line.split("|");
        let first: i32 = parts.next().unwrap().parse().unwrap();
        let second: i32 = parts.next().unwrap().parse().unwrap();
        page_rules.push((first, second));
    }

    // Second loop to process updates, starting from where the first loop left off
    for line in &lines[index..] {
        let parts = line.split(",").map(|x| x.parse().unwrap()).collect();
        updates.push(parts);
    }

    (page_rules, updates)
}
