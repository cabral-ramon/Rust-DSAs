use std::collections::HashMap;

fn main() {
    todo!();
}

// #[allow(dead_code)]
// fn is_palindrome_permutation(s: &str) -> bool {
//     let mut map: HashMap<char, i32> = HashMap::new();

//     for c in s.chars() {
//         map.entry(c).and_modify(|c| *c += 1).or_insert(1);
//     }

//     let odd_count = map.values().filter(|x| *x % 2 != 0).count();

//     odd_count < 2
// }


// Code golf
#[allow(dead_code)]
fn is_palindrome_permutation(s: &str) -> bool {
    s.chars()
        .fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|c| *c += 1).or_insert(1);
            map
        })
        .values()
        .filter(|x| *x & 1 != 0)
        .count()
        < 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_true() {
        let s = "abab";
        assert!(is_palindrome_permutation(s));
    }

    #[test]
    fn test_basic_false() {
        let s = "peas";
        assert!(!is_palindrome_permutation(s));
    }

    #[test]
    fn test_racecar() {
        let s = "racecar";
        assert!(is_palindrome_permutation(s));
    }

    #[test]
    fn test_baefeab() {
        let s = "baefeab";
        assert!(is_palindrome_permutation(s));
    }

    #[test]
    fn test_code() {
        let s = "code";
        assert!(!is_palindrome_permutation(s));
    }
}
