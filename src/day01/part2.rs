use std::collections::HashMap;

pub fn solve() -> i32 {
    let (mut vec1, mut vec2) = super::get_input();
    vec1.sort();
    vec2.sort();

    let mut freq_map = HashMap::new();
    for num in vec2 {
        freq_map.insert(num, freq_map.get(&num).unwrap_or(&0) + 1);
    }

    let mut sum = 0;
    for i in 0..vec1.len() {
        sum += vec1[i]*freq_map.get(&vec1[i]).unwrap_or(&0);
    }

    sum
}