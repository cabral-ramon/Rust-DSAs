use std::collections::HashMap;

fn main() {}

fn is_valid_anagram(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    };

    let mut table: HashMap<char, i32> = str1.chars().fold(HashMap::new(), |mut map, c| {
        map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        map
    });

    for c in str2.chars() {
        table.entry(c).and_modify(|count| *count -= 1).or_insert(1);
    }

    println!("{:?}", table);

    table.values().fold(0, |mut acc, x| {
        acc += x.abs();
        acc
    }) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_nagaram() {
        let s1 = "anagram";
        let s2 = "nagaram";

        assert!(is_valid_anagram(s1, s2))
    }

    #[test]
    fn test_spear_pears() {
        let s1 = "spear";
        let s2 = "pears";

        assert!(is_valid_anagram(s1, s2))
    }

    #[test]
    fn test_heart_earth() {
        let s1 = "heart";
        let s2 = "earth";

        assert!(is_valid_anagram(s1, s2))
    }

    #[test]
    fn test_super_upper() {
        let s1 = "super";
        let s2 = "upper";

        assert!(!is_valid_anagram(s1, s2))
    }

    #[test]
    fn test_triangle_integral() {
        let s1 = "triangle";
        let s2 = "integral";

        assert!(is_valid_anagram(s1, s2))
    }
}
