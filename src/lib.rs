pub fn add<T: std::ops::Add<Output = T>>(left: T, right: T) -> T {
    left + right
}

pub fn subtract<T: std::ops::Sub<Output = T>>(left: T, right: T) -> T {
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
    #[case((123, 456), 579)]
    #[case((123, -456), -333)]
    #[case((-123, 456), 333)]
    #[case((-123, -456), -579)]
    #[case((0.123, 0.456), 0.579)]
    #[case((0.123, -0.456), -0.333)]
    #[case((-0.123, 0.456), 0.333)]
    #[case((-0.123, -0.456), -0.579)]
    #[case((i8::MAX - 1, 1), i8::MAX)]
    #[case((i8::MIN, 1), i8::MIN + 1)]
    #[case((i16::MAX - 1, 1), i16::MAX)]
    #[case((i16::MIN, 1), i16::MIN + 1)]
    #[case((i32::MAX - 1, 1), i32::MAX)]
    #[case((i32::MIN, 1), i32::MIN + 1)]
    #[case((i64::MAX - 1, 1), i64::MAX)]
    #[case((i64::MIN, 1), i64::MIN + 1)]
    #[case((i128::MAX - 1, 1), i128::MAX)]
    #[case((i128::MIN, 1), i128::MIN + 1)]
    #[case((u8::MAX - 1, 1), u8::MAX)]
    #[case((u8::MIN, 1), u8::MIN + 1)]
    #[case((u16::MAX - 1, 1), u16::MAX)]
    #[case((u16::MIN, 1), u16::MIN + 1)]
    #[case((u32::MAX - 1, 1), u32::MAX)]
    #[case((u32::MIN, 1), u32::MIN + 1)]
    #[case((u64::MAX - 1, 1), u64::MAX)]
    #[case((u64::MIN, 1), u64::MIN + 1)]
    #[case((u128::MAX - 1, 1), u128::MAX)]
    #[case((u128::MIN, 1), u128::MIN + 1)]
    fn test_add<T: std::ops::Add<Output = T> + std::cmp::PartialEq + std::fmt::Debug>(
        #[case] params: (T, T),
        #[case] expected: T,
    ) {
        let (left, right) = params;
        let result = add(left, right);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case((0, 0), 0)]
    #[case((1, 1), 0)]
    #[case((123, 1), 122)]
    #[case((123, 456), -333)]
    #[case((123, -456), 579)]
    #[case((-123, 456), -579)]
    #[case((-123, -456), 333)]
    #[case((0.123, 0.456), -0.333)]
    #[case((0.123, -0.456), 0.579)]
    #[case((-0.123, 0.456), -0.579)]
    #[case((-0.123, -0.456), 0.333)]
    #[case((i8::MAX, 1), i8::MAX - 1)]
    #[case((i8::MIN + 1, 1), i8::MIN)]
    #[case((i16::MAX, 1), i16::MAX - 1)]
    #[case((i16::MIN + 1, 1), i16::MIN)]
    #[case((i32::MAX, 1), i32::MAX - 1)]
    #[case((i32::MIN + 1, 1), i32::MIN)]
    #[case((i64::MAX, 1), i64::MAX - 1)]
    #[case((i64::MIN + 1, 1), i64::MIN)]
    #[case((i128::MAX, 1), i128::MAX - 1)]
    #[case((i128::MIN + 1, 1), i128::MIN)]
    #[case((u8::MAX, 1), u8::MAX - 1)]
    #[case((u8::MIN + 1, 1), u8::MIN)]
    #[case((u16::MAX, 1), u16::MAX - 1)]
    #[case((u16::MIN + 1, 1), u16::MIN)]
    #[case((u32::MAX, 1), u32::MAX - 1)]
    #[case((u32::MIN + 1, 1), u32::MIN)]
    #[case((u64::MAX, 1), u64::MAX - 1)]
    #[case((u64::MIN + 1, 1), u64::MIN)]
    #[case((u128::MAX, 1), u128::MAX - 1)]
    #[case((u128::MIN + 1, 1), u128::MIN)]
    fn test_subtract<T: std::ops::Sub<Output = T> + std::cmp::PartialEq + std::fmt::Debug>(
        #[case] params: (T, T),
        #[case] expected: T,
    ) {
        let (left, right) = params;
        let result = subtract(left, right);
        assert_eq!(result, expected);
    }
}
