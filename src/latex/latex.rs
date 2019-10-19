
pub trait Command: Value {
	fn name(&self) -> &'static str;
}