use frac::Frac;
#[derive(Copy, Clone)]
pub enum BasicNumber {
	Float(f64),
	Int(i64),
}
impl Into<f64> for BasicNumber {
	fn into(self) -> f64 {
		match *self {
			BasicNumber::Float(f) => f,
			BasicNumber::Int(i) => i.into()
		}
	}
}
impl Default for BasicNumber {
	fn default() -> Self {
		BasicNumber::Int(0)
	}
}

impl std::ops::Add for BasicNumber {
	type Output = BasicNumber;

	fn add(self, rhs: Self) -> Self::Output {
		match (*self, rhs) {
			(BasicNumber::Int(a), BasicNumber::Int(b)) => BasicNumber::Int(a+b),
			_ => BasicNumber::Float(f64::from(self) + f64::from(rhs))
		}
	}
}

impl std::ops::Sub for BasicNumber {
	type Output = BasicNumber;

	fn sub(self, rhs: Self) -> Self::Output {
		match (*self, rhs) {
			(BasicNumber::Int(a), BasicNumber::Int(b)) => BasicNumber::Int(a - b),
			_ => BasicNumber::Float(f64::from(self) - f64::from(rhs))
		}
	}
}

impl std::ops::Mul for BasicNumber {
	type Output = BasicNumber;

	fn mul(self, rhs: Self) -> Self::Output {
		match (*self, rhs) {
			(BasicNumber::Int(a), BasicNumber::Int(b)) => BasicNumber::Int(a*b),
			_ => BasicNumber::Float(f64::from(self) * f64::from(rhs))
		}
	}
}

impl std::ops::Div for BasicNumber {
	type Output = BasicNumber;

	fn div(self, rhs: Self) -> Self::Output {
		match (*self, rhs) {
			(BasicNumber::Int(a), BasicNumber::Int(b)) => BasicNumber::Int(a/b),
			_ => BasicNumber::Float(f64::from(self) / f64::from(rhs))
		}
	}
}