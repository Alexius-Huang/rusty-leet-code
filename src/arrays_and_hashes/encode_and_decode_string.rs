/**
 *  Problem 271. Encode and Decode String (Medium)
 *  Design an algorithm to encode a list of strings to a string.
 *
 *  The encoded string is then sent over the network and is decoded
 *  back to the original list of strings.
 *
 *  Note:
 *  - The string may contain any possible characters out of 256 valid
 *    ascii characters. Your algorithm should be generalized enough
 *    to work on any possible characters.
 *  - Do not use class member/global/static variables to store states.
 *    Your encode and decode algorithms should be stateless.
 *  - Do not rely on any library method such as eval or serialize
 *    methods. You should implement your own encode/decode algorithm.
 *
 *  One possible example is: "I:;Love:;Leet:;Code"
 */

// One way of encoding is to specify numbers of char combined with special
// character as delimiter. (See test)
pub fn encode(strings: Vec<String>) -> String {
    let mut result = String::new();
    for string in strings {
        let len = string.len();
        result.push_str(format!("{len}#{string}").as_str());
    }

    result
}

pub fn decode(strings: String) -> Vec<String> {
    let mut num_string = String::new();
    let mut result: Vec<String> = vec![];
    let mut string_len = 0;

    let mut iter = strings.chars().enumerate();

    while let Some((index, chr)) = iter.next() {
        match chr {
            '0'..='9' => num_string.push(chr),
            '#' => {
                string_len = num_string.parse().unwrap();
                result.push(strings[(index + 1)..(index + 1 + string_len)].to_owned());

                num_string = String::new();

                // Skip string_len - 1 iteration, the string_len iteration
                // matches the next number
                iter.nth(string_len - 1);
            }
            _ => panic!("Invalid token: expect number or ending delimiter `#`"),
        };
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_encodes_the_string_by_special_delimiter() {
        assert_eq!(
            encode(vec![
                "Hello".to_owned(),
                "World".to_owned(),
                "123".to_owned(),
                "ab?d#$g:/#123".to_owned(),
                "ENDING".to_owned()
            ]),
            "5#Hello5#World3#12313#ab?d#$g:/#1236#ENDING".to_owned()
        );
    }

    #[test]
    fn it_decodes_the_string_contains_special_delimiter() {
        assert_eq!(
            decode("5#Hello5#World3#12313#ab?d#$g:/#1236#ENDING".to_owned()),
            vec![
                "Hello".to_owned(),
                "World".to_owned(),
                "123".to_owned(),
                "ab?d#$g:/#123".to_owned(),
                "ENDING".to_owned()
            ]
        );
    }
}
