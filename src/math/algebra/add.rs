use crate::math::num::Number;

pub struct Add<Num: Number> {
	operands: Vec<Num>,
}