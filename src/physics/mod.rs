pub mod body;
pub mod spring;
pub mod point;

use self::body::Body;
use self::spring::Spring;
use tools::library::Library;
use self::point::Point;
use ::constants;

pub struct Physics {
    pub bodies: Library<Body>, 
    pub springs: Library<Spring>, 
    pub walls: Library<Wall>, 
    time: f64
}

impl Physics {
    pub fn timestep(&mut self) {
        self.time += constants::DT;
        for mut b in self.bodies.iter() {
            b.integrate(constants::DT);
        }
        for mut s in self.springs.iter() {
            s.timestep(self.time);
        }
        self.gravity();
        self.wall_collisions();
    }

    pub fn new(bodies: Library<Body>, springs: Library<Spring>) -> Physics {
        Physics {
            bodies, springs,
            time: 0.0,
            walls: Library::new(vec![
                Wall {
                    pos: Point::new(0.0, 800.0),
                    normal: Point::new(0.0, -1.0)
                }
            ])
        }
    }

    fn gravity(&mut self) {
        for mut body in self.bodies.iter() {
            let m = body.mass;
            body.apply_force(m * constants::GRAVITY * Point::new(0.0, 1.0));
        }
    }
    fn wall_collisions(&mut self) {
        for mut body in self.bodies.iter() {
            for wall in self.walls.iter() {
                let projection_body = body.pos * wall.normal;
                let projection_wall = wall.pos * wall.normal;
                let depth = projection_wall - projection_body + body.radius;
                if depth > 0.0 {
                    let normal_impulse = body.mass * body.vel * wall.normal;
                    body.apply_impulse(wall.normal * (normal_impulse * (-1.0 - constants::WALL_RESTITUTION) + depth * constants::BAUMGARTE_CORRECTION_STRENGTH));
                    let tangent = wall.normal.normal();
                    let tangent_impulse = body.mass * body.vel * tangent;
                    body.apply_impulse(tangent * (-constants::WALL_FRICTION * tangent_impulse));
                }
            }
        }
    }

}

pub struct Wall {
    pub pos: Point,
    pub normal: Point
}

