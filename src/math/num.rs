use frac::Frac;
#[derive(Copy, Clone)]
pub enum Number {
	Float(f64),
	Int(i64),
}
impl Into<f64> for Number {
	fn into(self) -> f64 {
		match *self {
			Number::Float(f) => f,
			Number::Int(i) => i.into()
		}
	}
}
impl Default for Number {
	fn default() -> Self {
		Number::Int(0)
	}
}

impl std::ops::Add for Number {
	type Output = Number;

	fn add(self, rhs: Self) -> Self::Output {
		match (*self, rhs) {
			(Number::Int(a), Number::Int(b)) => Number::Int(a+b),
			_ => Number::Float(f64::from(self) + f64::from(rhs))
		}
	}
}

impl std::ops::Sub for Number {
	type Output = Number;

	fn sub(self, rhs: Self) -> Self::Output {
		match (*self, rhs) {
			(Number::Int(a), Number::Int(b)) => Number::Int(a - b),
			_ => Number::Float(f64::from(self) - f64::from(rhs))
		}
	}
}

impl std::ops::Mul for Number {
	type Output = Number;

	fn mul(self, rhs: Self) -> Self::Output {
		match (*self, rhs) {
			(Number::Int(a), Number::Int(b)) => Number::Int(a*b),
			_ => Number::Float(f64::from(self) * f64::from(rhs))
		}
	}
}

impl std::ops::Div for Number {
	type Output = Number;

	fn div(self, rhs: Self) -> Self::Output {
		match (*self, rhs) {
			(Number::Int(a), Number::Int(b)) => Number::Int(a/b),
			_ => Number::Float(f64::from(self) / f64::from(rhs))
		}
	}
}