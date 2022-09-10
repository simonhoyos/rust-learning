/*
  level: easy

  Given an array of positive integers representing the values of coins in your
  possession, write a function that returns the minimum amount of change (the
  minimum sum of money) that you cannot  create. The given coins can have
  any positive integer value and aren't necessarily unique (i.e., you can have
  multiple coins of the same value).

  For example, if you're given coins = [1, 2, 5], the minimum
  amount of change that you can't create is 4. If you're given no
  coins, the minimum amount of change that you can't create is 1.
*/

#[warn(dead_code)]
pub fn non_constructible_change(coins: &mut Vec<usize>) -> usize {
    coins.sort_unstable();
    let mut sum = 0;

    for coin in coins {
        if *coin - 1 > sum {
            return sum + 1;
        } else {
            sum += *coin;
        }
    }

    return sum + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_constructible_change_works() {
        assert_eq!(
            non_constructible_change(&mut vec![1_usize, 2_usize, 5_usize]),
            4
        );
    }
}
