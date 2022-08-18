use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in env::args().skip(1) {
        numbers
            .push(
                u64::from_str(&arg).expect("error parsing argument")
            );
    }

    if numbers.len() == 0 {
        eprintln!("Usage: sum NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = sum(d, *m);
    }

    println!("The sum of {:?} is {}", numbers, d);
}

fn sum(a: u64, b: u64) -> u64 {
    a + b
}

#[test]
fn test_sum() {
    assert_eq!(sum(1, 3), 4)
}
