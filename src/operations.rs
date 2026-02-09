pub fn modulus(a: i32, b: i32) -> i32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulus() {
        assert_eq!(modulus(10, 3), 1);
        assert_eq!(modulus(20, 5), 0);
        assert_eq!(modulus(7, 2), 1);

    }
}