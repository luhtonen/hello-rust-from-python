// math functions

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn div(a: i32, b: i32) -> i32 {
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(3, 2), 1);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2, 2), 4);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(4, 2), 2);
    }
}
