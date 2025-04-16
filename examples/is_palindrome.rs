fn main() {}

fn is_palindrome(s: String) -> bool {
    let mut l = 0;
    let mut r = s.len() - 1;
    let s: Vec<char> = s.chars().collect();

    while l < r {
        while l < r && !s[l].is_alphanumeric() {
            l += 1;
        }
        while r > l && !s[r].is_alphanumeric() {
            r -= 1;
        }

        if !s[l].eq_ignore_ascii_case(&s[r]) {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_true() {
        let s = "Was it a car or a cat I saw?".to_string();
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_is_palindrome_false() {
        let s = "tab a cat".to_string();
        assert_eq!(is_palindrome(s), false);
    }
}
