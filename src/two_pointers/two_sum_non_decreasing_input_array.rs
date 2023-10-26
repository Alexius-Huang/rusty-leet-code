use std::collections::HashMap;

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

// TODO: confirm if this is invalid answer as it doesn't use constant
//       extra space?
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_indices_sums_up_to_target() {
        assert_eq!(run(vec![2, 7, 11, 13], 9), vec![1, 2]);
        assert_eq!(run(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(run(vec![-1, 0], -1), vec![1, 2]);
    }
}