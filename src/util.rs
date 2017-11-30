use geometry::Point;
use models::{Particle, Vector};

/// Optimized version of `Vec::retain`
///
/// We achieve better performance by renouncing to keep the original order of the `Vec`
pub fn fast_retain<T, F>(vec: &mut Vec<T>, mut f: F)
where F: FnMut(&T) -> bool {
    let mut i = 0;
    while i < vec.len() {
        if !f(&vec[i]) {
            vec.swap_remove(i);
        }

        i += 1;
    }
}

/// Generates a new explosion of the given intensity at the given position.
/// This works best with values between 5 and 25
pub fn make_explosion(particles: &mut Vec<Particle>, position: &Point, intensity: u8) {
    use itertools_num;
    for rotation in itertools_num::linspace(0.0, 2.0 * ::std::f64::consts::PI, 30) {
        for ttl in (1..intensity).map(|x| (x as f64) / 10.0) {
            particles.push(Particle::new(Vector::new(position.clone(), rotation), ttl));
        }
    }
}
