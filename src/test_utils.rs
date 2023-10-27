use std::collections::HashMap;

pub fn are_string_vectors_similar(v1: &Vec<String>, v2: &Vec<String>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }

    let mut hmap: HashMap<String, i32> = HashMap::new();
    for string in v1 {
        let string = string.clone();
        match hmap.get(&string) {
            None => hmap.insert(string, 1),
            Some(&value) => hmap.insert(string, value + 1),
        };
    }

    for string in v2 {
        let string = string.clone();

        if let Some(&value) = hmap.get(&string) {
            if value == 1 {
                hmap.remove(&string);
            } else {
                hmap.insert(string, value - 1);
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod are_string_vectors_similar_test {
    use super::are_string_vectors_similar;

    #[test]
    fn returns_true_if_string_vectors_are_similar() {
        assert!(are_string_vectors_similar(
            &vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
            &vec!["def".to_owned(), "ghi".to_owned(), "abc".to_owned()]
        ));

        assert!(are_string_vectors_similar(
            &vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
            &vec!["def".to_owned(), "abc".to_owned(), "ghi".to_owned()]
        ));

        assert!(are_string_vectors_similar(
            &vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
            &vec!["ghi".to_owned(), "def".to_owned(), "abc".to_owned()]
        ));
    }

    #[test]
    fn returns_false_if_string_vectors_are_similar() {
        assert!(!are_string_vectors_similar(
            &vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
            &vec!["def".to_owned(), "abc".to_owned(), "abc".to_owned()]
        ));

        assert!(!are_string_vectors_similar(
            &vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
            &vec!["def".to_owned(), "xyz".to_owned(), "ghi".to_owned()]
        ));

        assert!(!are_string_vectors_similar(
            &vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
            &vec![
                "ghi".to_owned(),
                "def".to_owned(),
                "abc".to_owned(),
                "xyz".to_owned()
            ]
        ));
    }
}
