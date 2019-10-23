use std::collections::HashMap;
use crate::math::num::Number;
use std::cell::RefCell;
use crate::math::math::{Variable, Value};

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
}
impl<Num: Number> Value<Num> for BankVariable<Num> {
	fn calculate(&self) -> Option<Num> {
		self.parent.borrow().get_number(&self.name)
	}

	fn is_constant_to(&self, variable_name: &str) -> bool {
		unimplemented!()
	}
}
impl<Num: Number> Variable<Num> for BankVariable<Num> {
	fn get_name(&self) -> &str {
		self.name.as_str()
	}

	fn is_defined(&self) -> bool {
		self.parent.borrow().contains(&self.name)
	}
}
