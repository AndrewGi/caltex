use super::quantities::*;
use super::base_quantities;
use crate::dimyst::unit_symbol::{UnitSymbols, UnitSymbol};
use crate::dimyst::prefix::Prefixes;

const METER: UnitSymbol = UnitSymbol::new(LENGTH, "meter", "m", Prefixes::One);
const SECOND: UnitSymbol = UnitSymbol::new(TIME, "second", "s", Prefixes::One);
const KILOGRAM: UnitSymbol = UnitSymbol::new(MASS, "gram", "g", Prefixes::Kilo);
const KELVIN: UnitSymbol = UnitSymbol::new(TEMPERATURE, "kelvin", "K", Prefixes::One);
const CANDELA: UnitSymbol = UnitSymbol::new(LUMINOUS, "candela", "cd", Prefixes::One);
const MOLE: UnitSymbol = UnitSymbol::new(AMOUNT, "mole", "mol", Prefixes::One);
const AMP: UnitSymbol = UnitSymbol::new(CURRENT, "amp", "A", Prefixes::One);

pub fn si_units() -> UnitSymbols {
    let mut si = UnitSymbols::new();
}