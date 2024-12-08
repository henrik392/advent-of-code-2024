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

                if !map.point_within_map(antinode_pos) {
                    continue;
                }

                let antinode_pos = (antinode_pos.0 as u8, antinode_pos.1 as u8);
                antinodes.insert(antinode_pos);
            }
        }
    }

    antinodes.len().to_string()
}
