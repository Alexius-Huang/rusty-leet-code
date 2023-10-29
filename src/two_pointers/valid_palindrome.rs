/**
 *  Problem 125. Valid Palindrome
 *  See: https://leetcode.com/problems/valid-palindrome/
 *
 *  A phrase is a palindrome if, after converting all uppercase letters
 *  into lowercase letters and removing all non-alphanumeric characters,
 *  it reads the same forward and backward.
 *
 *  Alphanumeric characters include letters and numbers.
 *
 *  Given a string s, return `true` if it is a palindrome, or `false` otherwise.
 *
 */
pub fn run(s: String) -> bool {
    let mut forward_vec: Vec<char> = Vec::new();
    let mut backward_vec: Vec<char> = Vec::new();
    let characters: Vec<char> = s.chars().collect();
    let length = characters.len();

    for i in 0..length {
        let (&chr1, &chr2) = (
            characters.get(i).unwrap(),
            characters.get(length - 1 - i).unwrap(),
        );
        match chr1 {
            '0'..='9' | 'a'..='z' => forward_vec.push(chr1),
            'A'..='Z' => forward_vec.push(chr1.to_ascii_lowercase()),
            _ => (),
        }

        match chr2 {
            '0'..='9' | 'a'..='z' => backward_vec.push(chr2),
            'A'..='Z' => backward_vec.push(chr2.to_ascii_lowercase()),
            _ => (),
        }
    }

    forward_vec == backward_vec
}

pub fn run_with_no_additional_memory_required(s: String) -> bool {
    let s = s.trim();
    if s.len() < 2 { return true; }

    let chars = s.chars().collect::<Vec<char>>();
    let mut left_index = 0;
    let mut right_index = s.len() - 1;
    
    while right_index != left_index {
        let mut left_char = chars[left_index];
        match left_char {
            'a'..='z' | 'A'..='Z' => {
                left_char = left_char.to_ascii_lowercase();
            },
            '0'..='9' => (),
            _ => {
                left_index += 1;
                continue;    
            }
        }

        let mut right_char = chars[right_index];
        match right_char {
            'a'..='z' | 'A'..='Z' => {
                right_char = right_char.to_ascii_lowercase();
            },
            '0'..='9' => (),
            _ => {
                right_index -= 1;
                continue;
            }
        }

        if left_char != right_char { return false; }
        if right_index - left_index == 1 { return true; }

        left_index += 1;
        right_index -= 1;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_validates_if_the_string_is_palindrome() {
        assert!(run("A man, a plan, a canal: Panama".to_owned()));
        assert!(run("12321".to_owned()));
        assert!(run(" ".to_owned()));

        assert!(!run("race a car".to_owned()));
        assert!(!run("1234567890".to_owned()));
        assert!(!run("abc012".to_owned()));
    }

    #[test]
    fn it_validates_if_the_string_is_palindrome_by_using_two_pointer_algorithm() {
        assert!(run_with_no_additional_memory_required("A man, a plan, a canal: Panama".to_owned()));
        assert!(run_with_no_additional_memory_required("12321".to_owned()));
        assert!(run_with_no_additional_memory_required(" ".to_owned()));

        assert!(!run_with_no_additional_memory_required("race a car".to_owned()));
        assert!(!run_with_no_additional_memory_required("1234567890".to_owned()));
        assert!(!run_with_no_additional_memory_required("abc012".to_owned()));
        assert!(!run_with_no_additional_memory_required("0P".to_owned()));
    }
}
