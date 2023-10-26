/**
 *  Problem 238. Product of Array without Self (Medium)
 *  See: https://leetcode.com/problems/product-of-array-except-self/
 *  Given an integer array nums, return an array answer such that answer[i]
 *  is equal to the product of all the elements of nums except nums[i].
 * 
 *  The product of any prefix or suffix of nums is guaranteed to fit in a
 *  32-bit integer.
 *  
 *  You must write an algorithm that runs in O(n) time and without using
 *  the division operation.
 *  
 */
pub fn run(nums: Vec<i32>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_array_of_product_result_without_self() {
        assert_eq!(
            run(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );

        assert_eq!(
            run(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}