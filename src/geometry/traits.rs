//! Traits used by the models

use std::f64;

use super::{Point, Size};

/// A trait for objects that occupy a position in space
pub trait Position {
    /// Returns the x coordinate of the object
    fn x(&self) -> f64;

    /// Returns a mutable reference to the x coordinate
    fn x_mut(&mut self) -> &mut f64;

    /// Returns the y coordinate of the object
    fn y(&self) -> f64;

    /// Returns a mutable reference to the y coordinate
    fn y_mut(&mut self) -> &mut f64;

    /// Returns the position of the object
    fn position(&self) -> Point {
        Point::new(self.x(), self.y())
    }
}

/// A trait for objects that have can move in a given direction
pub trait Advance: Position {
    /// Returns the direction of the object, measured in radians
    ///
    /// Note: 0.0 points to the right and a positive number means a clockwise
    /// rotation
    fn direction(&self) -> f64;

    /// Returns a mutable reference to the direction of the object
    fn direction_mut(&mut self) -> &mut f64;

    /// Changes the direction of the vector to point to the given target
    fn point_to(&mut self, target: Point) {
        let m = (self.y() - target.y) / (self.x() - target.x);

        *self.direction_mut() = if target.x > self.x() {
            m.atan()
        } else {
            m.atan() + f64::consts::PI
        };
    }

    /// Advances the object in the given amount of units, according to its direction
    fn advance(&mut self, units: f64) {
        *self.x_mut() += self.direction().cos() * units;
        *self.y_mut() += self.direction().sin() * units;
    }

    /// Similar to `Advance::advance`, but the final position will be wrapped
    /// around the given bounds
    fn advance_wrapping(&mut self, units: f64, bounds: Size) {
        self.advance(units);

        fn wrap(k: &mut f64, bound: f64) {
            if *k < 0.0 {
                *k += bound;
            } else if *k >= bound {
                *k -= bound;
            }
        }

        wrap(self.x_mut(), bounds.width);
        wrap(self.y_mut(), bounds.height);
    }
}

/// A trait that provides collision detection for objects with a position and a radius
///
/// For collision purposes, all objects are treated as circles
pub trait Collide: Position {
    /// Returns the radius of the object
    fn radius(&self) -> f64;

    /// Returns the diameter of the objects
    fn diameter(&self) -> f64 {
        self.radius() * 2.0
    }

    /// Returns true if the two objects collide and false otherwise
    fn collides_with<O: Collide>(&self, other: &O) -> bool {
        let radii = self.radius() + other.radius();
        self.position().squared_distance_to(&other.position()) < radii * radii
    }
}
