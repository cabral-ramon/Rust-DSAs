/*
 * We are given an array of integers, nums, sorted in ascending order, and an integer value, target.
 * If the target exists in the array, return its index.
 * If the target does not exist, return -1.
 */

use std::cmp::{Ordering, Ord};

#[allow(dead_code)]
fn main() {
    let v = vec![1, 2, 3, 4];

    let res = binary_search_recursive(&v, 3);
    assert!(res.unwrap() == 2);

    let res = binary_search_recursive(&v, 5);
    assert!(res.is_none());

    let res = binary_search(&v, 3);
    assert!(res.unwrap() == 2);

    let res = binary_search(&v, 5);
    assert!(res.is_none());
}

fn binary_search<T>(nums: &[T], target: T) -> Option<usize> where T: Ord {
   let mut low = 0;
   let mut high = nums.len() - 1;
   let mut mid = high / 2;

   while low <= high {
      let curr = &nums[mid];
      match target.cmp(curr) {
         Ordering::Equal => return Some(mid),
         Ordering::Less => {
            high = mid - 1;
         },
         Ordering::Greater => {
            low = mid + 1;
         }
      }
      mid = (low + high) / 2;
   }

   None
}

fn binary_search_recursive<T>(nums: &[T], target: T) -> Option<usize>
where
    T: Ord,
{
    if nums.is_empty() {
        return None;
    }
    let mid = nums.len() / 2;

    if nums[mid] == target {
        return Some(mid);
    }

    if target < nums[mid] {
        binary_search_recursive(&nums[..mid], target)
    } else {
        binary_search_recursive(&nums[mid + 1..], target)
    }
}
