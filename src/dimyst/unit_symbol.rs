use std::collections::HashMap;
use crate::dimyst::base_quantities::Quantities;
use std::collections::hash_map::Entry;
use crate::dimyst::prefix::Prefixes;

pub struct UnitSymbol {
    quantities: Quantities,
    name: &'static str,
    symbol: &'static str,
    prefix: Prefixes,
}
impl UnitSymbol {
    pub fn new(quantities: Quantities, name: &'static str, symbol: &'static str, prefix: Prefixes) -> UnitSymbol {

    }
}
pub struct UnitSymbols {
    map: HashMap<Quantities, UnitSymbol>
}
impl UnitSymbols {
    pub fn new() -> UnitSymbols {
        UnitSymbols {
            map: Default::default()
        }
    }
    pub fn has_symbol(&self, quantities: Quantities) -> bool {
        self.map.contains_key(&quantities)
    }
    pub fn get_symbol(&self, quantities: Quantities) -> Option<&UnitSymbol> {
        self.map.get(&quantities)
    }
    pub fn add_symbol(&mut self, symbol: UnitSymbol) {
        match self.map.entry(quantities) {
            Entry::Occupied(o) => {
                panic!("already has quantities : {} with symbol {}", quantities, o.get());
            },
            Entry::Vacant(v) => v.insert(symbol),
        };
    }
}