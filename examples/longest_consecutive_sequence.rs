use std::collections::{HashSet};

fn main() {}

fn longest_consecutive_sequence(nums: Vec<i32>) -> i32 {
    let mut num_set: HashSet<i32> = HashSet::new();
    for num in &nums {
        num_set.insert(*num);
    }

    let mut longest = 0;

    for num in nums {
        if !num_set.contains(&(num - 1)) {
            let mut length  = 0;
            while num_set.contains(&(num + length)) {
                length += 1;
            }
            longest = i32::max(longest, length);
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive_sequence() {
        let nums = vec![2, 4, 5, 3, 100, 200];
        assert_eq!(longest_consecutive_sequence(nums), 4);
    }
}
