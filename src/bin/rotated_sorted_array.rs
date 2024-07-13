/* Given a sorted integer array, nums, and an integer value, target, the array is rotated by some arbitrary number.
 * Search and return the index of target in this array. If the target does not exist, return - 1
 */

fn main() {
    let arr = [
        176, 188, 199, 200, 210, 222, 1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162,
    ];
    let target = 200;
    let res = rotated_sorted_array(&arr, target);

    assert!(res == Some(3));
    println!("Found target: {target} at index: {}", res.unwrap());
}

fn rotated_sorted_array(arr: &[u32], target: u32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            return Some(mid);
        }

        if arr[low] <= arr[mid] {
            if target >= arr[low] && target <= arr[mid] {
                high = mid - 1;
                continue;
            } else {
                low = mid + 1;
                continue;
            }
        } else if target >= arr[mid] && target <= arr[high] {
                low = mid + 1;
                continue;
            } else {
                high = mid - 1;
                continue;
            }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_target_in_array_right() {
        let arr = [40, 50, 60, 10, 20, 30];
        let target = 20;
        let res = rotated_sorted_array(&arr, target);

        assert!(res == Some(4));
    }

    #[test]
    fn test_target_in_array_mid() {
        let arr = [40, 50, 60, 10, 20, 30];
        let target = 60;
        let res = rotated_sorted_array(&arr, target);

        assert!(res == Some(2));
    }

    #[test]
    fn test_target_not_in_array() {
        let arr = [40, 50, 60, 10, 20, 30];
        let target = 90;
        let res = rotated_sorted_array(&arr, target);

        assert!(res.is_none());
    }

    #[test]
    fn test_target_in_array_array_not_rotated() {
        let arr = [10, 20, 30, 40, 50, 60];
        let target = 20;
        let res = rotated_sorted_array(&arr, target);

        assert!(res == Some(1));
    }

    #[test]
    fn test_target_not_in_array_array_not_rotated() {
        let arr = [10, 20, 30, 40, 50, 60];
        let target = 5;
        let res = rotated_sorted_array(&arr, target);

        assert!(res.is_none());
    }
}
