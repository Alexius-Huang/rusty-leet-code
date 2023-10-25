/**
 *  Problem 27. Remove Element
 *  See: https://leetcode.com/problems/remove-element/
 * 
 *  Given an integer array `nums` and an integer `val`, remove all
 *  occurrences of `val` in `nums` in-place.
 * 
 *  The order of the elements may be changed.
 * 
 *  Then return the number of elements in `nums` which are not equal to `val`.
 * 
 *  Consider the number of elements in `nums` which are not equal
 *  to `val` be `k`, to get accepted, you need to do the following things:
 *  
 *  Change the array nums such that the first k elements of nums contain
 *  the elements which are not equal to val.
 * 
 *  The remaining elements of nums are not important as well as the size
 *  of nums. Lastly, return `k`.
 * 
 */
pub fn run(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut removed_count: i32 = 0;
    let length = nums.len();

    for (index, num) in nums.clone().iter().enumerate() {
        if *num != val { continue; }

        nums.swap(removed_count as usize, index);
        removed_count += 1;
    }

    let rest = length as i32 - removed_count;
    nums.drain(..(removed_count as usize));

    rest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_removes_particular_element_entirely_in_input_vector_and_returns_number_of_elements_being_removed() {
        let mut input = vec![1, 2, 3, 2, 1];
        let result = run(&mut input, 2);

        assert_eq!(input, vec![3, 1, 1]);
        assert_eq!(result, 3);

        let mut input = vec![3, 1, 2, 3, 5, 1, 4, 2, 3, 3, 5, 8];
        let result = run(&mut input, 3);

        assert_eq!(input, vec![5, 1, 4, 2, 2, 1, 5, 8]);
        assert_eq!(result, 8);
    }
}
