use super::base_quantities::*;

pub const LENGTH: Quantities = Quantities {length: 1, ..Default::default()};
pub const MASS: Quantities = Quantities {mass: 1, ..Default::default()};
pub const TIME: Quantities = Quantities {time: 1, ..Default::default()};
pub const CURRENT: Quantities = Quantities {current: 1, ..Default::default()};
pub const TEMPERATURE: Quantities = Quantities {temperature: 1, ..Default::default()};
pub const AMOUNT: Quantities = Quantities {amount: 1, ..Default::default()};
pub const LUMINOUS: Quantities = Quantities {luminous: 1, ..Default::default()};

pub const AREA: Quantities = LENGTH.to_the(2);
pub const VOLUME: Quantities = LENGTH.to_the(3);

pub const SPEED: Quantities = LENGTH / TIME;
pub const ACCELERATION: Quantities = SPEED / TIME;

pub const DENSITY: Quantities = MASS / VOLUME;
pub const SURFACE_DENSITY: Quantities = MASS / AREA;

pub const CURRENT_DENSITY: Quantities = CURRENT / AREA;

pub const MAGNETIC_FIELD_STRENGTH: Quantities = CURRENT / LENGTH;

pub const FREQUENCY: Quantities = TIME.invert();

pub const FORCE: Quantities = ACCELERATION * MASS;
pub const PRESSURE: Quantities = FORCE / AREA;
pub const ENERGY: Quantities = FORCE * LENGTH;

