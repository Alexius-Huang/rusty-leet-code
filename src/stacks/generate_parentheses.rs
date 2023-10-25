/**
 *  Problem 22. Generate Parentheses (Medium)
 *  See: https://leetcode.com/problems/generate-parentheses/
 * 
 *  Given n pairs of parentheses, write a function to generate all
 *  combinations of well-formed parentheses.
 */

use std::collections::HashMap;

pub fn run(n: i32) -> Vec<String> {
    let first_term = vec!["()".to_owned()];
    if n == 1 { return first_term; }

    let second_term = vec![
        "(())".to_owned(),
        "()()".to_owned()
    ];
    if n == 2 { return second_term; }

    let mut nth_map: HashMap<i32, Vec<String>> = HashMap::new();
    nth_map.insert(1, first_term);
    nth_map.insert(2, second_term);

    for i in 3..=n {
        let mut ith_result: Vec<String> = vec![];
        let prev_result = nth_map.get(&(i - 1)).unwrap();

        /* Building format (gen[n - 1]) and ()gen[n - 1]*/
        for string in prev_result {
            ith_result.push(
                format!("({})", string)
            );

            ith_result.push(
                format!("(){}", string)
            );
        }

        /* Building format (gen[j])gen[k] */
        for j in 1..(i - 1) {
            let k = i - j - 1;

            let jth_result = nth_map.get(&j).unwrap();
            let kth_result = nth_map.get(&k).unwrap();

            for jth_string in jth_result {
                for kth_string in kth_result {
                    ith_result.push(
                        format!("({}){}", jth_string, kth_string)
                    );
                }
            }
        }

        nth_map.insert(i, ith_result);
    }

    nth_map.remove(&n).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_string_vec(input: &mut Vec<&str>) -> Vec<String> {
        input
            .into_iter()
            .map(|str| str.to_owned())
            .collect()
    }

    #[test]
    fn it_generates_parentheses() {
        assert_eq!(run(1), to_string_vec(&mut vec!["()"]));

        assert_eq!(run(2), to_string_vec(&mut vec!["(())", "()()"]));

        assert_eq!(run(3), to_string_vec(&mut vec![
            "((()))", "()(())", "(()())", "()()()", "(())()"
        ]));

        assert_eq!(run(4), to_string_vec(&mut vec![
            "(((())))", "()((()))", "(()(()))", "()()(())",
            "((()()))", "()(()())", "(()()())", "()()()()",
            "((())())", "()(())()", "(())(())", "(())()()",
            "((()))()", "(()())()"
        ]));

        assert_eq!(run(5), to_string_vec(&mut vec![
            "((((()))))", "()(((())))", "(()((())))",
            "()()((()))", "((()(())))", "()(()(()))",
            "(()()(()))", "()()()(())", "(((()())))",
            "()((()()))", "(()(()()))", "()()(()())",
            "((()()()))", "()(()()())", "(()()()())",
            "()()()()()", "(((())()))", "()((())())",
            "(()(())())", "()()(())()", "((())(()))",
            "()(())(())", "((())()())", "()(())()()",
            "(((()))())", "()((()))()", "((()())())",
            "()(()())()", "(())((()))", "(())()(())",
            "(())(()())", "(())()()()", "(())(())()",
            "((()))(())", "((()))()()", "(()())(())",
            "(()())()()", "(((())))()", "(()(()))()",
            "((()()))()", "(()()())()", "((())())()"
        ]));
    }
}
