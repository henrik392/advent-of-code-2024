pub fn solve() -> String {
    let input = super::get_input();

    let mut res = 0;

    for line in input {
        match non_safe_index(&line) {
            Some(index) => {
                let line1 = line.iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .map(|(_, x)| x.clone())
                    .collect();
                let line2 = line.iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index-1)
                    .map(|(_, x)| x.clone())
                    .collect();

                if non_safe_index(&line1).is_none() || non_safe_index(&line2).is_none() {
                    // println!("{:?} increased", line);
                    res += 1;
                } else {
                    // println!("{:?} decreased", line);
                }
            }
            None => {
                // println!("{:?} increased", line);
                res += 1;
            }
        }
    }

    res.to_string()
}

fn non_safe_index(line: &Vec<i32>) -> Option<usize> {
    let ascending = line[0] < line[line.len()-1];

    for i in 1..line.len() {
        let diff = if ascending {line[i]-line[i-1]} else {line[i-1]-line[i]};
        if diff > 3 || diff <= 0 {
            return Some(i);
        }
    }

    None
}