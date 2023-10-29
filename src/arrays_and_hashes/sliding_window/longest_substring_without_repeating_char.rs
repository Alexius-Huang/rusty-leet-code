use std::collections::HashSet;

/**
 *  Problem 3. Longest Substring without Repeating Characters (Medium)
 *  See: https://leetcode.com/problems/longest-substring-without-repeating-characters/
 * 
 *  Given a string `s`, find the length of the longest substring without
 *  repeating characters.
 */
pub fn run(s: String) -> i32 {
    if s.len() < 2 { return s.len() as i32; }

    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    let mut char_set: HashSet<char> = HashSet::new();
    let mut result = 1;
    let chars = s.chars().collect::<Vec<char>>();

    loop {
        let chr = chars[right_index];

        if !char_set.insert(chr) {
            if char_set.len() > result {
                result = char_set.len();
            }

            loop {
                let left_index_char = chars[left_index];
                left_index += 1;

                if left_index_char == chr { break; }
                char_set.remove(&left_index_char);
            }
        }

        right_index += 1;

        if right_index == s.len() {
            return if char_set.len() > result {
                char_set.len()
            } else {
                result
            } as i32;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_longest_length_of_substring_without_repeating_chars() {
        assert_eq!(run("abcabcbb".to_owned()), 3);
        assert_eq!(run("bbbb".to_owned()), 1);
        assert_eq!(run("pwwkew".to_owned()), 3);
        assert_eq!(run("a".to_owned()), 1);
        assert_eq!(run("ab".to_owned()), 2);
        assert_eq!(run("abc".to_owned()), 3);
        assert_eq!(run("abac".to_owned()), 3);
    }
}