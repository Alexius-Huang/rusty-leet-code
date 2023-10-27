use std::collections::HashSet;

/**
 *  Problem 128. Longest Consecutive Sequence
 *  See: https://leetcode.com/problems/longest-consecutive-sequence/
 *
 *  Given an unsorted array of integers nums, return the length of
 *  the longest consecutive elements sequence.
 *  
 *  You must write an algorithm that runs in O(n) time.
 */
pub fn run(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut num_set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());

    let mut max_result = 0;

    loop {
        let num = *num_set.iter().next().unwrap();
        num_set.remove(&num);

        let mut cur_result = 1;
        let mut cur_target = num - 1;
        // search left
        loop {
            if !num_set.contains(&cur_target) {
                break;
            }

            num_set.remove(&cur_target);
            cur_result += 1;
            cur_target -= 1;
        }

        // search right
        cur_target = num + 1;
        loop {
            if !num_set.contains(&cur_target) {
                break;
            }

            num_set.remove(&cur_target);
            cur_result += 1;
            cur_target += 1;
        }

        if cur_result > max_result {
            max_result = cur_result;
        }

        if num_set.is_empty() {
            return max_result;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_number_of_longest_consecutive_sequence() {
        assert_eq!(run(vec![100, 4, 200, 1, 3, 2]), 4);

        assert_eq!(run(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
