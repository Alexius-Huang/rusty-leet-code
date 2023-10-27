use std::collections::HashMap;

/**
 *   Problem 347. Top K Frequent Element (Medium)
 *   See: https://leetcode.com/problems/top-k-frequent-elements/
 *   Given an integer array `nums` and an integer `k`, return the `k` most frequent elements.
 *   You may return the answer in any order.
 *
 *   Example 1:
 *   Input: nums = [1,1,1,2,2,3], k = 2
 *   Output: [1,2]
 *   Example 2:
 *   
 *   Input: nums = [1], k = 1
 *   Output: [1]
 */
pub fn run(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map = HashMap::new();

    for num in nums {
        if let Some(freq) = freq_map.get(&num) {
            freq_map.insert(num, freq + 1);
        } else {
            freq_map.insert(num, 1);
        }
    }

    let mut sorted = freq_map.iter().collect::<Vec<(&i32, &i32)>>();

    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut result: Vec<i32> = Vec::new();
    for index in 0..k {
        result.push(*sorted[index as usize].0);
    }

    result
}

pub fn solution_2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts_map: Vec<Vec<i32>> = vec![];
    let len = nums.len();
    for _ in 0..len {
        counts_map.push(Vec::new());
    }

    println!("{counts_map:?}");

    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    for num in nums.iter() {
        if let Some(freq) = freq_map.get(&num) {
            freq_map.insert(*num, freq + 1);
        } else {
            freq_map.insert(*num, 1);
        }
    }

    for (k, v) in freq_map {
        counts_map.get_mut((v - 1) as usize).unwrap().push(k);
    }

    let mut result = vec![];

    // Although this looks like this is O(n^2), but inner-loop would mostly
    // be 1 element if answers are distinct, hence combined together would
    // become O(n)
    'k_freq_elements_loop: for index in (0..len).rev() {
        let k_freq_elements = &counts_map[index];

        for el in k_freq_elements {
            result.push(*el);
            if result.len() == k as usize {
                break 'k_freq_elements_loop;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_top_k_frequent_elements() {
        assert_eq!(run(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn it_uses_bucket_sort() {
        assert_eq!(solution_2(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }
}
