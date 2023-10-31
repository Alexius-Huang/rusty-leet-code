/**
 *  Problem 1343. Number of Subarray Size K Greater Than Threshold (Medium)
 *  See: https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
 *
 *  Given an array of integers arr and two integers k and threshold,
 *  return the number of sub-arrays of size k and average greater than
 *  or equal to threshold.
 *
 */
pub fn run(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let len = arr.len();
    if arr.len() < k as usize {
        return 0;
    }

    let mut result = 0;
    let mut windowed_sum = 0;

    // Constructing windowed sum
    for i in 0..k as usize {
        windowed_sum += arr[i];
    }

    if windowed_sum / k >= threshold {
        result += 1;
    }

    // Sliding window
    for i in k as usize..len {
        windowed_sum -= arr[i - k as usize];
        windowed_sum += arr[i];

        if windowed_sum / k >= threshold {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_num_of_fixed_size_subarray_average_over_threshold() {
        assert_eq!(run(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);

        assert_eq!(run(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5), 6);

        assert_eq!(run(vec![2, 2, 2, 2, 2, 2, 2], 3, 2), 5);
    }
}
