use core::f32::consts::FRAC_PI_4;

use bevy::prelude::*;

#[derive(Component, Default, PartialEq)]
pub enum SpriteFacing {
    East,
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    #[default]
    West,
}

impl SpriteFacing {
    pub fn is_northward(&self) -> bool {
        matches!(self, Self::North | Self::NorthWest | Self::NorthEast)
    }

    pub fn is_westward(&self) -> bool {
        matches!(self, Self::West | Self::NorthWest | Self::SouthWest)
    }
}

impl From<Dir2> for SpriteFacing {
    fn from(value: Dir2) -> Self {
        let angle = value.as_vec2().y.atan2(value.as_vec2().x);
        // 0 = East, counter-clockwise
        let index = ((angle / FRAC_PI_4).round() as i32).rem_euclid(8);

        match index {
            0 => Self::East,
            1 => Self::NorthEast,
            2 => Self::North,
            3 => Self::NorthWest,
            4 => Self::West,
            5 => Self::SouthWest,
            6 => Self::South,
            7 => Self::SouthEast,
            _ => unreachable!(),
        }
    }
}
