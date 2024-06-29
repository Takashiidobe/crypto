use std::{
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
    },
};

use crate::polynomial::P16;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Gf256Aes(pub u8);

impl Gf256Aes {
    pub const POLYNOMIAL: P16 = P16(0x11b);

    pub const GENERATOR: Gf256Aes = Gf256Aes(0x03);

    pub const fn new(n: u8) -> Self {
        Self(n)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn add(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 ^ other.0)
    }

    pub const fn sub(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 ^ other.0)
    }

    pub const fn naive_mul(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(
            P16(self.0 as _)
                .naive_wrapping_mul(P16(other.0 as _))
                .naive_rem(Self::POLYNOMIAL)
                .0 as u8,
        )
    }

    pub const fn pow(self, exp: u8) -> Gf256Aes {
        let mut a = self;
        let mut exp = exp;
        let mut x = Gf256Aes(1);
        loop {
            if exp & 1 != 0 {
                x = x.naive_mul(a);
            }

            exp >>= 1;
            if exp == 0 {
                return x;
            }
            a = a.naive_mul(a);
        }
    }

    pub const fn naive_checked_recip(self) -> Option<Gf256Aes> {
        if self.0 == 0 {
            return None;
        }

        Some(self.pow(255 - 1))
    }

    pub const fn naive_recip(self) -> Gf256Aes {
        match self.naive_checked_recip() {
            Some(x) => x,
            None => panic!("Division by 0"),
        }
    }

    pub fn checked_recip(self) -> Option<Gf256Aes> {
        self.naive_checked_recip()
    }

    pub fn recip(self) -> Gf256Aes {
        self.checked_recip().expect("gf division by zero")
    }

    pub const fn mul(self, other: Gf256Aes) -> Gf256Aes {
        self.naive_mul(other)
    }

    pub const fn naive_checked_div(self, other: Gf256Aes) -> Option<Gf256Aes> {
        match other.naive_checked_recip() {
            Some(other_recip) => Some(self.naive_mul(other_recip)),
            None => None,
        }
    }

    pub const fn naive_div(self, other: Gf256Aes) -> Gf256Aes {
        match self.naive_checked_div(other) {
            Some(x) => x,
            None => panic!("Division by 0"),
        }
    }

    pub const fn div(self, other: Gf256Aes) -> Gf256Aes {
        self.naive_div(other)
    }
}

impl Neg for Gf256Aes {
    type Output = Gf256Aes;

    fn neg(self) -> Gf256Aes {
        self
    }
}

impl Neg for &Gf256Aes {
    type Output = Gf256Aes;

    fn neg(self) -> Gf256Aes {
        *self
    }
}

impl Add<Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn add(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes::add(self, other)
    }
}

impl Add<Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn add(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes::add(*self, other)
    }
}

impl Add<&Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn add(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes::add(self, *other)
    }
}

impl Add<&Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn add(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes::add(*self, *other)
    }
}

impl AddAssign<Gf256Aes> for Gf256Aes {
    fn add_assign(&mut self, other: Gf256Aes) {
        *self = self.add(other)
    }
}

impl AddAssign<&Gf256Aes> for Gf256Aes {
    fn add_assign(&mut self, other: &Gf256Aes) {
        *self = self.add(*other)
    }
}

impl Sum<Gf256Aes> for Gf256Aes {
    fn sum<I>(iter: I) -> Gf256Aes
    where
        I: Iterator<Item = Gf256Aes>,
    {
        iter.fold(Gf256Aes(0), |a, x| a + x)
    }
}

impl<'a> Sum<&'a Gf256Aes> for Gf256Aes {
    fn sum<I>(iter: I) -> Gf256Aes
    where
        I: Iterator<Item = &'a Gf256Aes>,
    {
        iter.fold(Gf256Aes(0), |a, x| a + *x)
    }
}

impl Sub for Gf256Aes {
    type Output = Gf256Aes;

    fn sub(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes::sub(self, other)
    }
}

impl Sub<Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn sub(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes::sub(*self, other)
    }
}

impl Sub<&Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn sub(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes::sub(self, *other)
    }
}

impl Sub<&Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn sub(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes::sub(*self, *other)
    }
}

impl SubAssign<Gf256Aes> for Gf256Aes {
    fn sub_assign(&mut self, other: Gf256Aes) {
        *self = self.sub(other)
    }
}

impl SubAssign<&Gf256Aes> for Gf256Aes {
    fn sub_assign(&mut self, other: &Gf256Aes) {
        *self = self.sub(*other)
    }
}

impl Mul for Gf256Aes {
    type Output = Gf256Aes;

    fn mul(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes::mul(self, other)
    }
}

impl Mul<Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn mul(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes::mul(*self, other)
    }
}

impl Mul<&Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn mul(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes::mul(self, *other)
    }
}

impl Mul<&Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn mul(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes::mul(*self, *other)
    }
}

impl MulAssign<Gf256Aes> for Gf256Aes {
    fn mul_assign(&mut self, other: Gf256Aes) {
        *self = self.mul(other)
    }
}

impl MulAssign<&Gf256Aes> for Gf256Aes {
    fn mul_assign(&mut self, other: &Gf256Aes) {
        *self = self.mul(*other)
    }
}

impl Product<Gf256Aes> for Gf256Aes {
    fn product<I>(iter: I) -> Gf256Aes
    where
        I: Iterator<Item = Gf256Aes>,
    {
        iter.fold(Gf256Aes(0), |a, x| a * x)
    }
}

impl<'a> Product<&'a Gf256Aes> for Gf256Aes {
    fn product<I>(iter: I) -> Gf256Aes
    where
        I: Iterator<Item = &'a Gf256Aes>,
    {
        iter.fold(Gf256Aes(0), |a, x| a * *x)
    }
}

//// Division ////

impl Div for Gf256Aes {
    type Output = Gf256Aes;

    fn div(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes::div(self, other)
    }
}

impl Div<Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn div(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes::div(*self, other)
    }
}

impl Div<&Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn div(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes::div(self, *other)
    }
}

impl Div<&Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn div(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes::div(*self, *other)
    }
}

impl DivAssign<Gf256Aes> for Gf256Aes {
    fn div_assign(&mut self, other: Gf256Aes) {
        *self = self.div(other)
    }
}

impl DivAssign<&Gf256Aes> for Gf256Aes {
    fn div_assign(&mut self, other: &Gf256Aes) {
        *self = self.div(*other)
    }
}

//// Bitwise operations ////

impl Not for Gf256Aes {
    type Output = Gf256Aes;

    fn not(self) -> Gf256Aes {
        Gf256Aes(!self.0)
    }
}

impl Not for &Gf256Aes {
    type Output = Gf256Aes;

    fn not(self) -> Gf256Aes {
        Gf256Aes(!self.0)
    }
}

impl BitAnd<Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitand(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 & other.0)
    }
}

impl BitAnd<Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitand(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 & other.0)
    }
}

impl BitAnd<&Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitand(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 & other.0)
    }
}

impl BitAnd<&Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitand(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 & other.0)
    }
}

impl BitAndAssign<Gf256Aes> for Gf256Aes {
    fn bitand_assign(&mut self, other: Gf256Aes) {
        *self = *self & other;
    }
}

impl BitAndAssign<&Gf256Aes> for Gf256Aes {
    fn bitand_assign(&mut self, other: &Gf256Aes) {
        *self = *self & *other;
    }
}

impl BitAnd<Gf256Aes> for u8 {
    type Output = Gf256Aes;

    fn bitand(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self & other.0)
    }
}

impl BitAnd<Gf256Aes> for &u8 {
    type Output = Gf256Aes;

    fn bitand(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self & other.0)
    }
}

impl BitAnd<&Gf256Aes> for u8 {
    type Output = Gf256Aes;

    fn bitand(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self & other.0)
    }
}

impl BitAnd<&Gf256Aes> for &u8 {
    type Output = Gf256Aes;

    fn bitand(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self & other.0)
    }
}

impl BitAnd<u8> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitand(self, other: u8) -> Gf256Aes {
        Gf256Aes(self.0 & other)
    }
}

impl BitAnd<u8> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitand(self, other: u8) -> Gf256Aes {
        Gf256Aes(self.0 & other)
    }
}

impl BitAnd<&u8> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitand(self, other: &u8) -> Gf256Aes {
        Gf256Aes(self.0 & other)
    }
}

impl BitAnd<&u8> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitand(self, other: &u8) -> Gf256Aes {
        Gf256Aes(self.0 & other)
    }
}

impl BitAndAssign<u8> for Gf256Aes {
    fn bitand_assign(&mut self, other: u8) {
        *self = *self & other;
    }
}

impl BitAndAssign<&u8> for Gf256Aes {
    fn bitand_assign(&mut self, other: &u8) {
        *self = *self & *other;
    }
}

impl BitOr<Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitor(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 | other.0)
    }
}

impl BitOr<Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitor(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 | other.0)
    }
}

impl BitOr<&Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitor(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 | other.0)
    }
}

impl BitOr<&Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitor(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 | other.0)
    }
}

impl BitOrAssign<Gf256Aes> for Gf256Aes {
    fn bitor_assign(&mut self, other: Gf256Aes) {
        *self = *self | other;
    }
}

impl BitOrAssign<&Gf256Aes> for Gf256Aes {
    fn bitor_assign(&mut self, other: &Gf256Aes) {
        *self = *self | *other;
    }
}

impl BitOr<Gf256Aes> for u8 {
    type Output = Gf256Aes;

    fn bitor(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self | other.0)
    }
}

impl BitOr<Gf256Aes> for &u8 {
    type Output = Gf256Aes;

    fn bitor(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self | other.0)
    }
}

impl BitOr<&Gf256Aes> for u8 {
    type Output = Gf256Aes;

    fn bitor(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self | other.0)
    }
}

impl BitOr<&Gf256Aes> for &u8 {
    type Output = Gf256Aes;

    fn bitor(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self | other.0)
    }
}

impl BitOr<u8> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitor(self, other: u8) -> Gf256Aes {
        Gf256Aes(self.0 | other)
    }
}

impl BitOr<u8> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitor(self, other: u8) -> Gf256Aes {
        Gf256Aes(self.0 | other)
    }
}

impl BitOr<&u8> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitor(self, other: &u8) -> Gf256Aes {
        Gf256Aes(self.0 | other)
    }
}

impl BitOr<&u8> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitor(self, other: &u8) -> Gf256Aes {
        Gf256Aes(self.0 | other)
    }
}

impl BitOrAssign<u8> for Gf256Aes {
    fn bitor_assign(&mut self, other: u8) {
        *self = *self | other;
    }
}

impl BitOrAssign<&u8> for Gf256Aes {
    fn bitor_assign(&mut self, other: &u8) {
        *self = *self | *other;
    }
}

impl BitXor<Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitxor(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 ^ other.0)
    }
}

impl BitXor<Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitxor(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 ^ other.0)
    }
}

impl BitXor<&Gf256Aes> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitxor(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 ^ other.0)
    }
}

impl BitXor<&Gf256Aes> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitxor(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self.0 ^ other.0)
    }
}

impl BitXorAssign<Gf256Aes> for Gf256Aes {
    fn bitxor_assign(&mut self, other: Gf256Aes) {
        *self = *self ^ other;
    }
}

impl BitXorAssign<&Gf256Aes> for Gf256Aes {
    fn bitxor_assign(&mut self, other: &Gf256Aes) {
        *self = *self ^ *other;
    }
}

impl BitXor<Gf256Aes> for u8 {
    type Output = Gf256Aes;

    fn bitxor(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self ^ other.0)
    }
}

impl BitXor<Gf256Aes> for &u8 {
    type Output = Gf256Aes;

    fn bitxor(self, other: Gf256Aes) -> Gf256Aes {
        Gf256Aes(self ^ other.0)
    }
}

impl BitXor<&Gf256Aes> for u8 {
    type Output = Gf256Aes;

    fn bitxor(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self ^ other.0)
    }
}

impl BitXor<&Gf256Aes> for &u8 {
    type Output = Gf256Aes;

    fn bitxor(self, other: &Gf256Aes) -> Gf256Aes {
        Gf256Aes(self ^ other.0)
    }
}

impl BitXor<u8> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitxor(self, other: u8) -> Gf256Aes {
        Gf256Aes(self.0 ^ other)
    }
}

impl BitXor<u8> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitxor(self, other: u8) -> Gf256Aes {
        Gf256Aes(self.0 ^ other)
    }
}

impl BitXor<&u8> for Gf256Aes {
    type Output = Gf256Aes;

    fn bitxor(self, other: &u8) -> Gf256Aes {
        Gf256Aes(self.0 ^ other)
    }
}

impl BitXor<&u8> for &Gf256Aes {
    type Output = Gf256Aes;

    fn bitxor(self, other: &u8) -> Gf256Aes {
        Gf256Aes(self.0 ^ other)
    }
}

impl BitXorAssign<u8> for Gf256Aes {
    fn bitxor_assign(&mut self, other: u8) {
        *self = *self ^ other;
    }
}

impl BitXorAssign<&u8> for Gf256Aes {
    fn bitxor_assign(&mut self, other: &u8) {
        *self = *self ^ *other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Gf256Aes(0x12) + Gf256Aes(0x34), Gf256Aes(0x26));
    }

    #[test]
    fn sub() {
        assert_eq!(Gf256Aes(0x12) - Gf256Aes(0x34), Gf256Aes(0x26));
    }

    #[test]
    fn mul() {
        assert_eq!(Gf256Aes(0x12) * Gf256Aes(0x34), Gf256Aes(0x05));
    }

    #[test]
    fn div() {
        assert_eq!(Gf256Aes(0x12) / Gf256Aes(0x34), Gf256Aes(0x54));
    }
}
