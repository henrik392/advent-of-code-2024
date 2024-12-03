pub fn solve() -> String {
    let input = super::get_input();

    let mut sum: i32 = 0;

    let valid = "mul(%,%)";
    let mut valid_index: usize = 0;

    let mut num1 = String::new();
    let mut num2 = String::new();

    for c in input.chars() {
        match valid_index {
            4 => {
                if num1.is_empty() && !c.is_ascii_digit() || (!c.is_ascii_digit() && c != ',') {
                    num1.clear();
                    num2.clear();
                    valid_index = 0;
                } else if c == ',' {
                    valid_index += 2;
                } else {
                    num1.push(c);
                }
            }
            6 => {
                if num2.is_empty() && !c.is_ascii_digit() || (!c.is_ascii_digit() && c != ')') {
                    num1.clear();
                    num2.clear();
                    valid_index = 0;
                } else if c == ')' {
                    // Calculate and reset
                    sum += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
                    num1.clear();
                    num2.clear();
                    valid_index = 0;
                } else {
                    num2.push(c);
                }
            }
            _ => {
                if let Some(valid_char) = valid.chars().nth(valid_index) {
                    if c == valid_char {
                        valid_index += 1;
                    } else {
                        num1.clear();
                        num2.clear();
                        valid_index = 0;
                    }
                } else {
                    panic!("Invalid valid_index");
                }
            }
        }
    }

    sum.to_string()
}
