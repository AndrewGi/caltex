use std::ops::{Mul, Div};
use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Quantities {
    length: i8, // Meters, feet, etc
    mass: i8, // Kilograms, pounds etc
    time: i8, // Seconds, minutes, etc
    current: i8, // Amps, Columns per second, etc
    temperature: i8, //Kelvin, Celsius, etc
    amount: i8, // Moles, etc
    luminous: i8 // Candela, etc,
}
impl Default for Quantities {
    fn default() -> Self {
        Quantities {..Default::default()}
    }
}
impl Quantities {
    pub fn new() -> Quantities {
        Quantities::default()
    }
    pub fn invert(self) -> Quantities {
        Quantities {
            length: -self.length,
            mass: -self.mass,
            time: -self.time,
            current: -self.current,
            temperature: -self.temperature,
            amount: -self.amount,
            luminous: -self.amount
        }
    }
    pub fn is_scalar(self) -> bool {
        self.length == 0 && self.mass == 0 && self.time == 0 && self.current == 0 &&
            self.temperature == 0 && self.amount == 0 && self.luminous == 0
    }
    pub fn to_the(self, power: i8) -> Quantities {
        Quantities {
            length: self.length * power,
            mass: self.mass * power,
            time: self.time * power,
            current: self.current * power,
            temperature: self.temperature * power,
            amount: self.amount * power,
            luminous: self.luminous * power
        }
    }
    pub fn power_size(self) -> usize {
        let abs = |i: i8| ((i as i32).abs() as usize);
        abs(self.length) + abs(self.mass) + abs(self.time) + abs(self.current) +
            abs(self.temperature) + abs(self.amount) + abs(self.luminous)
    }
}

impl Display for Quantities {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.length != 0 {
            write!(f, "L^{}", self.length)?;
        }
        if self.mass != 0 {
            write!(f, "M^{}", self.mass)?;
        }
        if self.time != 0 {
            write!(f, "T^{}", self.time)?;
        }
        if self.current != 0 {
            write!(f, "I^{}", self.current)?;
        }
        if self.temperature != 0 {
            write!(f, "K^{}", self.temperature)?;
        }
        if self.amount != 0 {
            write!(f, "N^{}", self.amount)?;
        }
        if self.luminous != 0 {
            write!(f, "J^{}", self.luminous)?;
        }
        Ok(())
    }
}

impl Mul for Quantities {
    type Output = Quantities;

    fn mul(self, rhs: Self) -> Self::Output {
        Quantities {
            length: self.length + rhs.length,
            mass: self.mass + rhs.mass,
            time: self.time + rhs.time,
            current: self.current + rhs.current,
            temperature: self.temperature + rhs.temperature,
            amount: self.amount + rhs.amount,
            luminous: self.luminous + rhs.luminous
        }
    }
}
impl Div for Quantities {
    type Output = Quantities;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.invert()
    }
}