
use crate::math::num::Number;
use crate::dimyst::unit::Unit;
use std::fmt::{Display, Formatter, Error};
use std::ops::Mul;;

pub struct Value<Num: Number> {
    value: Num,
    unit: Unit
}

impl<Num: Number> Display for Value<Num> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {}", self.value, self.unit)
    }
}
impl<Num: Number> Mul for Value<Num> {
    type Output = Value<Num>;

    fn mul(self, rhs: Self) -> Self::Output {
        let v = self.value * other.value;
        let unit = self.unit * rhs.unit;
    }
}