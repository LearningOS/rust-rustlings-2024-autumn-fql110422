pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4)); // 4 is even, so this should pass
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5)); // 5 is odd, so this should pass
    }
}
