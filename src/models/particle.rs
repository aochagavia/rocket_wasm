use super::Vector;
use geometry::Advance;

/// A model representing a particle
///
/// Particles are visible objects that have a time to live and move around
/// in a given direction until their time is up. They are spawned when the
/// player or an enemy is killed
pub struct Particle {
    pub vector: Vector,
    pub ttl: f64
}

derive_position_direction!(Particle);

impl Particle {
    /// Create a particle with the given vector and time to live in seconds
    pub fn new(vector: Vector, ttl: f64) -> Particle {
        Particle { vector: vector, ttl: ttl }
    }

    /// Update the particle
    pub fn update(&mut self, elapsed_time: f64) {
        self.ttl -= elapsed_time;
        let speed = 500.0 * self.ttl * self.ttl;
        self.advance(elapsed_time * speed);
    }
}
