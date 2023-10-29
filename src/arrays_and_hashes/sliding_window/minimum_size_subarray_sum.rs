/**
 *  Problem 209. Minimum Size Subarray Sum
 *  See: https://leetcode.com/problems/minimum-size-subarray-sum/
 * 
 *  Given an array of positive integers nums and a positive integer target,
 *  return the minimal length of a subarray whose sum is greater than
 *  or equal to target.
 * 
 *  If there is no such subarray, return 0 instead.
 */
pub fn run(target: i32, nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut result = len + 1;
    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    let mut windowed_sum = nums[0];
    let mut has_window = false;

    loop {
        // Decreasing window size continuously if windowed sum >= target
        if windowed_sum >= target {
            let window_size = right_index - left_index;
            if window_size == 0 { return 1; }
            result = window_size + 1;
            has_window = true;

            windowed_sum -= nums[left_index];
            left_index += 1;
            continue;
        }

        right_index += 1;
        if right_index == len { break; }
        windowed_sum += nums[right_index];

        if has_window {
            windowed_sum -= nums[left_index];
            left_index += 1;
        }
    }

    if result == len + 1 { 0 } else { result as i32 } 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_minimum_size_of_subarray_whose_sum_is_greater_than_or_equal_to_target() {
        assert_eq!(run(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(run(4, vec![1, 4, 4]), 1);
        assert_eq!(run(11, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}
