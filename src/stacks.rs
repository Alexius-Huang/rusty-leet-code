pub mod min_stack;

/**
 *   Problem 20. Valid Parentheses
 *   See: https://leetcode.com/problems/valid-parentheses/
 *   
 *   Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

 *   An input string is valid if:
 *   - Open brackets must be closed by the same type of brackets.
 *   - Open brackets must be closed in the correct order.
 *   - Every close bracket has a corresponding open bracket of the same type.
 *
 *   Truthy cases:
 *  
 *   ```rust
 *   use rusty_leet_code::stacks as import;
 *
 *   assert!(is_valid_parentheses("()".to_owned()));
 *   assert!(is_valid_parentheses(r"[](){}".to_owned()));
 *   assert!(is_valid_parentheses(r"({()})".to_owned()));
 *   assert!(is_valid_parentheses(r"({()[]}[]{[]})".to_owned()));
 *   ```
 * 
 *   Falsey cases:
 *   ```rust
 *   use rusty_leet_code::stacks as import;
 *
 *   assert!(!is_valid_parentheses(r"[({)})]".to_owned()));
 *   assert!(!is_valid_parentheses(r"[(({})}]".to_owned()));
 *   ``` 
 */
pub fn is_valid_parentheses(s: String) -> bool {
    let mut result: Vec<char> = vec![];

    for chr in s.chars() {
        match chr {
            '{' | '[' | '(' => result.push(chr),
            '}' | ']' | ')' => {
                if let Some(popped) = result.pop() {
                    if (chr == '}' && popped != '{') ||
                        (chr == ']' && popped != '[') ||
                        (chr == ')' && popped != '(') {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            _ => continue
        }
    }

    result.len() == 0
}

#[cfg(test)]
mod is_valid_parentheses_test {
    use super::is_valid_parentheses;

    #[test]
    fn it_is_truthy_if_parentheses_are_properly_enclosed() {
        assert!(is_valid_parentheses("()".to_owned()));
        assert!(is_valid_parentheses(r"[](){}".to_owned()));
        assert!(is_valid_parentheses(r"({()})".to_owned()));
        assert!(is_valid_parentheses(r"({()[]}[]{[]})".to_owned()));

        assert!(!is_valid_parentheses(r"[({)})]".to_owned()));
        assert!(!is_valid_parentheses(r"[(({})}]".to_owned()));
    }
}
