use std::collections::HashSet;

pub fn solve() -> String {
    let (m, guard_position) = super::get_input();

    unique_guard_positions(m, guard_position).to_string()
}

fn unique_guard_positions(m: Vec<Vec<bool>>, guard_position: (usize, usize)) -> usize {
    fn get_next_position(
        m: &[Vec<bool>],
        guard_position: &(usize, usize),
        directions: &[(i32, i32)],
        direction: usize,
    ) -> ((usize, usize), usize) {
        let next_position = (
            guard_position.0 as i32 + directions[direction].0,
            guard_position.1 as i32 + directions[direction].1,
        );

        if next_position.0 < 0
            || next_position.0 >= m.len() as i32
            || next_position.1 < 0
            || next_position.1 >= m[0].len() as i32
        {
            (*guard_position, direction)
        } else {
            let next_position = (next_position.0 as usize, next_position.1 as usize);
            if m[next_position.0][next_position.1] {
                get_next_position(m, guard_position, directions, (direction + 1) % 4)
            } else {
                (next_position as (usize, usize), direction)
            }
        }
    }

    let mut guard_position = guard_position;
    // Start up and moves to the right each time.
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction = 0;
    let mut positions = HashSet::new();
    loop {
        positions.insert(guard_position);

        let (next_position, new_direction) =
            get_next_position(&m, &guard_position, &directions, direction);

        if next_position == guard_position {
            break;
        }

        guard_position = next_position;
        direction = new_direction;
    }

    positions.len()
}
