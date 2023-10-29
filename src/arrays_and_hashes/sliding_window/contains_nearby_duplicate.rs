use std::collections::HashSet;

/**
 *  Problem 219. Contains Duplicate II (Easy)
 *  See: https://leetcode.com/problems/contains-duplicate-ii/
 *  
 *  Given an integer array nums and an integer k, return true if there
 *  are two distinct indices i and j in the array such that:
 *  
 *  > nums[i] == nums[j] and abs(i - j) <= k.
 */
pub fn run(nums: Vec<i32>, k: i32) -> bool {
    let mut window: HashSet<i32> = HashSet::new();
    let len = nums.len();

    for i in 0..=k as usize {
        // Window is larger than array and all are distinct
        if i == nums.len() { return false; }

        // Already contains duplicate during initialization
        if !window.insert(nums[i]) { return true; }
    }

    // Sliding window
    for i in ((k as usize) + 1)..len {
        window.remove(&nums[i - k as usize - 1]);
        if !window.insert(nums[i]) { return true; }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_is_truthy_if_within_k_size_of_window_are_duplicated() {
        assert!(run(vec![1, 2, 3, 1], 3));
        assert!(run(vec![1, 0, 1, 1], 1));

        assert!(!run(vec![1, 2, 3, 1], 2));
    }
}