pub fn solve() -> String {
    let equations = super::get_input();
    let mut correct_sum = 0;
    for equation in equations {
        let expected_sum = equation.0;
        let nums = equation.1;

        if dp(expected_sum, nums[0], &nums, 1) {
            correct_sum += expected_sum;
        }
    }

    correct_sum.to_string()
}

fn dp(expected_sum: i64, sum: i64, nums: &Vec<i64>, index: usize) -> bool {
    if index == nums.len() {
        return expected_sum == sum;
    }

    dp(expected_sum, sum + nums[index], nums, index + 1)
        || dp(expected_sum, sum * nums[index], nums, index + 1)
        || dp(expected_sum, concat(sum, nums[index]), nums, index + 1)
}

fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}
