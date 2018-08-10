extern crate rand;
use physics::point::Point;
use ::constants;

pub struct Configuration {
    pub skeleton: Skeleton,
    pub parameters: Vec<SpringParameter>
}

impl Configuration {
    pub fn new_random(skeleton: Skeleton) -> Configuration {
        let num_springs = skeleton.springs.len();
        let parameters = skeleton.springs.iter()
            .map(
                |_| SpringParameter 
                {
                    strength: Configuration::random_from_interval(constants::MIN_STRENGTH, constants::MAX_STRENGTH),
                    frequency: Configuration::random_from_interval(constants::MIN_FREQUENCY, constants::MAX_FREQUENCY),
                    amplitude: Configuration::random_from_interval(constants::MIN_AMPLITUDE, constants::MAX_AMPLITUDE),
                }
            ).collect();
        Configuration {
            skeleton: skeleton,
            parameters: parameters
        }
    }

    pub fn random_from_interval(min: f64, max: f64) -> f64 {
        rand::random::<f64>() * (max - min) + min
    }
}

pub struct Skeleton {
    pub body_positions: Vec<Point>,
    pub springs: Vec<(usize, usize)>
}

pub struct SpringParameter {
    pub strength: f64,
    pub frequency: f64,
    pub amplitude: f64,
}
