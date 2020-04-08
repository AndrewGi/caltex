use crate::math::math::{ GCD};
use std::ops::{Add, Mul, Div, Sub, Neg};
use crate::math::num::{Number, One, Zero};
use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Frac<Num: Number> {
	numerator: Num,
	denominator: Num,
}
impl<Num: Number + One> From<Num> for Frac<Num> {
    fn from(n: Num) -> Self {
        Frac::new(n, Num::one())
    }
}
impl<Num: Number> Frac<Num> {
	pub fn new(numerator: Num, denominator: Num) -> Frac<Num> {
		Frac {
			numerator,
			denominator
		}
	}
	pub fn invert(&self) -> Frac<Num> {
		Frac::new(self.denominator, self.numerator)
	}
	pub fn simplify(&self) -> Frac<Num> where Num: GCD {
		let gcd = self.numerator.gcd(self.denominator);
		Frac::new(self.numerator/gcd, self.denominator/gcd)
	}
}
impl<Num: Number + GCD> Number for Frac<Num> {

}
impl<Num: Number> Display for Frac<Num> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		write!(f, "{}/{}", self.numerator, self.denominator)
	}
}
impl<Num: Number + Zero + One> Zero for Frac<Num> {
	fn zero() -> Self {
		Frac::new(Num::zero(), Num::one())
	}
	fn is_zero(&self) -> bool {
		self.numerator.is_zero()
	}
}
impl<Num: Number + One> One for Frac<Num> {
	fn one() -> Self {
		Frac::new(Num::one(), Num::one())
	}
}
impl<Num: Number + One> Default for Frac<Num> {
	fn default() -> Self {
        Frac::one()
	}
}
impl<Num: Number + GCD> Add for Frac<Num> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Frac::new(self.numerator * rhs.denominator + rhs.numerator * self.denominator, self.denominator*rhs.denominator).simplify()
    }
}
impl<Num: Number + GCD> Sub for Frac<Num> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Frac::new(self.numerator * rhs.denominator - rhs.numerator * self.denominator, self.denominator*rhs.denominator).simplify()
	}
}
impl<Num: Number + GCD> Mul for Frac<Num> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Frac::new(self.numerator * rhs.numerator, self.denominator * rhs.denominator).simplify()
	}
}
impl<Num: Number + GCD> Div for Frac<Num> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		self * rhs.invert()
	}
}
impl<Num: Number> Neg for Frac<Num> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Frac::new(self.numerator.neg(), self.denominator)
	}
}