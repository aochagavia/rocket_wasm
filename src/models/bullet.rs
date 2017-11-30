use super::Vector;
use geometry::{Advance, Collide};

/// Bullets are spawned when the player shoots
///
/// When an enemy is reached by a bullet, it will explode
pub struct Bullet {
    vector: Vector
}

derive_position_direction!(Bullet);

impl Bullet {
    /// Create a bullet with the given vector
    pub fn new(vector: Vector) -> Bullet {
        Bullet { vector: vector }
    }

    /// Update the bullet's position
    pub fn update(&mut self, units: f64) {
        self.advance(units);
    }
}

impl Collide for Bullet {
    fn radius(&self) -> f64 { 3.0 }
}
