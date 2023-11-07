/**
 *  Problem 11. Container With Most Water (Medium)
 *  See: https://leetcode.com/problems/container-with-most-water/
 * 
 *  You are given an integer array height of length n.
 * 
 *  There are n vertical lines drawn such that the two endpoints of the ith
 *  line are (i, 0) and (i, height[i]).
 * 
 *  Find two lines that together with the x-axis form a container, such that
 *  the container contains the most water.
 *  
 *  Return the maximum amount of water a container can store.
 *  
 *  Notice that you may not slant the container.
 */
pub fn run(heights: Vec<i32>) -> i32 {
    let len = heights.len();
    if len < 2 { return 0; }
    if len == 2 { return i32::min(heights[0], heights[1]); }

    let (mut l_index, mut r_index) = (0, len - 1);
    let (mut l_max_height, mut r_max_height) = (heights[l_index], heights[r_index]);
    let mut result = i32::min(l_max_height, r_max_height) * (r_index - l_index) as i32;

    while l_index != r_index {
        if l_max_height > r_max_height {
            r_index -= 1;

            if heights[r_index] > r_max_height {
                r_max_height = heights[r_index];
                result = i32::max(
                    result,
                    i32::min(l_max_height, r_max_height) * (r_index - l_index) as i32
                );
            }
        } else {
            l_index += 1;
            if heights[l_index] > l_max_height {
                l_max_height = heights[l_index];
                result = i32::max(
                    result,
                    i32::min(l_max_height, r_max_height) * (r_index - l_index) as i32
                );
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_max_area_of_container_of_water() {
        assert_eq!(run(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(run(vec![1, 1]), 1)
    }
}
