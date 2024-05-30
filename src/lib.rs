pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0, 0), 0)]
    #[case((1, 1), 2)]
    #[case((123, 456), 579)]
    fn test_add(#[case] params: (usize, usize), #[case] expected: usize) {
        let (left, right) = params;
        let result = add(left, right);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
