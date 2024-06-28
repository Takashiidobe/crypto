use std::{
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Sub, SubAssign,
    },
};

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct P8(pub u8);

impl P8 {
    pub const fn new(v: u8) -> Self {
        Self(v)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn add(self, other: P8) -> P8 {
        Self(self.0 ^ other.0)
    }

    pub const fn sub(self, other: P8) -> P8 {
        Self(self.0 ^ other.0)
    }

    pub const fn naive_wrapping_mul(self, other: P8) -> P8 {
        let a = self.0;
        let b = other.0;
        let mut x = 0;
        let mut i = 0;
        while i < 8 {
            let mask = (((a as i8) << (8 - 1 - i)) >> (8 - 1)) as u8;
            x ^= mask & (b << i);
            i += 1;
        }
        P8(x)
    }

    pub const fn mul(self, other: P8) -> P8 {
        self.naive_wrapping_mul(other)
    }

    pub fn pow(self, exp: u8) -> P8 {
        let mut a = self;
        let mut exp = exp;
        let mut x = P8(1);
        loop {
            if exp & 1 != 0 {
                x = x.mul(a);
            }

            exp >>= 1;
            if exp == 0 {
                return x;
            }
            a = a.mul(a);
        }
    }

    pub const fn naive_checked_div(self, other: P8) -> Option<P8> {
        if other.0 == 0 {
            None
        } else {
            let mut a = self.0;
            let b = other.0;
            let mut x = 0;
            while a.leading_zeros() <= b.leading_zeros() {
                x ^= 1 << (b.leading_zeros() - a.leading_zeros());
                a ^= b << (b.leading_zeros() - a.leading_zeros());
            }
            Some(P8(x))
        }
    }

    pub const fn div(self, other: P8) -> P8 {
        match self.naive_checked_div(other) {
            Some(x) => x,
            None => panic!("Division by 0."),
        }
    }

    pub const fn naive_checked_rem(self, other: P8) -> Option<P8> {
        if other.0 == 0 {
            None
        } else {
            let mut a = self.0;
            let b = other.0;
            while a.leading_zeros() <= b.leading_zeros() {
                a ^= b << (b.leading_zeros() - a.leading_zeros());
            }
            Some(P8(a))
        }
    }

    pub const fn naive_rem(self, other: P8) -> P8 {
        match self.naive_checked_rem(other) {
            Some(x) => x,
            None => panic!("Division by 0."),
        }
    }
}

impl From<P8> for u8 {
    fn from(x: P8) -> u8 {
        x.0
    }
}

impl Add<P8> for P8 {
    type Output = P8;

    fn add(self, other: P8) -> P8 {
        P8::add(self, other)
    }
}

impl Add<P8> for &P8 {
    type Output = P8;

    fn add(self, other: P8) -> P8 {
        P8::add(*self, other)
    }
}

impl Add<&P8> for P8 {
    type Output = P8;

    fn add(self, other: &P8) -> P8 {
        P8::add(self, *other)
    }
}

impl Add<&P8> for &P8 {
    type Output = P8;

    fn add(self, other: &P8) -> P8 {
        P8::add(*self, *other)
    }
}

impl AddAssign<P8> for P8 {
    fn add_assign(&mut self, other: P8) {
        *self = self.add(other)
    }
}

impl AddAssign<&P8> for P8 {
    fn add_assign(&mut self, other: &P8) {
        *self = self.add(*other)
    }
}

impl Sum<P8> for P8 {
    fn sum<I>(iter: I) -> P8
    where
        I: Iterator<Item = P8>,
    {
        iter.fold(P8(0), |a, x| a + x)
    }
}

impl<'a> Sum<&'a P8> for P8 {
    fn sum<I>(iter: I) -> P8
    where
        I: Iterator<Item = &'a P8>,
    {
        iter.fold(P8(0), |a, x| a + *x)
    }
}

impl Sub for P8 {
    type Output = P8;

    fn sub(self, other: P8) -> P8 {
        P8::sub(self, other)
    }
}

impl Sub<P8> for &P8 {
    type Output = P8;

    fn sub(self, other: P8) -> P8 {
        P8::sub(*self, other)
    }
}

impl Sub<&P8> for P8 {
    type Output = P8;

    fn sub(self, other: &P8) -> P8 {
        P8::sub(self, *other)
    }
}

impl Sub<&P8> for &P8 {
    type Output = P8;

    fn sub(self, other: &P8) -> P8 {
        P8::sub(*self, *other)
    }
}

impl SubAssign<P8> for P8 {
    fn sub_assign(&mut self, other: P8) {
        *self = self.sub(other)
    }
}

impl SubAssign<&P8> for P8 {
    fn sub_assign(&mut self, other: &P8) {
        *self = self.sub(*other)
    }
}

impl Mul for P8 {
    type Output = P8;

    fn mul(self, other: P8) -> P8 {
        P8::mul(self, other)
    }
}

impl Mul<P8> for &P8 {
    type Output = P8;

    fn mul(self, other: P8) -> P8 {
        P8::mul(*self, other)
    }
}

impl Mul<&P8> for P8 {
    type Output = P8;

    fn mul(self, other: &P8) -> P8 {
        P8::mul(self, *other)
    }
}

impl Mul<&P8> for &P8 {
    type Output = P8;

    fn mul(self, other: &P8) -> P8 {
        P8::mul(*self, *other)
    }
}

impl MulAssign<P8> for P8 {
    fn mul_assign(&mut self, other: P8) {
        *self = self.mul(other)
    }
}

impl MulAssign<&P8> for P8 {
    fn mul_assign(&mut self, other: &P8) {
        *self = self.mul(*other)
    }
}

impl Product<P8> for P8 {
    fn product<I>(iter: I) -> P8
    where
        I: Iterator<Item = P8>,
    {
        iter.fold(P8(0), |a, x| a * x)
    }
}

impl<'a> Product<&'a P8> for P8 {
    fn product<I>(iter: I) -> P8
    where
        I: Iterator<Item = &'a P8>,
    {
        iter.fold(P8(0), |a, x| a * *x)
    }
}

impl Div for P8 {
    type Output = P8;

    fn div(self, other: P8) -> P8 {
        P8::div(self, other)
    }
}

impl Div<P8> for &P8 {
    type Output = P8;

    fn div(self, other: P8) -> P8 {
        P8::div(*self, other)
    }
}

impl Div<&P8> for P8 {
    type Output = P8;

    fn div(self, other: &P8) -> P8 {
        P8::div(self, *other)
    }
}

impl Div<&P8> for &P8 {
    type Output = P8;

    fn div(self, other: &P8) -> P8 {
        P8::div(*self, *other)
    }
}

impl DivAssign<P8> for P8 {
    fn div_assign(&mut self, other: P8) {
        *self = self.div(other)
    }
}

impl DivAssign<&P8> for P8 {
    fn div_assign(&mut self, other: &P8) {
        *self = self.div(*other)
    }
}

impl Rem for P8 {
    type Output = P8;

    fn rem(self, other: P8) -> P8 {
        P8::naive_rem(self, other)
    }
}

impl Rem<P8> for &P8 {
    type Output = P8;

    fn rem(self, other: P8) -> P8 {
        P8::naive_rem(*self, other)
    }
}

impl Rem<&P8> for P8 {
    type Output = P8;

    fn rem(self, other: &P8) -> P8 {
        P8::naive_rem(self, *other)
    }
}

impl Rem<&P8> for &P8 {
    type Output = P8;

    fn rem(self, other: &P8) -> P8 {
        P8::naive_rem(*self, *other)
    }
}

impl RemAssign<P8> for P8 {
    fn rem_assign(&mut self, other: P8) {
        *self = self.rem(other)
    }
}

impl RemAssign<&P8> for P8 {
    fn rem_assign(&mut self, other: &P8) {
        *self = self.rem(*other)
    }
}

impl Not for P8 {
    type Output = P8;

    fn not(self) -> P8 {
        P8(!self.0)
    }
}

impl Not for &P8 {
    type Output = P8;

    fn not(self) -> P8 {
        P8(!self.0)
    }
}

impl BitAnd<P8> for P8 {
    type Output = P8;

    fn bitand(self, other: P8) -> P8 {
        P8(self.0 & other.0)
    }
}

impl BitAnd<P8> for &P8 {
    type Output = P8;

    fn bitand(self, other: P8) -> P8 {
        P8(self.0 & other.0)
    }
}

impl BitAnd<&P8> for P8 {
    type Output = P8;

    fn bitand(self, other: &P8) -> P8 {
        P8(self.0 & other.0)
    }
}

impl BitAnd<&P8> for &P8 {
    type Output = P8;

    fn bitand(self, other: &P8) -> P8 {
        P8(self.0 & other.0)
    }
}

impl BitAndAssign<P8> for P8 {
    fn bitand_assign(&mut self, other: P8) {
        *self = *self & other;
    }
}

impl BitAndAssign<&P8> for P8 {
    fn bitand_assign(&mut self, other: &P8) {
        *self = *self & *other;
    }
}

impl BitAnd<P8> for u8 {
    type Output = P8;

    fn bitand(self, other: P8) -> P8 {
        P8(self & other.0)
    }
}

impl BitAnd<P8> for &u8 {
    type Output = P8;

    fn bitand(self, other: P8) -> P8 {
        P8(self & other.0)
    }
}

impl BitAnd<&P8> for u8 {
    type Output = P8;

    fn bitand(self, other: &P8) -> P8 {
        P8(self & other.0)
    }
}

impl BitAnd<&P8> for &u8 {
    type Output = P8;

    fn bitand(self, other: &P8) -> P8 {
        P8(self & other.0)
    }
}

impl BitAnd<u8> for P8 {
    type Output = P8;

    fn bitand(self, other: u8) -> P8 {
        P8(self.0 & other)
    }
}

impl BitAnd<u8> for &P8 {
    type Output = P8;

    fn bitand(self, other: u8) -> P8 {
        P8(self.0 & other)
    }
}

impl BitAnd<&u8> for P8 {
    type Output = P8;

    fn bitand(self, other: &u8) -> P8 {
        P8(self.0 & other)
    }
}

impl BitAnd<&u8> for &P8 {
    type Output = P8;

    fn bitand(self, other: &u8) -> P8 {
        P8(self.0 & other)
    }
}

impl BitAndAssign<u8> for P8 {
    fn bitand_assign(&mut self, other: u8) {
        *self = *self & other;
    }
}

impl BitAndAssign<&u8> for P8 {
    fn bitand_assign(&mut self, other: &u8) {
        *self = *self & *other;
    }
}

impl BitOr<P8> for P8 {
    type Output = P8;

    fn bitor(self, other: P8) -> P8 {
        P8(self.0 | other.0)
    }
}

impl BitOr<P8> for &P8 {
    type Output = P8;

    fn bitor(self, other: P8) -> P8 {
        P8(self.0 | other.0)
    }
}

impl BitOr<&P8> for P8 {
    type Output = P8;

    fn bitor(self, other: &P8) -> P8 {
        P8(self.0 | other.0)
    }
}

impl BitOr<&P8> for &P8 {
    type Output = P8;

    fn bitor(self, other: &P8) -> P8 {
        P8(self.0 | other.0)
    }
}

impl BitOrAssign<P8> for P8 {
    fn bitor_assign(&mut self, other: P8) {
        *self = *self | other;
    }
}

impl BitOrAssign<&P8> for P8 {
    fn bitor_assign(&mut self, other: &P8) {
        *self = *self | *other;
    }
}

impl BitOr<P8> for u8 {
    type Output = P8;

    fn bitor(self, other: P8) -> P8 {
        P8(self | other.0)
    }
}

impl BitOr<P8> for &u8 {
    type Output = P8;

    fn bitor(self, other: P8) -> P8 {
        P8(self | other.0)
    }
}

impl BitOr<&P8> for u8 {
    type Output = P8;

    fn bitor(self, other: &P8) -> P8 {
        P8(self | other.0)
    }
}

impl BitOr<&P8> for &u8 {
    type Output = P8;

    fn bitor(self, other: &P8) -> P8 {
        P8(self | other.0)
    }
}

impl BitOr<u8> for P8 {
    type Output = P8;

    fn bitor(self, other: u8) -> P8 {
        P8(self.0 | other)
    }
}

impl BitOr<u8> for &P8 {
    type Output = P8;

    fn bitor(self, other: u8) -> P8 {
        P8(self.0 | other)
    }
}

impl BitOr<&u8> for P8 {
    type Output = P8;

    fn bitor(self, other: &u8) -> P8 {
        P8(self.0 | other)
    }
}

impl BitOr<&u8> for &P8 {
    type Output = P8;

    fn bitor(self, other: &u8) -> P8 {
        P8(self.0 | other)
    }
}

impl BitOrAssign<u8> for P8 {
    fn bitor_assign(&mut self, other: u8) {
        *self = *self | other;
    }
}

impl BitOrAssign<&u8> for P8 {
    fn bitor_assign(&mut self, other: &u8) {
        *self = *self | *other;
    }
}

impl BitXor<P8> for P8 {
    type Output = P8;

    fn bitxor(self, other: P8) -> P8 {
        P8(self.0 ^ other.0)
    }
}

impl BitXor<P8> for &P8 {
    type Output = P8;

    fn bitxor(self, other: P8) -> P8 {
        P8(self.0 ^ other.0)
    }
}

impl BitXor<&P8> for P8 {
    type Output = P8;

    fn bitxor(self, other: &P8) -> P8 {
        P8(self.0 ^ other.0)
    }
}

impl BitXor<&P8> for &P8 {
    type Output = P8;

    fn bitxor(self, other: &P8) -> P8 {
        P8(self.0 ^ other.0)
    }
}

impl BitXorAssign<P8> for P8 {
    fn bitxor_assign(&mut self, other: P8) {
        *self = *self ^ other;
    }
}

impl BitXorAssign<&P8> for P8 {
    fn bitxor_assign(&mut self, other: &P8) {
        *self = *self ^ *other;
    }
}

impl BitXor<P8> for u8 {
    type Output = P8;

    fn bitxor(self, other: P8) -> P8 {
        P8(self ^ other.0)
    }
}

impl BitXor<P8> for &u8 {
    type Output = P8;

    fn bitxor(self, other: P8) -> P8 {
        P8(self ^ other.0)
    }
}

impl BitXor<&P8> for u8 {
    type Output = P8;

    fn bitxor(self, other: &P8) -> P8 {
        P8(self ^ other.0)
    }
}

impl BitXor<&P8> for &u8 {
    type Output = P8;

    fn bitxor(self, other: &P8) -> P8 {
        P8(self ^ other.0)
    }
}

impl BitXor<u8> for P8 {
    type Output = P8;

    fn bitxor(self, other: u8) -> P8 {
        P8(self.0 ^ other)
    }
}

impl BitXor<u8> for &P8 {
    type Output = P8;

    fn bitxor(self, other: u8) -> P8 {
        P8(self.0 ^ other)
    }
}

impl BitXor<&u8> for P8 {
    type Output = P8;

    fn bitxor(self, other: &u8) -> P8 {
        P8(self.0 ^ other)
    }
}

impl BitXor<&u8> for &P8 {
    type Output = P8;

    fn bitxor(self, other: &u8) -> P8 {
        P8(self.0 ^ other)
    }
}

impl BitXorAssign<u8> for P8 {
    fn bitxor_assign(&mut self, other: u8) {
        *self = *self ^ other;
    }
}

impl BitXorAssign<&u8> for P8 {
    fn bitxor_assign(&mut self, other: &u8) {
        *self = *self ^ *other;
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct P32(pub u32);

impl P32 {
    pub const fn new(v: u32) -> Self {
        Self(v)
    }

    pub const fn get(self) -> u32 {
        self.0
    }

    pub const fn add(self, other: P32) -> P32 {
        Self(self.0 ^ other.0)
    }

    pub const fn sub(self, other: P32) -> P32 {
        Self(self.0 ^ other.0)
    }

    pub const fn naive_wrapping_mul(self, other: P32) -> P32 {
        let a = self.0;
        let b = other.0;
        let mut x = 0;
        let mut i = 0;
        while i < 8 {
            let mask = (((a as i8) << (8 - 1 - i)) >> (8 - 1)) as u32;
            x ^= mask & (b << i);
            i += 1;
        }
        P32(x)
    }

    pub const fn mul(self, other: P32) -> P32 {
        self.naive_wrapping_mul(other)
    }

    pub fn pow(self, exp: u32) -> P32 {
        let mut a = self;
        let mut exp = exp;
        let mut x = P32(1);
        loop {
            if exp & 1 != 0 {
                x = x.mul(a);
            }

            exp >>= 1;
            if exp == 0 {
                return x;
            }
            a = a.mul(a);
        }
    }

    pub const fn naive_checked_div(self, other: P32) -> Option<P32> {
        if other.0 == 0 {
            None
        } else {
            let mut a = self.0;
            let b = other.0;
            let mut x = 0;
            while a.leading_zeros() <= b.leading_zeros() {
                x ^= 1 << (b.leading_zeros() - a.leading_zeros());
                a ^= b << (b.leading_zeros() - a.leading_zeros());
            }
            Some(P32(x))
        }
    }

    pub const fn div(self, other: P32) -> P32 {
        match self.naive_checked_div(other) {
            Some(x) => x,
            None => panic!("Division by 0."),
        }
    }

    pub const fn naive_checked_rem(self, other: P32) -> Option<P32> {
        if other.0 == 0 {
            None
        } else {
            let mut a = self.0;
            let b = other.0;
            while a.leading_zeros() <= b.leading_zeros() {
                a ^= b << (b.leading_zeros() - a.leading_zeros());
            }
            Some(P32(a))
        }
    }

    pub const fn naive_rem(self, other: P32) -> P32 {
        match self.naive_checked_rem(other) {
            Some(x) => x,
            None => panic!("Division by 0."),
        }
    }
}

impl From<P32> for u32 {
    fn from(x: P32) -> u32 {
        x.0
    }
}

impl Add<P32> for P32 {
    type Output = P32;

    fn add(self, other: P32) -> P32 {
        P32::add(self, other)
    }
}

impl Add<P32> for &P32 {
    type Output = P32;

    fn add(self, other: P32) -> P32 {
        P32::add(*self, other)
    }
}

impl Add<&P32> for P32 {
    type Output = P32;

    fn add(self, other: &P32) -> P32 {
        P32::add(self, *other)
    }
}

impl Add<&P32> for &P32 {
    type Output = P32;

    fn add(self, other: &P32) -> P32 {
        P32::add(*self, *other)
    }
}

impl AddAssign<P32> for P32 {
    fn add_assign(&mut self, other: P32) {
        *self = self.add(other)
    }
}

impl AddAssign<&P32> for P32 {
    fn add_assign(&mut self, other: &P32) {
        *self = self.add(*other)
    }
}

impl Sum<P32> for P32 {
    fn sum<I>(iter: I) -> P32
    where
        I: Iterator<Item = P32>,
    {
        iter.fold(P32(0), |a, x| a + x)
    }
}

impl<'a> Sum<&'a P32> for P32 {
    fn sum<I>(iter: I) -> P32
    where
        I: Iterator<Item = &'a P32>,
    {
        iter.fold(P32(0), |a, x| a + *x)
    }
}

impl Sub for P32 {
    type Output = P32;

    fn sub(self, other: P32) -> P32 {
        P32::sub(self, other)
    }
}

impl Sub<P32> for &P32 {
    type Output = P32;

    fn sub(self, other: P32) -> P32 {
        P32::sub(*self, other)
    }
}

impl Sub<&P32> for P32 {
    type Output = P32;

    fn sub(self, other: &P32) -> P32 {
        P32::sub(self, *other)
    }
}

impl Sub<&P32> for &P32 {
    type Output = P32;

    fn sub(self, other: &P32) -> P32 {
        P32::sub(*self, *other)
    }
}

impl SubAssign<P32> for P32 {
    fn sub_assign(&mut self, other: P32) {
        *self = self.sub(other)
    }
}

impl SubAssign<&P32> for P32 {
    fn sub_assign(&mut self, other: &P32) {
        *self = self.sub(*other)
    }
}

impl Mul for P32 {
    type Output = P32;

    fn mul(self, other: P32) -> P32 {
        P32::mul(self, other)
    }
}

impl Mul<P32> for &P32 {
    type Output = P32;

    fn mul(self, other: P32) -> P32 {
        P32::mul(*self, other)
    }
}

impl Mul<&P32> for P32 {
    type Output = P32;

    fn mul(self, other: &P32) -> P32 {
        P32::mul(self, *other)
    }
}

impl Mul<&P32> for &P32 {
    type Output = P32;

    fn mul(self, other: &P32) -> P32 {
        P32::mul(*self, *other)
    }
}

impl MulAssign<P32> for P32 {
    fn mul_assign(&mut self, other: P32) {
        *self = self.mul(other)
    }
}

impl MulAssign<&P32> for P32 {
    fn mul_assign(&mut self, other: &P32) {
        *self = self.mul(*other)
    }
}

impl Product<P32> for P32 {
    fn product<I>(iter: I) -> P32
    where
        I: Iterator<Item = P32>,
    {
        iter.fold(P32(0), |a, x| a * x)
    }
}

impl<'a> Product<&'a P32> for P32 {
    fn product<I>(iter: I) -> P32
    where
        I: Iterator<Item = &'a P32>,
    {
        iter.fold(P32(0), |a, x| a * *x)
    }
}

impl Div for P32 {
    type Output = P32;

    fn div(self, other: P32) -> P32 {
        P32::div(self, other)
    }
}

impl Div<P32> for &P32 {
    type Output = P32;

    fn div(self, other: P32) -> P32 {
        P32::div(*self, other)
    }
}

impl Div<&P32> for P32 {
    type Output = P32;

    fn div(self, other: &P32) -> P32 {
        P32::div(self, *other)
    }
}

impl Div<&P32> for &P32 {
    type Output = P32;

    fn div(self, other: &P32) -> P32 {
        P32::div(*self, *other)
    }
}

impl DivAssign<P32> for P32 {
    fn div_assign(&mut self, other: P32) {
        *self = self.div(other)
    }
}

impl DivAssign<&P32> for P32 {
    fn div_assign(&mut self, other: &P32) {
        *self = self.div(*other)
    }
}

impl Rem for P32 {
    type Output = P32;

    fn rem(self, other: P32) -> P32 {
        P32::naive_rem(self, other)
    }
}

impl Rem<P32> for &P32 {
    type Output = P32;

    fn rem(self, other: P32) -> P32 {
        P32::naive_rem(*self, other)
    }
}

impl Rem<&P32> for P32 {
    type Output = P32;

    fn rem(self, other: &P32) -> P32 {
        P32::naive_rem(self, *other)
    }
}

impl Rem<&P32> for &P32 {
    type Output = P32;

    fn rem(self, other: &P32) -> P32 {
        P32::naive_rem(*self, *other)
    }
}

impl RemAssign<P32> for P32 {
    fn rem_assign(&mut self, other: P32) {
        *self = self.rem(other)
    }
}

impl RemAssign<&P32> for P32 {
    fn rem_assign(&mut self, other: &P32) {
        *self = self.rem(*other)
    }
}

impl Not for P32 {
    type Output = P32;

    fn not(self) -> P32 {
        P32(!self.0)
    }
}

impl Not for &P32 {
    type Output = P32;

    fn not(self) -> P32 {
        P32(!self.0)
    }
}

impl BitAnd<P32> for P32 {
    type Output = P32;

    fn bitand(self, other: P32) -> P32 {
        P32(self.0 & other.0)
    }
}

impl BitAnd<P32> for &P32 {
    type Output = P32;

    fn bitand(self, other: P32) -> P32 {
        P32(self.0 & other.0)
    }
}

impl BitAnd<&P32> for P32 {
    type Output = P32;

    fn bitand(self, other: &P32) -> P32 {
        P32(self.0 & other.0)
    }
}

impl BitAnd<&P32> for &P32 {
    type Output = P32;

    fn bitand(self, other: &P32) -> P32 {
        P32(self.0 & other.0)
    }
}

impl BitAndAssign<P32> for P32 {
    fn bitand_assign(&mut self, other: P32) {
        *self = *self & other;
    }
}

impl BitAndAssign<&P32> for P32 {
    fn bitand_assign(&mut self, other: &P32) {
        *self = *self & *other;
    }
}

impl BitAnd<P32> for u32 {
    type Output = P32;

    fn bitand(self, other: P32) -> P32 {
        P32(self & other.0)
    }
}

impl BitAnd<P32> for &u32 {
    type Output = P32;

    fn bitand(self, other: P32) -> P32 {
        P32(self & other.0)
    }
}

impl BitAnd<&P32> for u32 {
    type Output = P32;

    fn bitand(self, other: &P32) -> P32 {
        P32(self & other.0)
    }
}

impl BitAnd<&P32> for &u32 {
    type Output = P32;

    fn bitand(self, other: &P32) -> P32 {
        P32(self & other.0)
    }
}

impl BitAnd<u32> for P32 {
    type Output = P32;

    fn bitand(self, other: u32) -> P32 {
        P32(self.0 & other)
    }
}

impl BitAnd<u32> for &P32 {
    type Output = P32;

    fn bitand(self, other: u32) -> P32 {
        P32(self.0 & other)
    }
}

impl BitAnd<&u32> for P32 {
    type Output = P32;

    fn bitand(self, other: &u32) -> P32 {
        P32(self.0 & other)
    }
}

impl BitAnd<&u32> for &P32 {
    type Output = P32;

    fn bitand(self, other: &u32) -> P32 {
        P32(self.0 & other)
    }
}

impl BitAndAssign<u32> for P32 {
    fn bitand_assign(&mut self, other: u32) {
        *self = *self & other;
    }
}

impl BitAndAssign<&u32> for P32 {
    fn bitand_assign(&mut self, other: &u32) {
        *self = *self & *other;
    }
}

impl BitOr<P32> for P32 {
    type Output = P32;

    fn bitor(self, other: P32) -> P32 {
        P32(self.0 | other.0)
    }
}

impl BitOr<P32> for &P32 {
    type Output = P32;

    fn bitor(self, other: P32) -> P32 {
        P32(self.0 | other.0)
    }
}

impl BitOr<&P32> for P32 {
    type Output = P32;

    fn bitor(self, other: &P32) -> P32 {
        P32(self.0 | other.0)
    }
}

impl BitOr<&P32> for &P32 {
    type Output = P32;

    fn bitor(self, other: &P32) -> P32 {
        P32(self.0 | other.0)
    }
}

impl BitOrAssign<P32> for P32 {
    fn bitor_assign(&mut self, other: P32) {
        *self = *self | other;
    }
}

impl BitOrAssign<&P32> for P32 {
    fn bitor_assign(&mut self, other: &P32) {
        *self = *self | *other;
    }
}

impl BitOr<P32> for u32 {
    type Output = P32;

    fn bitor(self, other: P32) -> P32 {
        P32(self | other.0)
    }
}

impl BitOr<P32> for &u32 {
    type Output = P32;

    fn bitor(self, other: P32) -> P32 {
        P32(self | other.0)
    }
}

impl BitOr<&P32> for u32 {
    type Output = P32;

    fn bitor(self, other: &P32) -> P32 {
        P32(self | other.0)
    }
}

impl BitOr<&P32> for &u32 {
    type Output = P32;

    fn bitor(self, other: &P32) -> P32 {
        P32(self | other.0)
    }
}

impl BitOr<u32> for P32 {
    type Output = P32;

    fn bitor(self, other: u32) -> P32 {
        P32(self.0 | other)
    }
}

impl BitOr<u32> for &P32 {
    type Output = P32;

    fn bitor(self, other: u32) -> P32 {
        P32(self.0 | other)
    }
}

impl BitOr<&u32> for P32 {
    type Output = P32;

    fn bitor(self, other: &u32) -> P32 {
        P32(self.0 | other)
    }
}

impl BitOr<&u32> for &P32 {
    type Output = P32;

    fn bitor(self, other: &u32) -> P32 {
        P32(self.0 | other)
    }
}

impl BitOrAssign<u32> for P32 {
    fn bitor_assign(&mut self, other: u32) {
        *self = *self | other;
    }
}

impl BitOrAssign<&u32> for P32 {
    fn bitor_assign(&mut self, other: &u32) {
        *self = *self | *other;
    }
}

impl BitXor<P32> for P32 {
    type Output = P32;

    fn bitxor(self, other: P32) -> P32 {
        P32(self.0 ^ other.0)
    }
}

impl BitXor<P32> for &P32 {
    type Output = P32;

    fn bitxor(self, other: P32) -> P32 {
        P32(self.0 ^ other.0)
    }
}

impl BitXor<&P32> for P32 {
    type Output = P32;

    fn bitxor(self, other: &P32) -> P32 {
        P32(self.0 ^ other.0)
    }
}

impl BitXor<&P32> for &P32 {
    type Output = P32;

    fn bitxor(self, other: &P32) -> P32 {
        P32(self.0 ^ other.0)
    }
}

impl BitXorAssign<P32> for P32 {
    fn bitxor_assign(&mut self, other: P32) {
        *self = *self ^ other;
    }
}

impl BitXorAssign<&P32> for P32 {
    fn bitxor_assign(&mut self, other: &P32) {
        *self = *self ^ *other;
    }
}

impl BitXor<P32> for u32 {
    type Output = P32;

    fn bitxor(self, other: P32) -> P32 {
        P32(self ^ other.0)
    }
}

impl BitXor<P32> for &u32 {
    type Output = P32;

    fn bitxor(self, other: P32) -> P32 {
        P32(self ^ other.0)
    }
}

impl BitXor<&P32> for u32 {
    type Output = P32;

    fn bitxor(self, other: &P32) -> P32 {
        P32(self ^ other.0)
    }
}

impl BitXor<&P32> for &u32 {
    type Output = P32;

    fn bitxor(self, other: &P32) -> P32 {
        P32(self ^ other.0)
    }
}

impl BitXor<u32> for P32 {
    type Output = P32;

    fn bitxor(self, other: u32) -> P32 {
        P32(self.0 ^ other)
    }
}

impl BitXor<u32> for &P32 {
    type Output = P32;

    fn bitxor(self, other: u32) -> P32 {
        P32(self.0 ^ other)
    }
}

impl BitXor<&u32> for P32 {
    type Output = P32;

    fn bitxor(self, other: &u32) -> P32 {
        P32(self.0 ^ other)
    }
}

impl BitXor<&u32> for &P32 {
    type Output = P32;

    fn bitxor(self, other: &u32) -> P32 {
        P32(self.0 ^ other)
    }
}

impl BitXorAssign<u32> for P32 {
    fn bitxor_assign(&mut self, other: u32) {
        *self = *self ^ other;
    }
}

impl BitXorAssign<&u32> for P32 {
    fn bitxor_assign(&mut self, other: &u32) {
        *self = *self ^ *other;
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct P64(pub u64);

impl P64 {
    pub const fn new(v: u64) -> Self {
        Self(v)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub const fn add(self, other: P64) -> P64 {
        Self(self.0 ^ other.0)
    }

    pub const fn sub(self, other: P64) -> P64 {
        Self(self.0 ^ other.0)
    }

    pub const fn naive_wrapping_mul(self, other: P64) -> P64 {
        let a = self.0;
        let b = other.0;
        let mut x = 0;
        let mut i = 0;
        while i < 8 {
            let mask = (((a as i8) << (8 - 1 - i)) >> (8 - 1)) as u64;
            x ^= mask & (b << i);
            i += 1;
        }
        P64(x)
    }

    pub const fn mul(self, other: P64) -> P64 {
        self.naive_wrapping_mul(other)
    }

    pub fn pow(self, exp: u64) -> P64 {
        let mut a = self;
        let mut exp = exp;
        let mut x = P64(1);
        loop {
            if exp & 1 != 0 {
                x = x.mul(a);
            }

            exp >>= 1;
            if exp == 0 {
                return x;
            }
            a = a.mul(a);
        }
    }

    pub const fn naive_checked_div(self, other: P64) -> Option<P64> {
        if other.0 == 0 {
            None
        } else {
            let mut a = self.0;
            let b = other.0;
            let mut x = 0;
            while a.leading_zeros() <= b.leading_zeros() {
                x ^= 1 << (b.leading_zeros() - a.leading_zeros());
                a ^= b << (b.leading_zeros() - a.leading_zeros());
            }
            Some(P64(x))
        }
    }

    pub const fn div(self, other: P64) -> P64 {
        match self.naive_checked_div(other) {
            Some(x) => x,
            None => panic!("Division by 0."),
        }
    }

    pub const fn naive_checked_rem(self, other: P64) -> Option<P64> {
        if other.0 == 0 {
            None
        } else {
            let mut a = self.0;
            let b = other.0;
            while a.leading_zeros() <= b.leading_zeros() {
                a ^= b << (b.leading_zeros() - a.leading_zeros());
            }
            Some(P64(a))
        }
    }

    pub const fn naive_rem(self, other: P64) -> P64 {
        match self.naive_checked_rem(other) {
            Some(x) => x,
            None => panic!("Division by 0."),
        }
    }
}

impl From<P64> for u64 {
    fn from(x: P64) -> u64 {
        x.0
    }
}

impl Add<P64> for P64 {
    type Output = P64;

    fn add(self, other: P64) -> P64 {
        P64::add(self, other)
    }
}

impl Add<P64> for &P64 {
    type Output = P64;

    fn add(self, other: P64) -> P64 {
        P64::add(*self, other)
    }
}

impl Add<&P64> for P64 {
    type Output = P64;

    fn add(self, other: &P64) -> P64 {
        P64::add(self, *other)
    }
}

impl Add<&P64> for &P64 {
    type Output = P64;

    fn add(self, other: &P64) -> P64 {
        P64::add(*self, *other)
    }
}

impl AddAssign<P64> for P64 {
    fn add_assign(&mut self, other: P64) {
        *self = self.add(other)
    }
}

impl AddAssign<&P64> for P64 {
    fn add_assign(&mut self, other: &P64) {
        *self = self.add(*other)
    }
}

impl Sum<P64> for P64 {
    fn sum<I>(iter: I) -> P64
    where
        I: Iterator<Item = P64>,
    {
        iter.fold(P64(0), |a, x| a + x)
    }
}

impl<'a> Sum<&'a P64> for P64 {
    fn sum<I>(iter: I) -> P64
    where
        I: Iterator<Item = &'a P64>,
    {
        iter.fold(P64(0), |a, x| a + *x)
    }
}

impl Sub for P64 {
    type Output = P64;

    fn sub(self, other: P64) -> P64 {
        P64::sub(self, other)
    }
}

impl Sub<P64> for &P64 {
    type Output = P64;

    fn sub(self, other: P64) -> P64 {
        P64::sub(*self, other)
    }
}

impl Sub<&P64> for P64 {
    type Output = P64;

    fn sub(self, other: &P64) -> P64 {
        P64::sub(self, *other)
    }
}

impl Sub<&P64> for &P64 {
    type Output = P64;

    fn sub(self, other: &P64) -> P64 {
        P64::sub(*self, *other)
    }
}

impl SubAssign<P64> for P64 {
    fn sub_assign(&mut self, other: P64) {
        *self = self.sub(other)
    }
}

impl SubAssign<&P64> for P64 {
    fn sub_assign(&mut self, other: &P64) {
        *self = self.sub(*other)
    }
}

impl Mul for P64 {
    type Output = P64;

    fn mul(self, other: P64) -> P64 {
        P64::mul(self, other)
    }
}

impl Mul<P64> for &P64 {
    type Output = P64;

    fn mul(self, other: P64) -> P64 {
        P64::mul(*self, other)
    }
}

impl Mul<&P64> for P64 {
    type Output = P64;

    fn mul(self, other: &P64) -> P64 {
        P64::mul(self, *other)
    }
}

impl Mul<&P64> for &P64 {
    type Output = P64;

    fn mul(self, other: &P64) -> P64 {
        P64::mul(*self, *other)
    }
}

impl MulAssign<P64> for P64 {
    fn mul_assign(&mut self, other: P64) {
        *self = self.mul(other)
    }
}

impl MulAssign<&P64> for P64 {
    fn mul_assign(&mut self, other: &P64) {
        *self = self.mul(*other)
    }
}

impl Product<P64> for P64 {
    fn product<I>(iter: I) -> P64
    where
        I: Iterator<Item = P64>,
    {
        iter.fold(P64(0), |a, x| a * x)
    }
}

impl<'a> Product<&'a P64> for P64 {
    fn product<I>(iter: I) -> P64
    where
        I: Iterator<Item = &'a P64>,
    {
        iter.fold(P64(0), |a, x| a * *x)
    }
}

impl Div for P64 {
    type Output = P64;

    fn div(self, other: P64) -> P64 {
        P64::div(self, other)
    }
}

impl Div<P64> for &P64 {
    type Output = P64;

    fn div(self, other: P64) -> P64 {
        P64::div(*self, other)
    }
}

impl Div<&P64> for P64 {
    type Output = P64;

    fn div(self, other: &P64) -> P64 {
        P64::div(self, *other)
    }
}

impl Div<&P64> for &P64 {
    type Output = P64;

    fn div(self, other: &P64) -> P64 {
        P64::div(*self, *other)
    }
}

impl DivAssign<P64> for P64 {
    fn div_assign(&mut self, other: P64) {
        *self = self.div(other)
    }
}

impl DivAssign<&P64> for P64 {
    fn div_assign(&mut self, other: &P64) {
        *self = self.div(*other)
    }
}

impl Rem for P64 {
    type Output = P64;

    fn rem(self, other: P64) -> P64 {
        P64::naive_rem(self, other)
    }
}

impl Rem<P64> for &P64 {
    type Output = P64;

    fn rem(self, other: P64) -> P64 {
        P64::naive_rem(*self, other)
    }
}

impl Rem<&P64> for P64 {
    type Output = P64;

    fn rem(self, other: &P64) -> P64 {
        P64::naive_rem(self, *other)
    }
}

impl Rem<&P64> for &P64 {
    type Output = P64;

    fn rem(self, other: &P64) -> P64 {
        P64::naive_rem(*self, *other)
    }
}

impl RemAssign<P64> for P64 {
    fn rem_assign(&mut self, other: P64) {
        *self = self.rem(other)
    }
}

impl RemAssign<&P64> for P64 {
    fn rem_assign(&mut self, other: &P64) {
        *self = self.rem(*other)
    }
}

impl Not for P64 {
    type Output = P64;

    fn not(self) -> P64 {
        P64(!self.0)
    }
}

impl Not for &P64 {
    type Output = P64;

    fn not(self) -> P64 {
        P64(!self.0)
    }
}

impl BitAnd<P64> for P64 {
    type Output = P64;

    fn bitand(self, other: P64) -> P64 {
        P64(self.0 & other.0)
    }
}

impl BitAnd<P64> for &P64 {
    type Output = P64;

    fn bitand(self, other: P64) -> P64 {
        P64(self.0 & other.0)
    }
}

impl BitAnd<&P64> for P64 {
    type Output = P64;

    fn bitand(self, other: &P64) -> P64 {
        P64(self.0 & other.0)
    }
}

impl BitAnd<&P64> for &P64 {
    type Output = P64;

    fn bitand(self, other: &P64) -> P64 {
        P64(self.0 & other.0)
    }
}

impl BitAndAssign<P64> for P64 {
    fn bitand_assign(&mut self, other: P64) {
        *self = *self & other;
    }
}

impl BitAndAssign<&P64> for P64 {
    fn bitand_assign(&mut self, other: &P64) {
        *self = *self & *other;
    }
}

impl BitAnd<P64> for u64 {
    type Output = P64;

    fn bitand(self, other: P64) -> P64 {
        P64(self & other.0)
    }
}

impl BitAnd<P64> for &u64 {
    type Output = P64;

    fn bitand(self, other: P64) -> P64 {
        P64(self & other.0)
    }
}

impl BitAnd<&P64> for u64 {
    type Output = P64;

    fn bitand(self, other: &P64) -> P64 {
        P64(self & other.0)
    }
}

impl BitAnd<&P64> for &u64 {
    type Output = P64;

    fn bitand(self, other: &P64) -> P64 {
        P64(self & other.0)
    }
}

impl BitAnd<u64> for P64 {
    type Output = P64;

    fn bitand(self, other: u64) -> P64 {
        P64(self.0 & other)
    }
}

impl BitAnd<u64> for &P64 {
    type Output = P64;

    fn bitand(self, other: u64) -> P64 {
        P64(self.0 & other)
    }
}

impl BitAnd<&u64> for P64 {
    type Output = P64;

    fn bitand(self, other: &u64) -> P64 {
        P64(self.0 & other)
    }
}

impl BitAnd<&u64> for &P64 {
    type Output = P64;

    fn bitand(self, other: &u64) -> P64 {
        P64(self.0 & other)
    }
}

impl BitAndAssign<u64> for P64 {
    fn bitand_assign(&mut self, other: u64) {
        *self = *self & other;
    }
}

impl BitAndAssign<&u64> for P64 {
    fn bitand_assign(&mut self, other: &u64) {
        *self = *self & *other;
    }
}

impl BitOr<P64> for P64 {
    type Output = P64;

    fn bitor(self, other: P64) -> P64 {
        P64(self.0 | other.0)
    }
}

impl BitOr<P64> for &P64 {
    type Output = P64;

    fn bitor(self, other: P64) -> P64 {
        P64(self.0 | other.0)
    }
}

impl BitOr<&P64> for P64 {
    type Output = P64;

    fn bitor(self, other: &P64) -> P64 {
        P64(self.0 | other.0)
    }
}

impl BitOr<&P64> for &P64 {
    type Output = P64;

    fn bitor(self, other: &P64) -> P64 {
        P64(self.0 | other.0)
    }
}

impl BitOrAssign<P64> for P64 {
    fn bitor_assign(&mut self, other: P64) {
        *self = *self | other;
    }
}

impl BitOrAssign<&P64> for P64 {
    fn bitor_assign(&mut self, other: &P64) {
        *self = *self | *other;
    }
}

impl BitOr<P64> for u64 {
    type Output = P64;

    fn bitor(self, other: P64) -> P64 {
        P64(self | other.0)
    }
}

impl BitOr<P64> for &u64 {
    type Output = P64;

    fn bitor(self, other: P64) -> P64 {
        P64(self | other.0)
    }
}

impl BitOr<&P64> for u64 {
    type Output = P64;

    fn bitor(self, other: &P64) -> P64 {
        P64(self | other.0)
    }
}

impl BitOr<&P64> for &u64 {
    type Output = P64;

    fn bitor(self, other: &P64) -> P64 {
        P64(self | other.0)
    }
}

impl BitOr<u64> for P64 {
    type Output = P64;

    fn bitor(self, other: u64) -> P64 {
        P64(self.0 | other)
    }
}

impl BitOr<u64> for &P64 {
    type Output = P64;

    fn bitor(self, other: u64) -> P64 {
        P64(self.0 | other)
    }
}

impl BitOr<&u64> for P64 {
    type Output = P64;

    fn bitor(self, other: &u64) -> P64 {
        P64(self.0 | other)
    }
}

impl BitOr<&u64> for &P64 {
    type Output = P64;

    fn bitor(self, other: &u64) -> P64 {
        P64(self.0 | other)
    }
}

impl BitOrAssign<u64> for P64 {
    fn bitor_assign(&mut self, other: u64) {
        *self = *self | other;
    }
}

impl BitOrAssign<&u64> for P64 {
    fn bitor_assign(&mut self, other: &u64) {
        *self = *self | *other;
    }
}

impl BitXor<P64> for P64 {
    type Output = P64;

    fn bitxor(self, other: P64) -> P64 {
        P64(self.0 ^ other.0)
    }
}

impl BitXor<P64> for &P64 {
    type Output = P64;

    fn bitxor(self, other: P64) -> P64 {
        P64(self.0 ^ other.0)
    }
}

impl BitXor<&P64> for P64 {
    type Output = P64;

    fn bitxor(self, other: &P64) -> P64 {
        P64(self.0 ^ other.0)
    }
}

impl BitXor<&P64> for &P64 {
    type Output = P64;

    fn bitxor(self, other: &P64) -> P64 {
        P64(self.0 ^ other.0)
    }
}

impl BitXorAssign<P64> for P64 {
    fn bitxor_assign(&mut self, other: P64) {
        *self = *self ^ other;
    }
}

impl BitXorAssign<&P64> for P64 {
    fn bitxor_assign(&mut self, other: &P64) {
        *self = *self ^ *other;
    }
}

impl BitXor<P64> for u64 {
    type Output = P64;

    fn bitxor(self, other: P64) -> P64 {
        P64(self ^ other.0)
    }
}

impl BitXor<P64> for &u64 {
    type Output = P64;

    fn bitxor(self, other: P64) -> P64 {
        P64(self ^ other.0)
    }
}

impl BitXor<&P64> for u64 {
    type Output = P64;

    fn bitxor(self, other: &P64) -> P64 {
        P64(self ^ other.0)
    }
}

impl BitXor<&P64> for &u64 {
    type Output = P64;

    fn bitxor(self, other: &P64) -> P64 {
        P64(self ^ other.0)
    }
}

impl BitXor<u64> for P64 {
    type Output = P64;

    fn bitxor(self, other: u64) -> P64 {
        P64(self.0 ^ other)
    }
}

impl BitXor<u64> for &P64 {
    type Output = P64;

    fn bitxor(self, other: u64) -> P64 {
        P64(self.0 ^ other)
    }
}

impl BitXor<&u64> for P64 {
    type Output = P64;

    fn bitxor(self, other: &u64) -> P64 {
        P64(self.0 ^ other)
    }
}

impl BitXor<&u64> for &P64 {
    type Output = P64;

    fn bitxor(self, other: &u64) -> P64 {
        P64(self.0 ^ other)
    }
}

impl BitXorAssign<u64> for P64 {
    fn bitxor_assign(&mut self, other: u64) {
        *self = *self ^ other;
    }
}

impl BitXorAssign<&u64> for P64 {
    fn bitxor_assign(&mut self, other: &u64) {
        *self = *self ^ *other;
    }
}
