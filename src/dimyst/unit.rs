use crate::dimyst::prefix::Prefixes;
use crate::dimyst::base_quantities::Quantities;

pub struct Unit {
    prefix: Prefixes,
    quantities: Quantities
}
impl Mul for Unit {

}