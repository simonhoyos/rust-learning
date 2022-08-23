/*
    Given two non-empty arrays of integers, write a function that determines
    whether the second array is a subsequence of the first one.

    A subsequence of an array is a set of numbers that aren't necessarily adjacent
    in the array but that are in the same order as they appear in the array. For
    instance, the numbers [1, 3, 4] form a subsequence of the array [1, 2, 3, 4],
    and so do the numbers [2, 4]. Note that a single number in an array and the
    array itself are both valid subsequences of the array.
*/

pub fn validate_subsequence(a: Vec<i32>, s: Vec<i32>) -> bool {
    if a.len() <= 0 || s.len() <= 0 {
        panic!("Must have at least one element");
    }

    if s.len() > a.len() {
        return false;
    }

    let mut p = 0;

    for v in a {
        if v == s[p] {
            p += 1;
        }
    }

    p == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_subsequence_works() {
        assert_eq!(validate_subsequence(vec![5, 1, 22, 25, 6, -1, 8, 10], vec![1, 6, -1, 10]), true)
    }

    #[test]
    #[should_panic]
    fn validate_subsequence_empty() {
        validate_subsequence(Vec::new(), Vec::new());
    }
}
