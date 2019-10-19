use crate::latex::Command;
use crate::math::{Number, Value};
use crate::math::trig::*;

pub struct Sin {
	argument: Box<dyn Value>
}
impl Value for Sin {
	fn get_number(&self) -> Number {
		sin(self.argument.get_number())
	}
}
impl Command for Sin{
	fn name(&self) -> &'static str {
		"sin"
	}
}

pub struct Cos {
	argument: Box<dyn Value>
}
impl Value for Cos {
	fn get_number(&self) -> Number {
		cos(self.argument.get_number())
	}
}
impl Command for Cos {
	fn name(&self) -> &'static str {
		"cos"
	}
}

pub struct Tan {
	argument: Box<dyn Value>
}
impl Value for Tan {
	fn get_number(&self) -> Number {
		tan(self.argument.get_number())
	}
}

impl Command for Sin{
	fn name(&self) -> &'static str {
		"tan"
	}
}