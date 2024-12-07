use std::collections::HashSet;

pub fn solve() -> String {
    let (m, guard_position) = super::get_input();

    num_potential_blocks(m, guard_position).to_string()
}

fn num_potential_blocks(m: Vec<Vec<bool>>, guard_position: (usize, usize)) -> usize {
    let mut guard_position = guard_position;
    let map = create_map_function(&m, None);
    // Start up and moves to the right each time.
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction = 0;
    let mut potential_block = HashSet::new();
    loop {
        let (next_position, new_direction) =
            get_next_position(&map, &guard_position, &directions, direction);

        if new_direction == 5 {
            break;
        }

        if !potential_block.contains(&next_position)
            && block_cause_loop(
                &create_map_function(&m, Some(next_position)),
                guard_position,
                direction,
                &directions,
                // &visited_states,
            )
        {
            potential_block.insert(next_position);
        }

        guard_position = next_position;
        direction = new_direction;
    }

    potential_block.len()
}

fn block_cause_loop<'a>(
    map: &MapFunction<'a>,
    guard_position: (usize, usize),
    direction: usize,
    directions: &[(i32, i32)],
    // visited_states: &std::collections::HashSet<((usize, usize), usize)>,
) -> bool {
    let mut guard_position = guard_position;

    // let mut visited_states = visited_states.clone();
    let mut visited_states = HashSet::new();
    let mut direction = direction;

    loop {
        let (next_position, new_direction) =
            get_next_position(&map, &guard_position, directions, direction);

        if new_direction == 5 {
            return false;
        }

        if visited_states.contains(&(next_position, new_direction)) {
            return true;
        }
        visited_states.insert((next_position, new_direction));

        guard_position = next_position;
        direction = new_direction;
    }
}

fn get_next_position<'a>(
    map: &MapFunction<'a>,
    guard_position: &(usize, usize),
    directions: &[(i32, i32)],
    direction: usize,
) -> ((usize, usize), usize) {
    let next_position = (
        guard_position.0 as i32 + directions[direction].0,
        guard_position.1 as i32 + directions[direction].1,
    );

    let next_tile = map(next_position);

    match next_tile {
        MapTile::Wall => {
            let next_direction = (direction + 1) % 4;
            get_next_position(map, guard_position, directions, next_direction)
        }
        MapTile::Empty => {
            let next_position = (next_position.0 as usize, next_position.1 as usize);
            (next_position, direction)
        }
        MapTile::Outside => (*guard_position, 5),
    }
}

enum MapTile {
    Wall,
    Empty,
    Outside,
}

type MapFunction<'a> = dyn Fn((i32, i32)) -> MapTile + 'a;

fn create_map_function<'a>(
    m: &'a [Vec<bool>],
    block_position: Option<(usize, usize)>,
) -> Box<MapFunction<'a>> {
    Box::new(move |pos: (i32, i32)| {
        if pos.0 < 0 || pos.0 >= m.len() as i32 || pos.1 < 0 || pos.1 >= m[0].len() as i32 {
            return MapTile::Outside;
        }

        let pos = (pos.0 as usize, pos.1 as usize);

        let mut is_wall = m[pos.0][pos.1];
        if let Some(block_position) = block_position {
            is_wall = is_wall || pos == block_position;
        }

        if is_wall {
            MapTile::Wall
        } else {
            MapTile::Empty
        }
    })
}

// 1752 is wrongk
