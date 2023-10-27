pub mod remove_duplicates;
pub mod remove_element;
pub mod top_k_frequent_elements;
pub mod valid_sudoku;
pub mod encode_and_decode_string;
pub mod longest_consecutive_sequence;

use std::collections::HashMap;

/**
 *   Problem 217. Contains Duplicate (Easy)
 *   See: https://leetcode.com/problems/contains-duplicate/
 *   Given an integer array nums, return `true` if any value appears at least twice in the array, and return `false` if every element is distinct.
 *
 *   Example 1:
 *   ```rust
 *   use rusty_leet_code::arrays_and_hashes as import;
 * 
 *   let input = vec![1, 2, 3, 4, 2];
 *   assert!(import::contains_duplicate(input));
 *   ```
 *
 *   Example 2:
 *   ```rust
 *   use rusty_leet_code::arrays_and_hashes as import;
 * 
 *   let input = vec![1, 2, 3, 4, 5];
 *   assert!(!import::contains_duplicate(input));
 *   ```
 *   
 *   Example 3:
 *   ```rust
 *   use rusty_leet_code::arrays_and_hashes as import;
 * 
 *   let input = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
 *   assert!(import::contains_duplicate(input));
 *   ```
 */
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut cache: HashMap<i32, ()> = HashMap::new();

    for num in nums {
        if cache.get(&num).is_some() {
            return true;
        }
        cache.insert(num, ());
    }
    false
}

#[cfg(test)]
mod contains_duplicate_tests {
    use super::contains_duplicate;

    #[test]
    fn returns_true_if_contains_duplicate() {
        assert!(contains_duplicate(vec!(1, 2, 3, 2, 4)));
        assert!(contains_duplicate(vec!(2, 3, 1, 4, 2)));
    }

    #[test]
    fn returns_false_if_no_duplicates() {
        assert!(!contains_duplicate(vec!(1, 2, 3, 4, 0)));
        assert!(!contains_duplicate(vec!(3, 2, 1, 4, 5)));
    }
}

/**
 *   Problem 242. Valid Anagram (Easy)
 *   See: https://leetcode.com/problems/valid-anagram/
 *
 *   Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, and `false` otherwise.
 * 
 *   An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *   
 *   Example 1:
 *   ```rust
 *   use rusty_leet_code::arrays_and_hashes as import;
 * 
 *   assert!(import::is_anagram("anagram".to_owned(), "nagaram".to_owned()));
 *   ```
 * 
 *   Example 2:
 *   ```rust
 *   use rusty_leet_code::arrays_and_hashes as import;
 * 
 *   assert!(!import::is_anagram("rat".to_owned(), "car".to_owned()));
 *   ```
 */
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut cache: HashMap<char, usize> = HashMap::new();

    for chr in s.chars() {
        cache.insert(
            chr,
            match cache.get(&chr) {
                Some(result) => result + 1,
                None => 1
            }
        );
    }

    for chr in t.chars() {
        let result = cache.get(&chr);
        if result.is_none() { return false; }

        let result = *result.unwrap();

        if result < 2 {
            cache.remove_entry(&chr);
        } else {
            cache.insert(chr, result - 1);
        }
    }

    return cache.len() == 0;
}

#[cfg(test)]
mod is_anagram_tests {
    use super::is_anagram;

    #[test]
    fn returns_true_if_is_anagram() {
        assert!(is_anagram("anagram".to_owned(), "nagaram".to_owned()));
    }

    #[test]
    fn returns_false_if_is_not_anagram() {
        assert!(!is_anagram("rat".to_owned(), "car".to_owned()));
    }
}

/**
 *   Problem 1. Two Sum (Easy)
 *   See: https://leetcode.com/problems/two-sum/
 *
 *   Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 *   
 *   You may assume that each input would have exactly one solution, and you may not use the same element twice.
 *   
 *   You can return the answer in any order.
 *   
 *   Examples:
 *   ```rust
 *   use rusty_leet_code::arrays_and_hashes as import;
 * 
 *   assert_eq!(import::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
 *   assert_eq!(import::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
 *   assert_eq!(import::two_sum(vec![3, 3], 6), vec![0, 1]);
 *   ```
 */
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // dict stores key as target, value is 
    let mut dict: HashMap<i32, usize> = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        if let Some(result) = dict.get(num) {
            return vec![*result as i32, index as i32];
        }

        dict.insert(target - *num, index);
    }

    return vec![0, 0];
}

#[cfg(test)]
mod two_sum_tests {
    use super::two_sum;

    #[test]
    fn it_returns_array_of_indexs_which_sum_is_the_target() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);              
    }
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result_map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for string in strs {
        let mut key = string.chars().collect::<Vec<char>>();
        key.sort();

        /* Longer Version */
        // if let Some(vec) = result_map.get_mut(&key) {
        //     vec.push(string);
        // } else {
        //     result_map.insert(key, vec![string]);
        // }

        /* Concise Version */
        result_map
            .entry(key)
            .or_insert(vec![])
            .push(string);
    }

    result_map.into_iter().map(|(_, data)| data).collect()
}

#[cfg(test)]
mod group_anagrams_test {
    use crate::test_utils::are_string_vectors_similar;
    use super::group_anagrams;

    #[test]
    #[ignore = "TODO: Implement proper test"]
    fn it_groups_the_anagrams() {
        let strs: Vec<String> = vec![
            "eat".to_owned(),
            "tea".to_owned(),
            "tan".to_owned(),
            "ate".to_owned(),
            "nat".to_owned(),
            "bat".to_owned()
        ];

        let result = group_anagrams(strs);

        assert!(are_string_vectors_similar(
            &result[0],
            &vec!["eat".to_owned(), "tea".to_owned(), "ate".to_owned()]
        ));
        assert!(are_string_vectors_similar(
            &result[1],
            &vec!["tan".to_owned(), "nat".to_owned()]
        ));
        assert!(are_string_vectors_similar(
            &result[2],
            &vec!["bat".to_owned()]
        ));

        assert_eq!(result.len(), 3);
    }
}

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
pub fn placeholder() {}

// pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
//     if k == 0 { return vec![]; }

//     let mut freq_map: HashMap<i32, i32> = HashMap::new();
//     let mut results: Vec<(i32, i32)> = Vec::new();

//     for num in nums {
//         if let Some(&occurance_count) = freq_map.get(&num) {
//             freq_map.insert(num, occurance_count + 1);
//         } else {
//             freq_map.insert(num, 1);
//         }
//     }

//     // TODO: Sort
 
//     vec![]
// }
