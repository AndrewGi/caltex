use std::fmt::{Display, Formatter, Error};
use std::path::Component::Prefix;
use crate::dimyst::prefix::Prefixes::One;

#[derive(Copy, Clone)]
pub enum Prefixes {
    Yotta = 24,
    Zetta = 21,
    Exa = 18,
    Peta = 15,
    Tera = 12,
    Giga = 9,
    Mega = 6,
    Kilo = 3,
    Hecto = 2,
    Deca = 1,
    One = 0,
    Deci = -1,
    Centi = -2,
    Milli = -3,
    Micro = -6,
    Nano = -9,
    Pico = -12,
    Femto = -15,
    Atto = -18,
    Zepto = -21,
    Yocto = -24
}
impl Prefixes {
    pub fn as_str(self) -> &'static str {
        match self {
            Prefixes::Yotta => "Y",
            Prefixes::Zetta => "Z",
            Prefixes::Exa => "E",
            Prefixes::Peta => "P",
            Prefixes::Tera => "T",
            Prefixes::Giga => "G",
            Prefixes::Mega => "M",
            Prefixes::Kilo => "k",
            Prefixes::Hecto => "h",
            Prefixes::Deca => "da",
            Prefixes::Deci => "d",
            Prefixes::Centi => "c",
            Prefixes::Milli => "m",
            Prefixes::Micro => "μ",
            Prefixes::Nano => "n",
            Prefixes::Pico => "p",
            Prefixes::Femto => "f",
            Prefixes::Atto => "a",
            Prefixes::Zepto => "z",
            Prefixes::Yocto => "y",
            Prefixes::One => "",
        }
    }
    pub fn nearest(power: i8) -> (Prefixes, i8) {
        if power > 24 {
            (Prefixes::Yotta, power-24)
        } else if power < -24 {
            (Prefixes::Yocto, power+24)
        } else {
            let prefix_level = power / 3;
            let prefix_offset = power % 3;
            if prefix_level == 0 {
                (match prefix_offset {
                    0 => Prefixes::One,
                    1 => Prefixes::Deca,
                    2 => Prefixes::Hecto,
                    -1 => Prefixes::Deci,
                    -2 => Prefixes::Centi,
                    _ => unimplemented!()
                }, 0)
            } else {
                (Prefixes(prefix_level*3), prefix_offset)
            }
        }
    }
}
impl Display for Prefixes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.as_str())
    }
}
