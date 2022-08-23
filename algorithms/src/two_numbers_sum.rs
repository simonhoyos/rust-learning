/*
  Write a function that takes in a non-empty array of distinct integers and an
  integer representing a target sum. If any two numbers in the input array sum
  up to the target sum, the function should return them in an array, in any
  order. If no two numbers sum up to the target sum, the function should return
  an empty array.


  Note that the target sum has to be obtained by summing two different integers
  in the array; you can't add a single integer to itself in order to obtain the
  target sum.

  You can assume that there will be at most one pair of numbers summing up to
  the target sum.
*/

use std::collections::HashMap;

fn two_number_sum(v: &Vec<isize>, target: isize) -> Vec<isize> {
    let mut r: Vec<isize> = Vec::with_capacity(2);

    if v.len() <= 1 {
        return r;
    }

    let mut sums = HashMap::new();

    for n in v {
        let d: isize = target - n;
        match sums.get(&d) {
            Some(_v) => {
                r.push(*n);
                r.push(target - n);
                return r;
            },
            None => {
                sums.insert(n, true);
            }
        }

        println!("{:?}", sums);
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_number_sum_empty_vector() {
        let a: Vec<isize> = Vec::new();
        let r = two_number_sum(&a, 10);

        assert_eq!(r.len(), 0);
    }

    #[test]
    fn two_number_sum_one_element_vector() {
        let a: Vec<isize> = vec![1];
        let r = two_number_sum(&a, 11);

        assert_eq!(r.len(), 0);
    }

    #[test]
    fn two_number_sum_more_elements_vector() {
        let a: Vec<isize> = vec![3, 5, -4, 8, 11, 1, -1, 6];
        let r = two_number_sum(&a, 10);

        println!("{:?}", r);

        assert_eq!(r.len(), 2);
        assert!(r.contains(&-1));
        assert!(r.contains(&11));
    }
}
