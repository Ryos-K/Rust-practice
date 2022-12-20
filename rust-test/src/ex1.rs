pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        for i in (0..10).step_by(2) {
            assert_eq!(is_even(i), true);
        }
    }

    #[test]
    fn is_false_when_odd() {
        for i in (1..10).step_by(2) {
            assert_eq!(is_even(i), false);
        }
    }
}
