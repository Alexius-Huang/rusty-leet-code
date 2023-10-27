/**
 *  Problem 22. Generate Parentheses (Medium)
 *  See: https://leetcode.com/problems/generate-parentheses/
 *
 *  Given n pairs of parentheses, write a function to generate all
 *  combinations of well-formed parentheses.
 */
use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

pub fn run(n: i32) -> Vec<String> {
    let first_term = vec!["()".to_owned()];
    if n == 1 {
        return first_term;
    }

    let second_term = vec!["(())".to_owned(), "()()".to_owned()];
    if n == 2 {
        return second_term;
    }

    let mut nth_map: HashMap<i32, Vec<String>> = HashMap::new();
    nth_map.insert(1, first_term);
    nth_map.insert(2, second_term);

    for i in 3..=n {
        let mut ith_result: Vec<String> = vec![];
        let prev_result = nth_map.get(&(i - 1)).unwrap();

        /* Building format (gen[n - 1]) and ()gen[n - 1]*/
        for string in prev_result {
            ith_result.push(format!("({})", string));

            ith_result.push(format!("(){}", string));
        }

        /* Building format (gen[j])gen[k] */
        for j in 1..(i - 1) {
            let k = i - j - 1;

            let jth_result = nth_map.get(&j).unwrap();
            let kth_result = nth_map.get(&k).unwrap();

            for jth_string in jth_result {
                for kth_string in kth_result {
                    ith_result.push(format!("({}){}", jth_string, kth_string));
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
        input.into_iter().map(|str| str.to_owned()).collect()
    }

    #[test]
    fn it_generates_parentheses() {
        assert_eq!(run(1), to_string_vec(&mut vec!["()"]));

        assert_eq!(run(2), to_string_vec(&mut vec!["(())", "()()"]));

        assert_eq!(
            run(3),
            to_string_vec(&mut vec!["((()))", "()(())", "(()())", "()()()", "(())()"])
        );

        assert_eq!(
            run(4),
            to_string_vec(&mut vec![
                "(((())))", "()((()))", "(()(()))", "()()(())", "((()()))", "()(()())", "(()()())",
                "()()()()", "((())())", "()(())()", "(())(())", "(())()()", "((()))()", "(()())()"
            ])
        );

        assert_eq!(
            run(5),
            to_string_vec(&mut vec![
                "((((()))))",
                "()(((())))",
                "(()((())))",
                "()()((()))",
                "((()(())))",
                "()(()(()))",
                "(()()(()))",
                "()()()(())",
                "(((()())))",
                "()((()()))",
                "(()(()()))",
                "()()(()())",
                "((()()()))",
                "()(()()())",
                "(()()()())",
                "()()()()()",
                "(((())()))",
                "()((())())",
                "(()(())())",
                "()()(())()",
                "((())(()))",
                "()(())(())",
                "((())()())",
                "()(())()()",
                "(((()))())",
                "()((()))()",
                "((()())())",
                "()(()())()",
                "(())((()))",
                "(())()(())",
                "(())(()())",
                "(())()()()",
                "(())(())()",
                "((()))(())",
                "((()))()()",
                "(()())(())",
                "(()())()()",
                "(((())))()",
                "(()(()))()",
                "((()()))()",
                "(()()())()",
                "((())())()"
            ])
        );
    }
}

// Solution 2 based on one crucial rule: closing parentheses must
// be more than opening parentheses

struct Parentheses {
    result: String,
    remaining_open: u32,
    remaining_close: u32,
}

impl Parentheses {
    fn new(remaining: u32) -> Self {
        Self {
            result: "(".to_owned(),
            remaining_open: remaining - 1,
            remaining_close: remaining,
        }
    }

    fn push_opening(&self) -> Self {
        Self {
            result: format!("{}(", self.result),
            remaining_open: self.remaining_open - 1,
            ..*self
        }
    }

    fn push_closing(&self) -> Self {
        Self {
            result: format!("{})", self.result),
            remaining_close: self.remaining_close - 1,
            ..*self
        }
    }

    fn is_closable(&self) -> bool {
        self.remaining_close > self.remaining_open
    }

    fn finalize(&self) -> String {
        format!(
            "{}{}",
            self.result,
            ")".repeat(self.remaining_close as usize).as_str()
        )
    }
}

pub fn solution_2(n: i32) -> Vec<String> {
    fn parse_parentheses(p: Parentheses) -> Vec<String> {
        if p.remaining_open == 0 {
            return vec![if p.remaining_close == 0 {
                p.result
            } else {
                p.finalize()
            }];
        }

        let mut result = parse_parentheses(p.push_opening());

        if p.is_closable() {
            result.append(&mut parse_parentheses(p.push_closing()));
        }

        result
    }

    parse_parentheses(Parentheses::new(n as u32))
}

pub fn solution_3(n: i32) -> Vec<String> {
    fn parse_parentheses(p: Parentheses) -> Vec<String> {
        if p.remaining_open == 0 {
            return vec![if p.remaining_close == 0 {
                p.result
            } else {
                p.finalize()
            }];
        }

        if !p.is_closable() {
            return parse_parentheses(p.push_opening());
        }

        let mut opening_result = vec![];
        let mut closing_result = vec![];

        thread::scope(|scope| {
            scope.spawn(|| {
                opening_result.append(&mut parse_parentheses(p.push_opening()));
            });

            scope.spawn(|| {
                closing_result.append(&mut parse_parentheses(p.push_closing()));
            });
        });

        opening_result.append(&mut closing_result);

        opening_result
    }

    parse_parentheses(Parentheses::new(n as u32))
}

// After experiment, thread version isn't faster than without thread
pub fn run_comparison(loop_time: i32, max_gen: i32) {
    println!("Running {loop_time} times gen_parens 1 to {max_gen} without thread");

    let mut result_without_thread: Vec<Duration> = vec![];

    for i in 1..=max_gen {
        let now = Instant::now();
        for _ in 0..loop_time {
            solution_2(i);
        }
        result_without_thread.push(Instant::now() - now);

        println!("Without thread: gen({i}) complete!");
    }

    println!("Running {loop_time} times gen_parens 1 to {max_gen} with thread");

    let mut result_with_thread: Vec<Duration> = vec![];

    for i in 1..=max_gen {
        let now = Instant::now();
        for _ in 0..loop_time {
            solution_3(i);
        }
        result_with_thread.push(Instant::now() - now);

        println!("With thread: gen({i}) complete!");
    }

    println!("Without Thread v.s. With Thread");

    for i in 1..=max_gen {
        let without_thread = result_without_thread[i as usize - 1];
        let with_thread = result_with_thread[i as usize - 1];
        println!(
            "gen({i}) {:?} v.s. {:?} => {} wins!",
            without_thread,
            with_thread,
            if with_thread < without_thread {
                "With Thread"
            } else {
                "Without Thread"
            }
        );
    }
}
