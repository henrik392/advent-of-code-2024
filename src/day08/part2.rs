use super::{Map, Point};
use std::collections::HashSet;

pub fn solve() -> String {
    let map = super::get_input();
    let mut antinodes: HashSet<Point> = HashSet::new();

    for antennas in map.antenna.values() {
        for antenna1 in antennas {
            for antenna2 in antennas {
                if antenna1 == antenna2 {
                    continue;
                }

                let point_diff = normalized_diff(*antenna1, *antenna2);
                let mut current_point = *antenna1;

                while let Some(antinode) = map.verify_point((
                    current_point.0 as i32 + point_diff.0,
                    current_point.1 as i32 + point_diff.1,
                )) {
                    antinodes.insert(antinode);
                    current_point = antinode;
                }
            }
        }
    }

    print_map(&map, &antinodes);
    antinodes.len().to_string()
}

fn normalized_diff(a: Point, b: Point) -> (i32, i32) {
    let dist = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);
    let gcd = gcd(dist.0.abs(), dist.1.abs());
    (dist.0 / gcd, dist.1 / gcd)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn print_map(map: &Map, antinodes: &HashSet<Point>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let point = (x as u8, y as u8);
            if let Some(freq) = map
                .antenna
                .iter()
                .find(|(_freq, antennas)| antennas.contains(&point))
            {
                print!("{}", freq.0);
            } else if antinodes.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
