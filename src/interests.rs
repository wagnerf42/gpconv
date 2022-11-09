use super::Point;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InterestPoint {
    pub point: Point,
    pub interest: u8,
}

lazy_static! {
    pub static ref INTERESTS: HashMap<(&'static str, &'static str), u8> = {
        [
            (("shop", "bakery"), 0),
            (("amenity", "drinking_water"), 1),
            (("amenity", "toilets"), 2),
            (("tourism", "artwork"), 3),
        ]
        .into_iter()
        .collect()
    };
}

pub const COLORS: [&'static str; 4] = ["red", "blue", "cyan", "green"];
impl InterestPoint {
    pub fn color(&self) -> &'static str {
        COLORS[self.interest as usize]
    }
}
