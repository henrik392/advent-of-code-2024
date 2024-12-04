const SEARCH_WORD: &str = "XMAS";

pub fn solve() -> String {
    let m = super::get_input();

    let mut count = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 'X' {
                count += search_word(&m, i, j);
            }
        }
    }

    count.to_string()
}

fn search_word(m: &[Vec<char>], i: usize, j: usize) -> i32 {
    let mut count = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if search_word_in_direction(m, i, j, dx, dy) {
                count += 1;
            }
        }
    }

    count
}

fn search_word_in_direction(m: &[Vec<char>], i: usize, j: usize, dx: i32, dy: i32) -> bool {
    for k in 0..SEARCH_WORD.len() {
        let new_i = i as i32 + k as i32 * dx;
        let new_j = j as i32 + k as i32 * dy;
        if new_i < 0 || new_i >= m.len() as i32 || new_j < 0 || new_j >= m[i].len() as i32 {
            return false;
        }

        let new_i = new_i as usize;
        let new_j = new_j as usize;

        if m[new_i][new_j] != SEARCH_WORD.chars().nth(k).unwrap() {
            return false;
        }
    }

    true
}
