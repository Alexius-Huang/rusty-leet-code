/**
 *  Problem 52. Maximum Subarray (Medium)
 * 
 *  Given an integer array nums, find the subarray with the largest
 *  sum, and return its sum.
 * 
 */
pub fn run(nums: Vec<i32>) -> i32 {
    let mut result = nums[0];
    let mut current_max = nums[0];

    for i in 1..nums.len() {
        let num = nums[i];
        let current_sum = current_max + num;

        current_max = if current_sum > num {
            current_sum
        } else {
            num
        };

        if current_max > result {
            result = current_max;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_maximum_value_of_subarray() {
        assert_eq!(run(
            vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
        ), 6);

        assert_eq!(run(vec![1]), 1);

        assert_eq!(run(vec![5, 4, -1, 7, 8]), 23);
    }
}