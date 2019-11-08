use std::collections::HashMap;
use crate::math::num::Number;
use std::cell::RefCell;
use crate::math::math::{Variable, Value, MathError};
use std::marker::PhantomData;
use std::fmt::{Display, Formatter, Error};

pub struct VariableBank<Num: Number> {
	variables: HashMap<String, Num>,
}
impl<Num: Number> VariableBank<Num> {
	pub fn get_number(&self, var_name: &str) -> Option<Num> {
		self.variables.get(var_name)
	}
	pub fn contains(&self, var_name: &str) -> bool {
		self.variables.contains_key(var_name)
	}
}
pub struct BankVariable<'a, Num: Number> {
	parent: RefCell<VariableBank<Num>>,
	name: String,
	phantom: PhantomData<&'a str>,
}
impl<'a, Num: Number> Value<Num> for BankVariable<'a, Num> {
	fn calculate(&self) -> Result<Num, MathError> {
		self.parent.borrow().get_number(&self.name).ok_or(MathError::UndefinedVariable(self.get_name()))
	}

	fn is_constant(&self) -> bool {
		false
	}

	fn is_constant_to(&self, variable_name: &str) -> bool {
		!self.name.as_str() == variable_name
	}
}
impl<'a, Num: Number> Variable<Num> for BankVariable<'a, Num> {
	fn get_name(&self) -> &str {
		self.name.as_str()
	}

	fn is_defined(&self) -> bool {
		self.parent.borrow().contains(&self.name)
	}
}
impl<'a, Num: Number> Display for BankVariable<'a, Num> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		write!(f, "{}", self.get_name())
	}
}