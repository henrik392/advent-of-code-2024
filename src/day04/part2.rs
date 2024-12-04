pub fn solve() -> String {
    let m = super::get_input();

    let mut count = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 'A' && search_cross(&m, i, j) {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn search_cross(m: &[Vec<char>], i: usize, j: usize) -> bool {
    let mut m_count = 0;
    let mut s_count = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 || dy == 0 {
                continue;
            }

            let new_i = i as i32 + dx;
            let new_j = j as i32 + dy;

            if new_i < 0 || new_i >= m.len() as i32 || new_j < 0 || new_j >= m[i].len() as i32 {
                return false;
            }

            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if m[new_i][new_j] == 'M' {
                m_count += 1;
            } else if m[new_i][new_j] == 'S' {
                s_count += 1;
            }
        }
    }

    m[i- 1usize][j- 1usize] != m[i+1][j+1] && m_count == 2 && s_count == 2
}
