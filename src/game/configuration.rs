extern crate rand;
use physics::point::Point;
use ::constants::{MUTATION_SPEED,MIN_STRENGTH,MAX_STRENGTH,MIN_FREQUENCY,MAX_FREQUENCY,MIN_AMPLITUDE,MAX_AMPLITUDE};
use game::simulation::Simulation;

#[derive(Clone)]
pub struct Configuration {
    pub parameters: Vec<SpringParameter>,
    pub score: i64
}

impl Configuration {
    fn new(parameters: Vec<SpringParameter>) -> Configuration {
        Configuration {
            parameters: parameters,
            score: i64::min_value()
        }
    }

    pub fn new_random(skeleton: &Skeleton) -> Configuration {
        let parameters = skeleton.springs.iter().map(|_| SpringParameter::new_random()).collect();
        Configuration::new(parameters)
    }

    pub fn evaluate(&mut self, skeleton: &Skeleton) {
        let mut sim = Simulation::new(skeleton, self);
        self.score = sim.evaluate();
    }

    pub fn mutate(&self) -> Configuration {
        let parameters = self.parameters.iter().map(|param| param.mutate()).collect();
        Configuration::new(parameters)
    }
}

#[derive(Clone)]
pub struct Skeleton {
    pub body_positions: Vec<Point>,
    pub springs: Vec<(usize, usize)>
}

#[derive(Clone)]
pub struct SpringParameter {
    pub strength: f64,
    pub frequency: f64,
    pub amplitude: f64,
}

impl SpringParameter {
    pub fn new_random() -> SpringParameter {
        SpringParameter {
            strength: SpringParameter::random_from_interval(MIN_STRENGTH, MAX_STRENGTH),
            frequency: SpringParameter::random_from_interval(MIN_FREQUENCY, MAX_FREQUENCY),
            amplitude: SpringParameter::random_from_interval(MIN_AMPLITUDE, MAX_AMPLITUDE),
        }
    }

    pub fn mutate(&self) -> SpringParameter {
        SpringParameter {
            strength: SpringParameter::mutate_param(self.strength, MIN_STRENGTH, MAX_STRENGTH),
            frequency: SpringParameter::mutate_param(self.frequency, MIN_FREQUENCY, MAX_FREQUENCY),
            amplitude: SpringParameter::mutate_param(self.amplitude, MIN_AMPLITUDE, MAX_AMPLITUDE),
        }
    }

    pub fn random_from_interval(min: f64, max: f64) -> f64 {
        rand::random::<f64>() * (max - min) + min
    }

    pub fn mutate_param(value: f64, min_value: f64, max_value: f64) -> f64 {
        let temp_value = value + (rand::random::<f64>() * 2.0 - 1.0) * MUTATION_SPEED * (max_value - min_value);
        // clamp to boundaries
        temp_value.max(min_value).min(max_value)
    }
}
