use std::collections::HashSet;

/**
 *  Problem 26. Remove Duplicates from Sorted Array (easy)
 *  See: https://leetcode.com/problems/remove-duplicates-from-sorted-array/
 *
 *  Given an integer array nums sorted in non-decreasing order,
 *  remove the duplicates in-place such that each unique element appears only once.
 *
 *  The relative order of the elements should be kept the same.
 *
 *  Then return the number of unique elements in nums.
 *
 *  Consider the number of unique elements of nums to be `k`, to get accepted,
 *  you need to do the following things:
 *
 *  - Change the array nums such that the first `k` elements of nums contain
 *    the unique elements in the order they were present in nums initially.
 *  - The remaining elements of nums are not important as well as the size of nums.
 *  - Return k.
 *
 */
pub fn run(nums: &mut Vec<i32>) -> i32 {
    let mut unique_count: i32 = 0;
    let mut set: HashSet<i32> = HashSet::new();

    for num in nums.clone().iter() {
        if set.contains(num) {
            continue;
        }
        set.insert(*num);
        nums[unique_count as usize] = *num;
        unique_count += 1;
    }

    nums.drain((unique_count as usize)..);

    unique_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_removes_duplicate_and_returns_unique_element_count() {
        let mut input = vec![1, 2, 3, 2, 1];
        let unique_elements_count = run(&mut input);

        assert_eq!(input, vec![1, 2, 3]);
        assert_eq!(unique_elements_count, 3);

        let mut input = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
        let unique_elements_count = run(&mut input);

        assert_eq!(input, vec![1, 2, 3, 4]);
        assert_eq!(unique_elements_count, 4);

        let mut input = vec![3, 1, 2, 3, 1, 5, 3, 7, 2, 3, 4, 1];
        let unique_elements_count = run(&mut input);

        assert_eq!(input, vec![3, 1, 2, 5, 7, 4]);
        assert_eq!(unique_elements_count, 6);
    }
}
