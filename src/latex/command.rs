use crate::math::num::Number;
use crate::math::math::{Value, Function};

pub trait Command<Num: Number>: Function<Num> {
	fn name(&self) -> &'static str;
}