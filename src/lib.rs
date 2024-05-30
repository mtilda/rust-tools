pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
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

    #[rstest]
    #[case((0, 0), 0)]
    #[case((1, 1), 0)]
    #[case((123, 1), 122)]
    fn test_subtract(#[case] params: (usize, usize), #[case] expected: usize) {
        let (left, right) = params;
        let result = subtract(left, right);
        assert_eq!(result, expected);
    }
}
