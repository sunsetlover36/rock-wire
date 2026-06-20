use serde::{Deserialize, Serialize};

use crate::PlayerId;

pub type Name = String;
pub type OwnedBy = PlayerId;

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}
impl Vector2D {
    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn distance(&self, other: &Self) -> f32 {
        self.distance_squared(other).sqrt()
    }
}
pub type Position = Vector2D;

pub type Rotation = u8;
