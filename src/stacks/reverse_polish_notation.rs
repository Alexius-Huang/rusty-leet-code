/**
 *  Problem 150. Evaluate Reverse Polish Notation
 *  See: https://leetcode.com/problems/evaluate-reverse-polish-notation/ (Medium)
 * 
 *  You are given an array of strings tokens that represents an arithmetic
 *  expression in a Reverse Polish Notation.
 *
 *  Evaluate the expression. Return an integer that represents the value
 *  of the expression.
 *  
 *  Note that:
 *  - The valid operators are '+', '-', '*', and '/'.
 *  - Each operand may be an integer or another expression.
 *  - The division between two integers always truncates toward zero.
 *  - There will not be any division by zero.
 *  - The input represents a valid arithmetic expression in a reverse polish notation.
 *  - The answer and all the intermediate calculations can be represented in a 32-bit integer.
 * 
 */
pub fn run(tokens: Vec<String>) -> i32 {
    if tokens.len() == 1 { return tokens[0].parse().unwrap(); }

    let mut stack: Vec<i32> = vec![];
    let mut result = 0;

    for token in tokens.iter() {
        let maybe_num: Result<i32, _> = token.parse();

        match maybe_num {
            Ok(num) => {
                stack.push(num);
            },
            Err(_) => {
                let y = stack.pop().unwrap();
                let x = stack.pop().unwrap();
                match token.as_str() {
                    "+" => { result = x + y },
                    "-" => { result = x - y },
                    "*" => { result = x * y },
                    "/" => { result = x / y },
                    _ => panic!("Invald token {token}")
                }

                stack.push(result);
            }
        }
    }

    result
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
    fn it_evaluates_reverse_polish_notation_correctly() {
        let input = to_string_vec(
            &mut vec!["2", "1", "+", "3", "*"]
        );
        assert_eq!(run(input), 9);

        let input = to_string_vec(
            &mut vec!["4", "13", "5", "/", "+"]
        );
        assert_eq!(run(input), 6);

        let input = to_string_vec(
            &mut vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
        );
        assert_eq!(run(input), 22);

        let input = to_string_vec(
            &mut vec!["18"]
        );
        assert_eq!(run(input), 18);
    }
}
