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
    let len = nums.len();
    if len == 0 { return vec![]; }
    if len == 1 { return vec![0]; }

    let mut prefix_product: Vec<i32> = vec![nums[0]];
    let mut postfix_product: Vec<i32> = vec![nums[len - 1]];

    for i in 1..len {
        let prev_prefix_product = prefix_product[i - 1];
        prefix_product.push(prev_prefix_product * nums[i]);

        let prev_postfix_product = postfix_product[i - 1];
        postfix_product.push(prev_postfix_product * nums[len - i - 1]);
    }
    println!("{prefix_product:?} {postfix_product:?}");
    let mut result = vec![postfix_product[len - 2]];

    for i in 1..(len - 1) {
        result.push(
            prefix_product[i - 1] * postfix_product[len - i - 2]
        );
    }

    result.push(prefix_product[len - 2]);
    result
}

pub fn run_with_o_one_space_complexity(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut result = vec![1; len];

    for i in 0..(len - 1) {
        result[i + 1] *= nums[i] * result[i];
    }

    let mut post_product = 1;
    for i in (1..len).rev() {
        result[i - 1] *= nums[i] * post_product;
        post_product *= nums[i];
    }

    result
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

    #[test]
    fn it_uses_o_one_space_complexity_to_calculate_result() {
        assert_eq!(
            run_with_o_one_space_complexity(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );

        assert_eq!(
            run_with_o_one_space_complexity(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
