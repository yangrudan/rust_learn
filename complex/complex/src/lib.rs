//complex/src/lib.rs
use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output = T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Complex {
            re: value.0,
            im: value.1,
        }
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;

    #[test]
    fn complex_basics() {
        let first = Complex::new(3, 5);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.re, 3);
        assert_eq!(second.re, 0);
    }

    #[test]
    fn complex_addition() {
        let a = Complex::new(1, 1);
        let b = Complex::new(1, 1);
        let res = a + b;
        assert_eq!(res, Complex::new(2, 2));
    }

    #[test]
    fn complex_from() {
        let a = (1, 1);
        let b = Complex::from(a);
        assert_eq!(b.re, 1);
        assert_eq!(b.im, 1);
    }

    #[test]
    fn complex_display() {
        let b = Complex::new(234, 236);
        println!("{}", b);
    }
}
