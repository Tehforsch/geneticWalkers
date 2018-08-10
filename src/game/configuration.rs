extern crate rand;
use physics::point::Point;
use ::constants::{MIN_STRENGTH,MAX_STRENGTH,MIN_PHASE_SHIFT,MAX_PHASE_SHIFT,MIN_AMPLITUDE,MAX_AMPLITUDE};
use game::simulation::Simulation;

#[derive(Clone)]
pub struct Configuration {
    pub spring_parameters: Vec<SpringParameter>,
    pub body_parameters: Vec<BodyParameter>,
    pub score: i64
}

impl Configuration {
    fn new(spring_parameters: Vec<SpringParameter>, body_parameters: Vec<BodyParameter>) -> Configuration {
        Configuration {
            spring_parameters: spring_parameters,
            body_parameters: body_parameters,
            score: i64::min_value()
        }
    }

    pub fn new_random(skeleton: &Skeleton) -> Configuration {
        let spring_parameters = skeleton.springs.iter().map(|_| SpringParameter::new_random()).collect();
        let body_parameters = skeleton.body_positions.iter().map(|_| BodyParameter::new_random()).collect();
        Configuration::new(spring_parameters, body_parameters)
    }

    pub fn evaluate(&mut self, skeleton: &Skeleton) {
        let mut sim = Simulation::new(skeleton, self);
        self.score = sim.evaluate();
    }

    pub fn mutate(&self, mutation_rate: f64) -> Configuration {
        let spring_parameters = self.spring_parameters.iter().map(|param| param.mutate(mutation_rate)).collect();
        let body_parameters = self.body_parameters.iter().map(|param| param.mutate(mutation_rate)).collect();
        Configuration::new(spring_parameters, body_parameters)
    }
}

#[derive(Clone)]
pub struct Skeleton {
    pub body_positions: Vec<Point>,
    pub springs: Vec<(usize, usize)>
}

#[derive(Clone)]
pub struct BodyParameter {
    pub phase_shift: f64,
}

#[derive(Clone)]
pub struct SpringParameter {
    pub strength: f64,
    pub phase_shift: f64,
    pub amplitude: f64,
}

impl SpringParameter {
    pub fn new_random() -> SpringParameter {
        SpringParameter {
            strength: random_from_interval(MIN_STRENGTH, MAX_STRENGTH),
            phase_shift: random_from_interval(MIN_PHASE_SHIFT, MAX_PHASE_SHIFT),
            amplitude: random_from_interval(MIN_AMPLITUDE, MAX_AMPLITUDE),
        }
    }

    pub fn mutate(&self, mutation_rate: f64) -> SpringParameter {
        SpringParameter {
            strength: mutate_param(self.strength, MIN_STRENGTH, MAX_STRENGTH, mutation_rate),
            phase_shift: mutate_param(self.phase_shift, MIN_PHASE_SHIFT, MAX_PHASE_SHIFT, mutation_rate),
            amplitude: mutate_param(self.amplitude, MIN_AMPLITUDE, MAX_AMPLITUDE, mutation_rate),
        }
    }

}

impl BodyParameter {
    pub fn new_random() -> BodyParameter {
        BodyParameter {
            phase_shift: random_from_interval(MIN_PHASE_SHIFT, MAX_PHASE_SHIFT),
        }
    }

    pub fn mutate(&self, mutation_rate: f64) -> BodyParameter {
        BodyParameter {
            phase_shift: mutate_param(self.phase_shift, MIN_PHASE_SHIFT, MAX_PHASE_SHIFT, mutation_rate),
        }
    }

}

pub fn mutate_param(value: f64, min_value: f64, max_value: f64, mutation_rate: f64) -> f64 {
    let temp_value = value + (rand::random::<f64>() * 2.0 - 1.0) * mutation_rate * (max_value - min_value);
    // clamp to boundaries
    temp_value.max(min_value).min(max_value)
}

pub fn random_from_interval(min: f64, max: f64) -> f64 {
    rand::random::<f64>() * (max - min) + min
}

