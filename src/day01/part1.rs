pub(crate) fn solve() -> i32 {
    let (mut vec1, mut vec2) = super::get_input();
    vec1.sort();
    vec2.sort();

    let mut sum = 0;
    for i in 0..vec1.len() {
        sum += (vec1[i] - vec2[i]).abs();
    }

    sum
}