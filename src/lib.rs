use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootTwo<T>(T, T);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dyadic(i64, u32);

// #########################################
// #######                           #######
// ####        Traits for RootTwo       ####
// #######                           #######
// #########################################

impl<T: ops::Add<Output = T>> ops::Add<RootTwo<T>> for RootTwo<T> {
    type Output = RootTwo<T>;
    fn add(self, rhs: RootTwo<T>) -> Self::Output {
        RootTwo(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub<RootTwo<T>> for RootTwo<T> {
    type Output = RootTwo<T>;
    fn sub(self, rhs: RootTwo<T>) -> Self::Output {
        RootTwo(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: ops::Neg<Output = T>> ops::Neg for RootTwo<T> {
    type Output = RootTwo<T>;
    fn neg(self) -> Self::Output {
        RootTwo::<T>(-self.0, -self.1)
    }
}

// #########################################
// #######                           #######
// ####        Traits for Dyadic        ####
// #######                           #######
// #########################################

fn dyadic(mut x: i64, mut k: u32) -> Dyadic {
    // simplify: keep dividing by 2 while numerator is power of 2
    while x > 0 && k > 0 && (x & (x - 1)) == 0 {
        x /= 2;
        k -= 1;
    }
    Dyadic(x, k)
}

impl ops::Add<Dyadic> for Dyadic {
    type Output = Dyadic;
    fn add(self, rhs: Dyadic) -> Self::Output {
        if self.1 == rhs.1 {
            return dyadic(self.0 + rhs.0, self.1);
        }
        let (a, b) = if self.1 < rhs.1 {
            (self, rhs)
        } else {
            (rhs, self)
        };
        let k_delta = i64::pow(2, b.1 - a.1);
        dyadic(a.0 * k_delta + b.0, b.1)
    }
}

impl ops::Sub<Dyadic> for Dyadic {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_add_zroottwo() {
        let first = RootTwo(1, 2);
        let second = RootTwo(3, 4);
        assert_eq!(first + second, RootTwo(4, 6));
        assert_eq!(first - second, RootTwo(-2, -2));
    }

    #[test]
    fn basic_add_dyadic() {
        let first = Dyadic(3, 2);
        let second = Dyadic(1, 2);
        assert_eq!(first + second, Dyadic(1, 0));
    }

    #[test]
    fn basic_add_droottwo() {
        let first = RootTwo(Dyadic(3, 2), Dyadic(3, 7));
        let second = RootTwo(Dyadic(4, 2), Dyadic(3, 8));
        assert_eq!(first + second, RootTwo(Dyadic(7, 2), Dyadic(9, 8)));
        assert_eq!(first - second, RootTwo(Dyadic(-1, 2), Dyadic(3, 8)));
    }
}
