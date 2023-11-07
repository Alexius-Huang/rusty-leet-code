use std::collections::{HashMap, VecDeque};

/**
 *  Problem 567. Permutation in String (Medium)
 *  See: https://leetcode.com/problems/permutation-in-string/
 * 
 *  Given two strings s1 and s2, return true if s2 contains a permutation of
 *  s1, or false otherwise.
 *
 *  In other words, return true if one of s1's permutations is the substring of s2.
 */
pub fn run(s1: String, s2: String) -> bool {
    let mut chr_map = HashMap::new();
    let len = s1.len();
    let mut window: VecDeque<char> = VecDeque::with_capacity(len);

    for chr in s1.chars() {
        *chr_map.entry(chr).or_insert(0) += 1;
    }

    for chr in s2.chars() {
        if let Some(count) = chr_map.get_mut(&chr) {
            if *count != 0 {
                *count -= 1;
                window.push_back(chr);

                if window.len() == len { return true; }
            } else {
                while let Some(front_chr) = window.pop_front() {
                    if front_chr == chr {
                        window.push_back(chr);
                        break;
                    } else {
                        chr_map.entry(front_chr).and_modify(|count| {
                            *count += 1;
                        });
                    }
                }
            }
            continue;
        }

        while let Some(chr) = window.pop_front() {
            chr_map.entry(chr).and_modify(|count| {
                *count += 1;
            });
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_checks_if_the_string_contains_any_permutation_substring() {
        assert!(run("abc".to_owned(), "dafoebcafa".to_owned()));
        assert!(run("abc".to_owned(), "daabbcafboebdafa".to_owned()));
        assert!(!run("abc".to_owned(), "dacfboebdafa".to_owned()));

        assert!(run("abacb".to_owned(), "daabbcfboebdafa".to_owned()));
        assert!(!run("abacb".to_owned(), "dabbcfboebdaccab".to_owned()));

        assert!(run("abcde".to_owned(), "abcdabcde".to_owned()));
        assert!(run("abcde".to_owned(), "abcdbace".to_owned()));
    }
}