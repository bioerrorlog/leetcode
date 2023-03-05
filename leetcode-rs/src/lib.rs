mod solutions;

// Sample
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Normal test sample
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Parametlized test sample by rstest
    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 1, 2)]
    #[case(2, 1, 3)]
    #[case(3, 2, 5)]
    #[case(4, 3, 7)]
    fn parametrized_works(#[case] left: usize, #[case] right: usize, #[case] expected: usize) {
        let result = add(left, right);
        assert_eq!(result, expected);
    }
}
