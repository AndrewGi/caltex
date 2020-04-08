use crate::math::num::Number;
use crate::math::math::{Function};

pub trait Command<Num: Number>: Function<Num> {
	fn name(&self) -> &'static str;
}