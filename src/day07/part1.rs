pub fn solve() -> String {
    let equations = super::test_input();
    let mut correct_sum = 0;
    for equation in equations {
        let expected_sum = equation.0;
        let nums = equation.1;

        if dp(expected_sum, nums[0], nums[0], &nums, 1) {
            correct_sum += expected_sum;
        }
    }

    correct_sum.to_string()
}

fn dp(expected_sum: i32, sum: i32, prev_part: i32, nums: &Vec<i32>, index: usize) -> bool {
    if index == nums.len() {
        return expected_sum == sum;
    }

    dp(
        expected_sum,
        sum + nums[index],
        nums[index],
        nums,
        index + 1,
    ) || dp(
        expected_sum,
        sum + prev_part * (nums[index]-1),
        prev_part * nums[index],
        nums,
        index + 1,
    )
}
