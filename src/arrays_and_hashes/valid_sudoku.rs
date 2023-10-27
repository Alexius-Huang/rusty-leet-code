use std::collections::HashSet;

/**
 *  Problem 36. Valid Sudoku (Medium)
 *  See: https://leetcode.com/problems/valid-sudoku/
 *  Determine if a 9 x 9 Sudoku board is valid.
 * 
 *  Only the filled cells need to be validated according to the
 *  following rules:
 * 
 *  - Each row must contain the digits 1-9 without repetition.
 *  - Each column must contain the digits 1-9 without repetition.
 *  - Each of the nine 3 x 3 sub-boxes of the grid must contain the
 *    digits 1-9 without repetition.
 *  
 *  Note:
 *  - A Sudoku board (partially filled) could be valid but is not
 *    necessarily solvable.
 *  - Only the filled cells need to be validated according to the
 *    mentioned rules. 
 */
pub fn run(board: Vec<Vec<char>>) -> bool {
    let mut rows: Vec<HashSet<char>> = vec![];
    let mut columns: Vec<HashSet<char>> = vec![];
    let mut grids: Vec<HashSet<char>> = vec![];
    for _ in 0..9 {
        rows.push(HashSet::new());
        columns.push(HashSet::new());
        grids.push(HashSet::new());
    }

    for row in 0..9 {
        for col in 0..9 {
            let grid_index = (row / 3) * 3 + (col / 3);

            let chr = board[row][col];

            if chr == '.' { continue; }

            if !rows[row].insert(chr) ||
               !columns[col].insert(chr) ||
               !grids[grid_index].insert(chr) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_is_truthy_if_the_sudoku_board_is_valid() {
        assert!(run(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
    }

    #[test]
    fn it_is_falsey_if_one_of_the_row_has_duplicates() {
        assert!(!run(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '1', '.', '.', '1'], // <- Duplicated with 1
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
    }

    #[test]
    fn it_is_falsey_if_one_of_the_column_has_duplicates() {
        assert!(!run(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '6', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
        //                            ^ Duplicated with 6
    }

    #[test]
    fn it_is_falsey_if_one_of_the_grid_has_duplicates() {
        assert!(!run(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '2', '.', '3', '.', '.', '1'],  // <- Central grid
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],  // <- duplicated with 2
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
    }
}
