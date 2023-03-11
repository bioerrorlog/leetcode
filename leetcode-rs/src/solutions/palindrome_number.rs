// Problem 9.
// https://leetcode.com/problems/palindrome-number/
#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x > 0 && x % 10 == 0) {
        return false;
    }

    x.to_string().chars().rev().eq(x.to_string().chars())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(121, true)]
    #[case(-121, false)]
    #[case(10, false)]
    fn is_palindrome_test(#[case] input: i32, #[case] expected: bool) {
        assert_eq!(is_palindrome(input), expected);
    }
}
