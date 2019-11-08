use crate::math::num::{Number, Trig};
use crate::math::math::{Value, Function, MathError};
use std::fmt::{Display, Formatter, Error};

pub enum TrigFunctions {
	Sin,
	Cos,
	Tan
}
pub struct TrigFunction<Num: Number + Trig> {
	which_function: TrigFunctions,
	operand: Box<dyn Value<Num>>
}
impl<Num: Number + Trig> Value<Num> for TrigFunction<Num> {
	fn calculate(&self) -> Result<Num, MathError<'_>> {
		let v= self.operand.calculate()?;
		Ok(match self.which_function {
			TrigFunctions::Sin => v.sin(),
			TrigFunctions::Cos => v.cos(),
			TrigFunctions::Tan => v.tan()
		})
	}

	fn is_constant(&self) -> bool {
		self.operand.is_constant()
	}

	fn is_constant_to(&self, variable_name: &str) -> bool {
		self.operand.is_constant_to(variable_name)
	}
}
impl<Num: Number + Trig> Function<Num> for TrigFunction<Num> {
	fn get_name(&self) -> &'static str {
		match self.which_function {
			TrigFunctions::Sin => "sin",
			TrigFunctions::Cos => "cos",
			TrigFunctions::Tan => "tan"
		}
	}

	fn arg_count(&self) -> usize {
		1
	}

	fn get_arg(&self, arg_i: usize) -> &dyn Value<Num> {
		assert_eq!(arg_i, 0, "trig functions only have one arg not {}", arg_i+1);
		self.operand.as_ref()
	}
}
impl<Num: Number + Trig> Display for TrigFunction<Num> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		write!(f, "{}({})", self.get_name(), self.operand)
	}
}