pub fn fizzbuzz(a: usize) -> String {
    if a % 3 == 0 && a % 5 == 0 {
        return "fizzbuzz".to_string();
    } else if a % 3 == 0 {
        return "fizz".to_string();
    } else if a % 5 == 0 {
        return "buzz".to_string();
    } else {
        return a.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_works() {
        assert_eq!(fizzbuzz(3), "fizz");
        assert_eq!(fizzbuzz(5), "buzz");
        assert_eq!(fizzbuzz(15), "fizzbuzz");
        assert_eq!(fizzbuzz(1), "1");
    }
}
