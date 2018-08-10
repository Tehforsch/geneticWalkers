use physics::Point;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Point,
    pub vel: Point,
    pub acc: Point,
    pub mass: f64,
    pub radius: f64,
    pub moment_of_inertia: f64
}

impl Body {
    pub fn integrate(&mut self, dt : f64) {
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
        self.acc.x = 0.0;
        self.acc.y = 0.0;
    }

    pub fn apply_force(&mut self, force: Point) {
        self.acc += force / self.mass;
    }

    pub fn apply_impulse(&mut self, impulse: Point) {
        self.vel += impulse / self.mass;
    }

    pub fn new(pos: Point, mass: f64, radius: f64) -> Body {
        let moment_of_inertia = mass * radius * radius * 0.5;
        Body {
            pos: pos,
            vel: Point { x: 0.0, y: 0.0 },
            acc: Point { x: 0.0, y: 0.0 },
            mass: mass,
            radius: radius,
            moment_of_inertia: moment_of_inertia
        }
    }
}
