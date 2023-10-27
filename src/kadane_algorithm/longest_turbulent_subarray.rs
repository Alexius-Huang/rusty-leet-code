/**
 *  Problem 978. Longest Turbulent Subarray (Medium)
 *  See: https://leetcode.com/problems/longest-turbulent-subarray/ 
 * 
 *  Given an integer array arr, return the length of a maximum size
 *  turbulent subarray of arr.

 *  A subarray is turbulent if the comparison sign flips between each
 *  adjacent pair of elements in the subarray.
 *  
 *  More formally, a subarray [arr[i], arr[i + 1], ..., arr[j]] of
 *  `arr` is said to be turbulent if and only if:
 *  
 *  - For i <= k < j:
 *    - arr[k] > arr[k + 1] when k is odd, and
 *    - arr[k] < arr[k + 1] when k is even.
 *  - Or, for i <= k < j:
 *    - arr[k] > arr[k + 1] when k is even, and
 *    - arr[k] < arr[k + 1] when k is odd.
 */
#[derive(PartialEq)]
enum Trending { Lower, Flat, Higher }

impl Trending {
    fn new(num: i32) -> Self {
        match num {
            1.. => Trending::Higher,
            0 => Trending::Flat,
            _ => Trending::Lower
        }
    }
}

pub fn run(arr: Vec<i32>) -> i32 {
    if arr.len() < 2 { return arr.len() as i32; }

    let mut prev_num = arr[1];
    let mut prev_trend = Trending::new(arr[1] - arr[0]);

    let mut result = if prev_trend == Trending::Flat { 1 } else { 2 };
    let mut current_max_result = result;

    for i in 2..arr.len() {
        let num = arr[i];
        let cur_trend = Trending::new(arr[i] - prev_num);

        match (&prev_trend, &cur_trend) {
            (Trending::Higher, Trending::Lower) |
            (Trending::Lower, Trending::Higher) => {
                current_max_result += 1;
                if current_max_result > result {
                    result = current_max_result;
                }
            },
            (Trending::Flat, Trending::Flat) => {
                current_max_result = 1;
            },
            _ => {
                current_max_result = 2;
                if current_max_result > result {
                    result = current_max_result;
                }
            }
        }

        prev_trend = cur_trend;
        prev_num = num;
    }

    result
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn it_returns_maximum_length_of_turbulent_subarray() {
        assert_eq!(run(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]), 5);

        assert_eq!(run(vec![4, 8, 12, 16]), 2);

        assert_eq!(run(vec![100]), 1);

        assert_eq!(run(vec![9, 9]), 1);

        assert_eq!(run(vec![100, 100, 100]), 1);

        assert_eq!(run(vec![1, 1, 2]), 2);
    }
}