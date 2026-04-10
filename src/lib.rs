/// Returns the sum of two integers.
pub const fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let tests = [
            ("positive numbers", 1, 2, 3),
            ("negative numbers", -1, -2, -3),
            ("zero", 0, 0, 0),
        ];

        for (name, a, b, want) in tests {
            assert_eq!(add(a, b), want, "failed: {name}");
        }
    }
}
