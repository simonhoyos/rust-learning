/*
* Find the largest number in an arbitrary list containing at least one value
*/

use std::io::{Error, ErrorKind};

pub fn find_largest(list: Vec<isize>) -> Result<isize, Error> {
    if list.len() <= 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "List should include at least one number",
        ));
    }

    let mut max = list[0];

    for number in list {
        if number > max {
            max = number;
        }
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_largest_should_throw_error_if_list_is_empty() {
        let list = vec![];
        let result = find_largest(list);

        println!("Hola, {:?}", result);

        assert!(result.is_err())
    }

    #[test]
    fn find_largest_should_work() {
        let list = vec![1, 2, 3, 4];
        let result = find_largest(list).unwrap();

        assert_eq!(4, result)
    }

    #[test]
    fn find_largest_should_work_with_negatives() {
        let list = vec![0, -10, 4, 100, 3, 2, 1];
        let result = find_largest(list).unwrap();

        assert_eq!(100, result)
    }
}
