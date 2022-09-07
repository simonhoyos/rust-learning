pub fn anagram(str: &str) -> bool {
    let reversed_word = str.chars().filter(|x| *x != ' ').rev().collect::<String>();

    str.chars().filter(|x| *x != ' ').collect::<String>() == reversed_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagram_works() {
        assert_eq!(anagram("anita lava la tina"), true)
    }
}
