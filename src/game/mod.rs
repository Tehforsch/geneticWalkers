use physics::Physics;
use physics::body::Body;
use physics::spring::Spring;
use tools::library::Library;
use self::configuration::Configuration;

pub mod configuration;

pub struct Game {
    pub physics: Physics
}

impl Game {
    pub fn timestep(&mut self) {
        self.physics.timestep();
    }

    pub fn new(conf: Configuration) -> Game {
        let mut bodies = Library::new(vec![]);
        let mut springs = Library::new(vec![]);
        for pos in conf.skeleton.body_positions {
            bodies.push(Body::new(pos, 1.0, 10.0));
        }
        for (spring, params) in conf.skeleton.springs.iter().zip(conf.parameters) {
            let b1 =  bodies.get_rc(spring.0).unwrap();
            let b2 =  bodies.get_rc(spring.1).unwrap();
            let dist = (b1.borrow().pos - b2.borrow().pos).norm();
            springs.push(
                Spring {
                    b1: b1,
                    b2: b2,
                    strength: params.strength,
                    frequency: params.frequency,
                    amplitude: params.amplitude,
                    rest_length: dist,
                }
            );
        }
        Game {
            physics: Physics::new(bodies, springs)
        }
    }
}
