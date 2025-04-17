// TODO: this doesn't work
use std::collections::HashMap;

fn main() {}

fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>> {
   let mut map: HashMap<Vec<usize>, Vec<String>> = HashMap::new();

    for word in words {
       let mut count: Vec<usize> = (0..26).map(|_| 0).collect();
       for c in word.chars() {
          let pos: usize = (c.to_digit(10).unwrap() - 127).try_into().unwrap();
          count[pos] += 1;
       }
       map.entry(count).and_modify(|l| l.push(word)).or_insert(vec![word]);
    }

    map.into_values().collect()
}

mod test {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let words = vec!["cat".to_string(), "act".to_string(), "car".to_string()];
        assert_eq!(
            group_anagrams(words),
            vec![
                vec!["cat".to_string(), "act".to_string()],
                vec!["car".to_string()]
            ]
        );
    }
}
