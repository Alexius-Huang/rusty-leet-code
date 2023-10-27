/**
 *  Problem 918. Maximum Sum Circular Subarray (Medium)
 * 
 *  Given a circular integer array nums of length n, return the maximum
 *  possible sum of a non-empty subarray of nums.
 *  
 *  A circular array means the end of the array connects to the beginning
 *  of the array.
 * 
 *  Formally, the next element of nums[i] is nums[(i + 1) % n] and the
 *  previous element of nums[i] is nums[(i - 1 + n) % n].
 *  
 *  A subarray may only include each element of the fixed buffer nums at
 *  most once.
 * 
 *  Formally, for a subarray nums[i], nums[i + 1], ..., nums[j], there
 *  does not exist i <= k1, k2 <= j with k1 % n == k2 % n.
 * 
 */
pub fn run(nums: Vec<i32>) -> i32 {
    let mut global_min = nums[0];
    let mut current_min = nums[0];
    let mut global_max = nums[0];
    let mut current_max = nums[0];
    let mut total = nums[0];

    for i in 1..nums.len() {
        let num = nums[i];
        total += num;

        // Updating global max
        let current_sum_with_max = num + current_max;
        current_max = if current_sum_with_max > num {
            current_sum_with_max
        } else {
            num
        };

        if global_max < current_max {
            global_max = current_max;
        }

        // Updating global min
        let current_sum_with_min = num + current_min;
        current_min = if current_sum_with_min < num {
            current_sum_with_min
        } else {
            num
        };


        if global_min > current_min {
            global_min = current_min;
        }
    }

    let diff_from_global_min = total - global_min;

    let all_elements_are_negative = global_max < 0;
    let global_max_is_larger = global_max > diff_from_global_min;

    if all_elements_are_negative || global_max_is_larger {
        global_max
    } else {
        diff_from_global_min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_maximum_value_of_circular_subarray() {
        assert_eq!(run(vec![1, -2, 3, -2]), 3);

        assert_eq!(run(vec![5, -3, 5]), 10);

        assert_eq!(run(vec![-3, -2, -3]), -2);

        assert_eq!(run(vec![-1, 3, -3, 9, -6, 8, -5, -5, -6, 10]), 20);
    }
}