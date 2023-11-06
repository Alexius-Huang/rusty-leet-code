use std::{collections::HashMap, cmp::Ordering};

/**
 *  Problem 167. Two Sum II with Non-Decreasing Sorted Input Array
 *  See: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
 *
 *  Given a 1-indexed array of integers numbers that is already sorted in
 *  non-decreasing order, find two numbers such that they add up to a
 *  specific target number.
 *
 *  Let these two numbers be numbers[index1] and numbers[index2] where:
 *  1 <= index1 < index2 < numbers.length.
 *
 *  Return the indices of the two numbers, `index1` and `index2`, added
 *  by one as an integer array [index1, index2] of length 2.
 *  
 *  The tests are generated such that there is exactly one solution.
 *
 *  You may not use the same element twice.
 *  
 *  Your solution must use only "constant extra space".
 */
// Normal two sum using O(n) additional hash
pub fn run(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hmap: HashMap<i32, i32> = HashMap::new();

    for (index, num) in numbers.iter().enumerate() {
        let seek = target - num;

        if let Some(i) = hmap.get(&seek) {
            return vec![*i + 1, index as i32 + 1];
        } else {
            hmap.insert(*num, index as i32);
        }
    }

    vec![]
}

pub fn run_no_additional_space(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    let has_negative = nums[0] < 0;
    let offset = if has_negative { -nums[0] } else { 0 };
    let target = if has_negative { target + 2 * offset } else { target };

    let (mut l_index, mut r_index): (usize, usize) = (0, len - 1);

    while nums[r_index] + offset > target { r_index -= 1; }

    let mut seek = target - (nums[r_index] + offset);

    while l_index != r_index {
        let num = nums[l_index] + offset;

        match i32::cmp(&num, &seek) {
            Ordering::Equal => return vec![l_index as i32 + 1, r_index as i32 + 1],
            Ordering::Less => { l_index += 1; },
            Ordering::Greater => {
                r_index -=1;
                seek = target - (nums[r_index] + offset);
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_indices_sums_up_to_target() {
        assert_eq!(run(vec![2, 7, 11, 13], 9), vec![1, 2]);
        assert_eq!(run(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(run(vec![-1, 0], -1), vec![1, 2]);

        assert_eq!(run_no_additional_space(vec![2, 7, 11, 13], 9), vec![1, 2]);
        assert_eq!(run_no_additional_space(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(run_no_additional_space(vec![-4, 1, 2, 4], -2), vec![1, 3]);
        assert_eq!(run_no_additional_space(vec![-1, 0], -1), vec![1, 2]);
    }
}
