mod ex1;

fn main() {
    for i in (1..10).step_by(2) {
        print!("{}", i);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_work() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore = "not yet reviewed"]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4);
    }
}
