use std::{f64::consts::SQRT_2, ops};

use num::pow;

/// The root-two conjugate. `adj2(a + b√2) == a - b√2`
pub trait Adj2 {
    fn adj2(self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootTwo<T>(T, T);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dyadic(i64, u32);

// #########################################
// #######                           #######
// ####        Traits for RootTwo       ####
// #######                           #######
// #########################################

impl<T: ops::Add<Output = T>> ops::Add for RootTwo<T> {
    type Output = RootTwo<T>;
    fn add(self, rhs: RootTwo<T>) -> Self::Output {
        RootTwo(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for RootTwo<T> {
    type Output = RootTwo<T>;
    fn sub(self, rhs: RootTwo<T>) -> Self::Output {
        RootTwo(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: ops::Neg<Output = T>> ops::Neg for RootTwo<T> {
    type Output = RootTwo<T>;
    fn neg(self) -> Self::Output {
        RootTwo(-self.0, -self.1)
    }
}

impl<T> ops::Mul for RootTwo<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T> + ops::Mul<i64, Output = T> + Copy,
{
    type Output = RootTwo<T>;
    fn mul(self, rhs: RootTwo<T>) -> Self::Output {
        RootTwo(
            self.0 * rhs.0 + self.1 * rhs.1 * 2,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl pow::Pow<u32> for RootTwo<i64> {
    type Output = RootTwo<i64>;
    fn pow(self, power: u32) -> Self::Output {
        if power == 0 {
            return RootTwo(0, 0);
        }
        // if power < 0 {
        //     return 1 / pow(self, -power);
        // }
        let mut result = self;
        let mut power = power - 1;
        while power > 0 {
            result = result * self;
            power -= 1;
        }
        result
    }
}

impl<T: Into<f64>> Into<f64> for RootTwo<T> {
    fn into(self) -> f64 {
        self.0.into() + self.1.into() * SQRT_2
    }
}

impl<T: ops::Neg<Output = T>> Adj2 for RootTwo<T> {
    fn adj2(self) -> Self {
        RootTwo(self.0, -self.1)
    }
}

// #########################################
// #######                           #######
// ####        Traits for Dyadic        ####
// #######                           #######
// #########################################

impl Dyadic {
    fn simplify(mut x: i64, mut k: u32) -> Dyadic {
        // keep dividing by 2 while numerator is power of 2
        //
        // TODO: consider adding an enum, then returning i64 if k is 0
        while x > 0 && k > 0 && (x & (x - 1)) == 0 {
            x /= 2;
            k -= 1;
        }
        Dyadic(x, k)
    }
}

impl ops::Add for Dyadic {
    type Output = Dyadic;
    fn add(self, rhs: Dyadic) -> Self::Output {
        if self.1 == rhs.1 {
            return Dyadic::simplify(self.0 + rhs.0, self.1);
        }
        let (a, b) = if self.1 < rhs.1 {
            (self, rhs)
        } else {
            (rhs, self)
        };
        let k_delta = 1i64 << (b.1 - a.1);
        Dyadic::simplify(a.0 * k_delta + b.0, b.1)
    }
}

impl ops::Sub for Dyadic {
    type Output = Dyadic;
    fn sub(self, rhs: Dyadic) -> Self::Output {
        self + -rhs
    }
}

impl ops::Neg for Dyadic {
    type Output = Dyadic;
    fn neg(self) -> Self::Output {
        Dyadic(-self.0, self.1)
    }
}

impl ops::Mul for Dyadic {
    type Output = Dyadic;
    fn mul(self, rhs: Self) -> Self::Output {
        Dyadic::simplify(self.0 * rhs.0, self.1 + rhs.1)
    }
}

impl ops::Mul<i64> for Dyadic {
    type Output = Dyadic;
    fn mul(self, rhs: i64) -> Self::Output {
        Dyadic::simplify(self.0 * rhs, self.1)
    }
}

impl ops::Mul<Dyadic> for i64 {
    type Output = Dyadic;
    fn mul(self, rhs: Dyadic) -> Self::Output {
        Dyadic::simplify(self * rhs.0, rhs.1)
    }
}

impl Into<f64> for Dyadic {
    fn into(self) -> f64 {
        let num = self.0 as f64;
        let denom = (1i64 << self.1) as f64;
        num / denom
    }
}

#[cfg(test)]
mod roottwo_tests {
    use super::*;
    #[test]
    fn basic_add_zroottwo() {
        let first = RootTwo(1, 2);
        let second = RootTwo(3, 4);
        assert_eq!(first + second, RootTwo(4, 6));
        assert_eq!(first - second, RootTwo(-2, -2));
    }

    #[test]
    fn basic_add_droottwo() {
        let first = RootTwo(Dyadic(3, 2), Dyadic(3, 7));
        let second = RootTwo(Dyadic(4, 2), Dyadic(3, 8));
        assert_eq!(first + second, RootTwo(Dyadic(7, 2), Dyadic(9, 8)));
        assert_eq!(first - second, RootTwo(Dyadic(-1, 2), Dyadic(3, 8)));
    }

    #[test]
    fn mul_two_root_twos() {
        let first = RootTwo(3, 4);
        let second = RootTwo(5, 6);
        let expected = RootTwo(63, 38);
        assert_eq!(first * second, expected);
        assert_eq!(second * first, expected);
    }
}

#[cfg(test)]
mod dyadic_tests {
    use super::*;
    #[test]
    fn basic_add_dyadic() {
        let first = Dyadic(3, 2);
        let second = Dyadic(1, 2);
        assert_eq!(first + second, Dyadic(1, 0));
        assert_eq!(-Dyadic(3, 2), Dyadic(-3, 2));
    }

    #[test]
    fn into_float_works() {
        assert_eq!(Into::<f64>::into(Dyadic(3, 2)), 0.75);
    }

    #[test]
    fn powers_of_two() {
        for i in 0..20 {
            assert_eq!(1 << i, i64::pow(2, i))
        }
    }
}
