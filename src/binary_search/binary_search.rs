use std::cmp::Ordering;

/**
 *  Problem 704. Binary Search (Easy)
 *  See: https://leetcode.com/problems/binary-search/
 *
 *  Given an array of integers nums which is sorted in ascending order,
 *  and an integer target, write a function to search target in nums.
 *
 *  If target exists, then return its index. Otherwise, return -1.
 *
 *  You must write an algorithm with O(log n) runtime complexity.
 *
 */

// This is my original answer, however, this is for searching with
// "non-ascending order", hence it isn't optimized
pub fn run(nums: Vec<i32>, target: i32) -> i32 {
    fn search(nums: &[i32], target: i32, offset: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return -1;
        }
        if len == 1 {
            return if nums[0] == target { offset } else { -1 };
        }

        let split_index = len / 2;
        let left_slice = &nums[0..split_index];
        let right_slice = &nums[split_index..];

        let mut result_left = -1;
        if left_slice.len() != 0 {
            result_left = search(left_slice, target, offset);
        }

        let mut result_right = -1;
        if right_slice.len() != 0 {
            result_right = search(right_slice, target, offset + split_index as i32);
        }

        if result_left != -1 {
            return result_left;
        }
        if result_right != -1 {
            return result_right;
        }

        -1
    }

    search(&*nums, target, 0)
}

// Since the array is already in ascending order, we just need to
// compare one side of it
pub fn run_optimized(nums: Vec<i32>, target: i32) -> i32 {
    let mut lower_bound: usize = 0;
    let mut upper_bound = nums.len();
    let mut mid_index = nums.len() / 2;
    loop {
        let result = nums[mid_index];

        match result.cmp(&target) {
            Ordering::Equal => return mid_index as i32,
            Ordering::Greater => {
                upper_bound = mid_index;
            }
            Ordering::Less => {
                lower_bound = mid_index + 1;
            }
        }

        if lower_bound == upper_bound {
            return -1;
        }

        mid_index = (lower_bound + upper_bound) / 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_searches_target_and_returns_the_index_of_it() {
        assert_eq!(run(vec![-1, 0, 3, 5, 9, 12], 9), 4);

        assert_eq!(run_optimized(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn it_returns_negative_1_if_target_is_missing() {
        assert_eq!(run(vec![-1, 0, 3, 5, 9, 12], 2), -1);

        assert_eq!(run_optimized(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
