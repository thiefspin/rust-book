use std::result::Result;

#[derive(Debug)]
struct Error {
    max: u128,
}

impl Error {
    pub fn get_message(&self) -> String {
        return format!(
            "Could not calculate fibonacci for a number that big. Max {}",
            self.max
        );
    }
}

fn fibonacci_mutable_64_bit(n: u8) -> Result<u64, Error> {
    if n <= 1 {
        return Ok(n as u64);
    }
    if n > 93 {
        return Err(Error {
            max: (n - 1) as u128,
        });
    }
    let mut sum: u64 = 0;
    let mut last: u64 = 0;
    let mut curr: u64 = 1;

    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    return Ok(sum);
}

fn fibonacci_mutable_128_bit(n: u8) -> Result<u128, Error> {
    if n <= 1 {
        return Ok(n as u128);
    }
    if n > 186 {
        return Err(Error {
            max: (n - 1) as u128,
        });
    }
    let mut sum: u128 = 0;
    let mut last: u128 = 0;
    let mut curr: u128 = 1;

    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    return Ok(sum);
}

fn fibonacci_sequence_128_bit(max: u8) {
    for i in 0..max {
        match fibonacci_mutable_128_bit(i) {
            Ok(result) => println!("Fibonacci of {} is {}", i, result),
            Err(err) => {
                println!("{}", err.get_message());
                break;
            }
        }
    }
}

fn fibonacci_sequence_64_bit(max: u8) {
    for i in 0..max {
        match fibonacci_mutable_64_bit(i) {
            Ok(result) => println!("Fibonacci of {} is {}", i, result),
            Err(err) => {
                println!("{}", err.get_message());
                break;
            }
        }
    }
}

fn main() {
    let max: u8 = 255;
    println!("Calculating the fibonacci sequence as 128 bit integers");
    fibonacci_sequence_128_bit(max);
    println!("Calculating the fibonacci sequence as 64 bit integers");
    fibonacci_sequence_64_bit(max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_mutable() {
        assert_eq!(fibonacci_mutable_64_bit(0).unwrap(), 0);
        assert_eq!(fibonacci_mutable_64_bit(1).unwrap(), 1);
        assert_eq!(fibonacci_mutable_64_bit(2).unwrap(), 1);
        assert_eq!(fibonacci_mutable_64_bit(10).unwrap(), 55);
        assert_eq!(fibonacci_mutable_64_bit(15).unwrap(), 610);
        assert_eq!(fibonacci_mutable_64_bit(18).unwrap(), 2584);
        assert_eq!(fibonacci_mutable_64_bit(22).unwrap(), 17711);
        assert_eq!(fibonacci_mutable_64_bit(24).unwrap(), 46368);
    }

    #[test]
    #[should_panic]
    fn test_fibonacci_mutable_128_bit_big_number() {
        fibonacci_mutable_128_bit(187).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_fibonacci_mutable_64_bit_big_number() {
        fibonacci_mutable_64_bit(94).unwrap();
    }
}
