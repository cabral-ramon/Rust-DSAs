use std::collections::HashMap;

fn main() {}

fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.into_iter().enumerate() {
        let diff = target - num;
        if map.contains_key(&diff) {
            return Some((*map.get(&diff).unwrap(), i));
        }
        map.insert(num, i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![1, 2, 3];
        assert_eq!(two_sum(nums, 5), Some((1, 2)))
    }

    #[test]
    fn test_two_sum_only_two() {
        let nums = vec![1, 2];
        assert_eq!(two_sum(nums, 3), Some((0, 1)))
    }

    #[test]
    fn test_two_sum_last_two() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(two_sum(nums, 11), Some((4, 5)))
    }

    #[test]
    fn test_two_sum_first_two() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(two_sum(nums, 3), Some((0, 1)))
    }

    #[test]
    fn test_two_sum_doesnt_exist() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(two_sum(nums, 39), None)
    }
}
