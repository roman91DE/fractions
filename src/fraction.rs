use num::Integer;
use std::fmt::Display;

#[derive(Debug)]

pub struct Fraction<T: Integer + Copy + Display> {
    numerator: T,
    denominator: T,
}

impl<T: Integer + Copy + Display> Fraction<T> {
    pub fn new(numerator: T, denominator: T) -> Self {
        if denominator == T::zero() {
            panic!("Denominator cannot be zero");
        }
        let gcd: T = Fraction::greatest_common_divisor(numerator, denominator);
        Fraction {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }

    pub fn multiply(&self, other: &Fraction<T>) -> Fraction<T> {
        Fraction::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    pub fn divide(&self, other: &Fraction<T>) -> Fraction<T> {
        Fraction::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }

    pub fn add(&self, other: &Fraction<T>) -> Fraction<T> {
        Fraction::new(
            ((self.numerator * other.denominator) + (other.numerator * self.denominator)),
            (self.denominator * other.denominator),
        )
    }

    pub fn subtract(&self, other: &Fraction<T>) -> Fraction<T> {
        Fraction::new(
            ((self.numerator * other.denominator) - (other.numerator * self.denominator)),
            (self.denominator * other.denominator),
        )
    }

    fn greatest_common_divisor(a: T, b: T) -> T {
        if a < b {
            return Fraction::greatest_common_divisor(b, a);
        }
        if b == T::zero() {
            return a;
        } else {
            return Fraction::greatest_common_divisor(b, a % b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let frac = Fraction::new(4, 8);
        assert_eq!(frac.numerator, 1);
        assert_eq!(frac.denominator, 2);

        let frac = Fraction::new(0, 1);
        assert_eq!(frac.numerator, 0);
        assert_eq!(frac.denominator, 1);
    }

    #[test]
    #[should_panic(expected = "Denominator cannot be zero")]
    fn test_new_with_zero_denominator() {
        Fraction::new(1, 0);
    }

    #[test]
    fn test_to_string() {
        let frac = Fraction::new(1, 2);
        assert_eq!(frac.to_string(), "1/2");

        let frac = Fraction::new(10, 20);
        assert_eq!(frac.to_string(), "1/2");
    }

    #[test]
    fn test_multiply() {
        let frac1 = Fraction::new(1, 2);
        let frac2 = Fraction::new(3, 4);
        let result = frac1.multiply(&frac2);
        assert_eq!(result.to_string(), "3/8");
    }

    #[test]
    fn test_divide() {
        let frac1 = Fraction::new(1, 2);
        let frac2 = Fraction::new(3, 4);
        let result = frac1.divide(&frac2);
        assert_eq!(result.to_string(), "2/3");
    }

    #[test]
    fn test_add() {
        let frac1 = Fraction::new(1, 2);
        let frac2 = Fraction::new(1, 3);
        let result = frac1.add(&frac2);
        assert_eq!(result.to_string(), "5/6");
    }

    #[test]
    fn test_subtract() {
        let frac1 = Fraction::new(1, 2);
        let frac2 = Fraction::new(1, 3);
        let result = frac1.subtract(&frac2);
        assert_eq!(result.to_string(), "1/6");
    }

    #[test]
    fn test_gcd() {
        assert_eq!(Fraction::greatest_common_divisor(48, 18), 6);
        assert_eq!(Fraction::greatest_common_divisor(56, 98), 14);
    }

    #[test]
    fn test_with_different_integer_types() {
        let frac = Fraction::new(100u32, 200u32);
        assert_eq!(frac.to_string(), "1/2");

        let frac = Fraction::new(100i64, 200i64);
        assert_eq!(frac.to_string(), "1/2");

        let frac = Fraction::new(100u8, 200u8);
        assert_eq!(frac.to_string(), "1/2");
    }
}