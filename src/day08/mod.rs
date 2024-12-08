use crate::day_trait::Day;
use std::collections::HashMap;
mod part1;
mod part2;

pub struct Day08;
impl Day for Day08 {
    fn part1(&self) -> String {
        part1::solve()
    }

    fn part2(&self) -> String {
        part2::solve()
    }
}

pub type Point = (u8, u8);

pub struct Map {
    width: usize,
    height: usize,
    pub antenna: HashMap<char, Vec<Point>>,
}

impl Map {
    pub fn new(width: usize, height: usize, antenna: HashMap<char, Vec<Point>>) -> Self {
        Self {
            width,
            height,
            antenna,
        }
    }

    pub fn verify_point(&self, point: (i32, i32)) -> Option<Point> {
        if self.point_within_map(point) {
            Some((point.0 as u8, point.1 as u8))
        } else {
            None
        }
    }

    fn point_within_map(&self, point: (i32, i32)) -> bool {
        let (x, y) = point;
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }
}

#[allow(dead_code)]
pub fn get_input() -> Map {
    let input = include_str!("input.txt");
    parse_input(input)
}

#[allow(dead_code)]
pub fn test_input() -> Map {
    let input = include_str!("test.txt");
    parse_input(input)
}

fn parse_input(str_input: &str) -> Map {
    let mut antenna = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in str_input.lines().enumerate() {
        height = y + 1;
        for (x, c) in line.chars().enumerate() {
            width = x + 1;
            if c != '.' {
                let entry = antenna.entry(c).or_insert(Vec::new());
                entry.push((x as u8, y as u8));
            }
        }
    }

    Map::new(width, height, antenna)
}
