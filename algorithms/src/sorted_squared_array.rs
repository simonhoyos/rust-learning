/*
  difficult: easy

  Write a function that takes in a non-empty array of integers that are sorted
  in ascending order and returns a new array of the same length with the squares
  of the original integers also sorted in ascending order.
*/

pub fn sorted_squared_array(a: &Vec<isize>) -> Vec<isize> {
    if a.len() <= 0 {
        panic!("Should have at least one element");
    }

    let mut v: Vec<isize> = (0 as isize..a.len() as isize).collect();

    let mut p1 = 0;
    let mut p2: usize = a.len() - 1;
    let mut i = a.len() - 1;

    loop {
        let n = a[p1];
        let m = a[p2];

        if n.abs() > m.abs() {
            v[i] = n * n;
            p1 += 1;
        } else {
            v[i] = m * m;
            p2 = p2.saturating_sub(1);
        }

        if p1 > p2 || i == 0 {
            break;
        }

        i -= 1;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn sorted_squared_array_empty() {
        sorted_squared_array(&Vec::new());
    }

    #[test]
    fn sorted_squared_array_one_element() {
        let v = vec![4];
        let result = sorted_squared_array(&v);
        assert_eq!(result, vec![16]);
    }

    #[test]
    fn sorted_squared_array_multiple_elements() {
        let v = vec![1,2,3,4];
        let result = sorted_squared_array(&v);
        assert_eq!(result, vec![1, 4, 9, 16]);
    }

    #[test]
    fn sorted_squared_array_multiple_elements_all_negatives() {
        let v = vec![-4,-3,-2,-1];
        let result = sorted_squared_array(&v);
        assert_eq!(result, vec![1, 4, 9, 16]);
    }
}
