
use core::ops::*;
use core::fmt::Display;
use core::convert::TryInto;

pub trait Number: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
	Neg<Output=Self> +	Clone + Copy + Sized +
	PartialOrd + PartialEq +
	Display +
	{

}
#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct Float(f64);
#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Hash, Debug)]
pub struct Integer(i64);
impl Display for Float {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		write!(f, "{}", self.0)
	}
}
impl Display for Integer {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		write!(f, "{}", self.0)
	}
}
impl Number for Float {}
impl Number for Integer {}
pub trait Zero {
	fn zero() -> Self;
	fn is_zero(&self) -> bool where Self: PartialEq + Sized {
		self == &Self::zero()
	}
}
pub trait One {
	fn one() -> Self;
	fn is_one(&self) -> bool where Self: PartialEq + Sized {
		self == &Self::one()
	}
}
pub trait Pow: Number {
	fn pow(self, power: Self) -> Self;
}
pub trait Trig: Number {
	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;
}
impl Trig for Float {
	fn sin(self) -> Self {
		Float(self.0.sin())
	}

	fn cos(self) -> Self {
		Float(self.0.cos())
	}

	fn tan(self) -> Self {
		Float(self.0.tan())
	}
}

impl Pow for Float {
	fn pow(self, power: Self) -> Self {
		Float(self.0.powf(power.0))
	}
}
impl Pow for Integer {
	fn pow(self, power: Self) -> Self {
		Integer(self.0.pow(power.0.try_into().unwrap_or(0u32)))
	}
}


impl Zero for Float {
	fn zero() -> Self {
		Float(0f64)
	}
}
impl Zero for Integer {
	fn zero() -> Self {
		Integer(0i64)
	}
}
impl One for Float {
	fn one() -> Self {
		Float(1f64)
	}
}
impl One for Integer {
	fn one() -> Self {
		Integer(1i64)
	}
}


impl core::ops::Add for Float {
	type Output = Float;

	fn add(self, rhs: Self) -> Self::Output {
		Float(self.0 + rhs.0)
	}
}
impl core::ops::Sub for Float {
	type Output = Float;

	fn sub(self, rhs: Self) -> Self::Output {
		Float(self.0 - rhs.0)
	}
}
impl core::ops::Mul for Float {
	type Output = Float;

	fn mul(self, rhs: Self) -> Self::Output {
		Float(self.0 * rhs.0)
	}
}
impl core::ops::Div for Float {
	type Output = Float;

	fn div(self, rhs: Self) -> Self::Output {
		Float(self.0 / rhs.0)
	}
}
impl core::ops::Neg for Float {
	type Output = Float;

	fn neg(self) -> Self::Output {
		Float(-self.0)
	}
}


impl core::ops::Add for Integer {
	type Output = Integer;

	fn add(self, rhs: Self) -> Self::Output {
		Integer(self.0 + rhs.0)
	}
}
impl core::ops::Sub for Integer {
	type Output = Integer;

	fn sub(self, rhs: Self) -> Self::Output {
		Integer(self.0 - rhs.0)
	}
}
impl core::ops::Mul for Integer {
	type Output = Integer;

	fn mul(self, rhs: Self) -> Self::Output {
		Integer(self.0 * rhs.0)
	}
}
impl core::ops::Div for Integer {
	type Output = Integer;

	fn div(self, rhs: Self) -> Self::Output {
		Integer(self.0 / rhs.0)
	}
}
impl core::ops::Neg for Integer {
	type Output = Integer;

	fn neg(self) -> Self::Output {
		Integer(-self.0)
	}
}
pub trait Abs {
	fn abs(self) -> Self;
}
impl Abs for Float {
	fn abs(self) -> Self {
		Float(self.0.abs())
	}
}
impl Abs for Integer {
	fn abs(self) -> Self {
		Integer(self.0.abs())
	}
}
