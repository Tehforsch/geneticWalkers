use physics::Physics;
use physics::body::Body;
use physics::spring::Spring;
use tools::library::Library;
use game::configuration::{Configuration, Skeleton};
use ::constants;

pub struct Simulation {
    pub physics: Physics
}

impl Simulation {
    pub fn timestep(&mut self) {
        self.physics.timestep();
    }

    pub fn evaluate(&mut self) -> i64 {
        for _ in 1..((constants::EVAL_TIME / constants::DT) as i64) {
            self.timestep();
        }
        self.score()
    }

    pub fn score(&self) -> i64 {
        self.physics.bodies.iter().map(|b| b.pos.x as i64).max().unwrap()
    }

    pub fn new(skeleton: &Skeleton, conf: &Configuration) -> Simulation {
        let mut bodies = Library::new(vec![]);
        let mut springs = Library::new(vec![]);
        for (&pos, params) in skeleton.body_positions.iter().zip(conf.body_parameters.iter()) {
            bodies.push(Body::new(pos, 1.0, 10.0, params.phase_shift));
        }
        for (spring, params) in skeleton.springs.iter().zip(conf.spring_parameters.iter()) {
            let b1 =  bodies.get_rc(spring.0).unwrap();
            let b2 =  bodies.get_rc(spring.1).unwrap();
            let dist = (b1.borrow().pos - b2.borrow().pos).norm();
            springs.push(
                Spring {
                    b1: b1,
                    b2: b2,
                    strength: params.strength,
                    phase_shift: params.phase_shift,
                    amplitude: params.amplitude,
                    rest_length: dist,
                }
            );
        }
        Simulation {
            physics: Physics::new(bodies, springs)
        }
    }
}
