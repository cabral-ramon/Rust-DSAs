use std::collections::HashSet;

fn main() {}

fn has_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for num in nums.iter() {
        if seen.contains(num) {
            return true;
        }
        seen.insert(num);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate_no_duplicates() {
        let nums = vec![1, 2, 3, 4];

        assert_eq!(has_duplicate(nums), false)
    }

    #[test]
    fn test_has_duplicate_one_duplicate() {
        let nums = vec![1, 2, 3, 4, 1];

        assert_eq!(has_duplicate(nums), true)
    }

    #[test]
    fn test_has_duplicate_two_duplicates() {
        let nums = vec![1, 2, 1, 4, 3];

        assert_eq!(has_duplicate(nums), true)
    }

    #[test]
    fn test_has_duplicate_all_duplicates() {
        let nums = vec![1, 1, 1];

        assert_eq!(has_duplicate(nums), true)
    }

    #[test]
    fn test_has_duplicate_empty() {
        let nums = vec![];

        assert_eq!(has_duplicate(nums), false)
    }
}
