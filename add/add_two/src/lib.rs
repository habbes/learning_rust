pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_works() {
        let result = add_two(2);
        assert_eq!(4, result);
    }
}
