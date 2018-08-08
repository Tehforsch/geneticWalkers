pub mod body;
pub mod point;

use self::body::Body;
use tools::library::Library;
use self::point::Point;

pub struct Physics {
    pub bodies: Library<Body>
}

impl Physics {
    pub fn timestep(&mut self) {
        for mut b in self.bodies.iter() {
            b.integrate(1.0);
        }
    }

    pub fn new() -> Physics {
        let mut bodies = Library::new(vec![]);
        bodies.push(Body::new(Point::new(0., 0.), 1.0, 1.0));
        Physics {
            bodies: bodies
        }
    }
}
