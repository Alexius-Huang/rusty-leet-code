/**
 *  Problem 153. Find Minimum in Rotated Sorted Array (Medium)
 *  See: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
 * 
 *  Suppose an array of length n sorted in ascending order is rotated
 *  between 1 and n times. For example, the array nums = [0,1,2,4,5,6,7]
 *  might become:

 *  - [4, 5, 6, 7, 0, 1, 2] if it was rotated 4 times.
 *  - [0, 1, 2, 4, 5, 6, 7] if it was rotated 7 times.
 *  
 *  Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time
 *  results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
 *  
 *  Given the sorted rotated array nums of unique elements, return the
 *  minimum element of this array.
 *  
 *  You must write an algorithm that runs in O(log n) time.
 *
 */
pub fn run(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    match len {
        0 => { panic!("Expect to have at least 1 element"); },
        1 => { return nums[0]; },
        2 => { return if nums[0] > nums[1] { nums[1] } else { nums[0]}; }
        _ => ()
    }

    let mut lower_bound = 0;
    let mut upper_bound = len;
    let mut mid_index = len / 2;
    loop {
        let left_slice_edge = (
            nums[lower_bound],
            nums[mid_index - 1]
        );
        let is_left_ascending =
            left_slice_edge.1 - left_slice_edge.0 >= 0;

        let right_slice_edge = (
            nums[mid_index],
            nums[upper_bound - 1]
        );
        let is_right_ascending =
            right_slice_edge.1 - right_slice_edge.0 >= 0;

        match (is_left_ascending, is_right_ascending) {
            (true, true) => {
                return if left_slice_edge.0 > right_slice_edge.0 {
                    right_slice_edge.0
                } else {
                    left_slice_edge.0
                };
            },
            (false, true) => {
                upper_bound = mid_index;
            },
            (true, false) => {
                lower_bound = mid_index;

                // Exception: when lower slice only has single element
                if lower_bound == 0 {
                    return right_slice_edge.1;
                }
            },
            _ => panic!("Impossible situation")
        }

        if lower_bound == upper_bound { return nums[lower_bound]; }
        mid_index = (lower_bound + upper_bound) / 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_minimum_value_from_rotated_array() {
        assert_eq!(run(vec![1, 2, 3]), 1); // A(1) A(2,3)
        assert_eq!(run(vec![3, 1, 2]), 1); // A(3) A(1,2)
        assert_eq!(run(vec![2, 3, 1]), 1); // A(2) D(3,1)!

        assert_eq!(run(vec![1, 2, 3, 4]), 1); // A(1,2)! A(3,4)
        assert_eq!(run(vec![4, 1, 2, 3]), 1); // D(4,1)! A(2,3)
        assert_eq!(run(vec![3, 4, 1, 2]), 1); // A(3,4)  A(1,2)!
        assert_eq!(run(vec![2, 3, 4, 1]), 1); // A(2,3)  D(4,1)!

        assert_eq!(run(vec![1, 2, 3, 4, 5]), 1); // A(1,2)! A(3,5)
        assert_eq!(run(vec![5, 1, 2, 3, 4]), 1); // D(5,1)! A(2,4)
        assert_eq!(run(vec![4, 5, 1, 2, 3]), 1); // A(4,5)  A(1,3)!
        assert_eq!(run(vec![3, 4, 5, 1, 2]), 1); // A(3,4)  D(5,2)!
        assert_eq!(run(vec![2, 3, 4, 5, 1]), 1); // A(2,3)  D(4,1)!

        assert_eq!(run(vec![1, 2, 3, 4, 5, 6]), 1);
        assert_eq!(run(vec![6, 1, 2, 3, 4, 5]), 1);
        assert_eq!(run(vec![5, 6, 1, 2, 3, 4]), 1);
        assert_eq!(run(vec![4, 5, 6, 1, 2, 3]), 1);
        assert_eq!(run(vec![3, 4, 5, 6, 1, 2]), 1);
        assert_eq!(run(vec![2, 3, 4, 5, 6, 1]), 1);

        assert_eq!(run(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(run(vec![11, 13, 15, 17]), 11);

        assert_eq!(run(vec![100]), 100);
    }
}