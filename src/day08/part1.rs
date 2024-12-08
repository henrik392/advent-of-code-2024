use super::Point;
use std::collections::HashSet;

pub fn solve() -> String {
    let map = super::get_input();
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (freq, antennas) in &map.antenna {
        for antenna1 in antennas {
            for antenna2 in antennas {
                if antenna1 == antenna2 {
                    continue;
                }
                let point_diff: (i32, i32) = (
                    antenna2.0 as i32 - antenna1.0 as i32,
                    antenna2.1 as i32 - antenna1.1 as i32,
                );
                let antinode_pos = (
                    antenna2.0 as i32 + point_diff.0,
                    antenna2.1 as i32 + point_diff.1,
                );

                if let Some(antinode) = map.verify_point(antinode_pos) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len().to_string()
}
