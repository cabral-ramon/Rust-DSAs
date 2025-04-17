use std::cmp::Ordering;

fn main() {}

fn two_sum(numbers: Vec<i32>, target: i32) -> Option<(usize, usize)> {

    let mut l = 0;
    let mut r = numbers.len() - 1;

    while l < r {
        let left_num = numbers[l];
        let right_num = numbers[r];

        let curr_sum = left_num + right_num;

        match curr_sum.cmp(&target) {
            Ordering::Equal => return Some((l+1, r+1)), 
            Ordering::Less => l += 1,
            Ordering::Greater => r -= 1,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_finds_leftmost() {
        let numbers = vec![1, 2, 3];
        let actual = two_sum(numbers, 3);
        assert_eq!(actual, Some((1, 2)))
    }

    #[test]
    fn test_two_sum_finds_rightmost() {
        let numbers = vec![1, 2, 3];
        let actual = two_sum(numbers, 5);
        assert_eq!(actual, Some((2, 3)))
    }


    #[test]
    fn test_two_sum_finds_in_the_middle() {
        let numbers = vec![1, 2, 3, 4, 50];
        let actual = two_sum(numbers, 7);
        assert_eq!(actual, Some((3, 4)))
    }
}
