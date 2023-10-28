use std::cmp::Ordering;

/**
 *  Problem 74. Search a 2D Matrix (Medium)
 *  See: https://leetcode.com/problems/search-a-2d-matrix/
 * 
 *   You are given an m x n integer matrix matrix with the following
 *   two properties:
 *   - Each row is sorted in non-decreasing order.
 *   - The first integer of each row is greater than the last integer
 *     of the previous row.
 *   
 *   Given an integer target, return true if target is in matrix or
 *   false otherwise.
 *   
 *   You must write a solution in O(log(m * n)) time complexity.
 *   
 */
pub fn run(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    if rows == 0 { return false; }

    let columns = matrix[0].len();
    if columns == 0 { return false; }

    let total_elements = rows * columns;

    let mut lower_bound: usize = 0;
    let mut upper_bound = total_elements;
    let mut mid_index = total_elements / 2;

    loop {
        let derived_coord = (
            mid_index / columns,
            mid_index % columns
        );

        let num = matrix[derived_coord.0][derived_coord.1];

        match num.cmp(&target) {
            Ordering::Equal => { return true; },
            Ordering::Greater => {
                upper_bound = mid_index;
            },
            Ordering::Less => {
                lower_bound = mid_index + 1;
            }
        }

        if upper_bound == lower_bound { return false; }
        mid_index = (upper_bound + lower_bound) / 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_searches_the_element_in_matrix() {
        assert!(run(vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ], 30));

        assert!(!run(vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ], 4));
    }
}
