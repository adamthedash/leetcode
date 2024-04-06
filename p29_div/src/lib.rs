use std::ops::{Not, Shl};

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    println!("{:b} {:b} {:b}", dividend, divisor, dividend / divisor);

    let is_neg = divisor < 0;
    // two's complement
    let divisor = if is_neg {
        println!("flipping {:b}", divisor);
        divisor.not() + 1
    } else { divisor };
    println!("-> {:b}", divisor);

    println!("{:b} {:b} {:b}", dividend, divisor, dividend / divisor);

    let mut dividend = dividend;
    let mut quotient = 0;
    for i in 0..32 {
        println!("\t{} {:b} {:b}", i, dividend, quotient);
        if dividend < divisor { break; }

        quotient += 1 << i;
        dividend -= divisor;
        dividend >>= 1;
    }

    if is_neg {
        println!("flipping {:b}", quotient);
        quotient = quotient.not();
        println!("-> {:b}", quotient);
    }

    println!("{}", quotient);

    quotient
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let a = 10;
        let b = 3;
        let solution = 3;
        let guess = divide(a, b);
        assert_eq!(guess, solution)
    }

    #[test]
    fn example2() {
        let a = 7;
        let b = -3;
        let solution = -2;
        let guess = divide(a, b);
        assert_eq!(guess, solution)
    }

    #[test]
    fn example3() {
        let a = 70;
        let b = -10;
        let solution = -7;
        let guess = divide(a, b);
        assert_eq!(guess, solution)
    }
}
