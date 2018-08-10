pub const DT: f64 = 0.01;
pub const WALL_RESTITUTION: f64 = 0.0;
pub const BAUMGARTE_CORRECTION_STRENGTH: f64 = 1.0;
pub const GRAVITY: f64 = 200.0;
pub const WALL_FRICTION: f64 = 1.0;

pub const MIN_STRENGTH: f64 = 50.0;
pub const MAX_STRENGTH: f64 = 100.0;
pub const MIN_FREQUENCY: f64 = 0.2;
pub const MAX_FREQUENCY: f64 = 5.0;
pub const MIN_AMPLITUDE: f64 = 0.0;
pub const MAX_AMPLITUDE: f64 = 100.0;

pub const EVAL_TIME: f64 = 10.0;

pub const NUM_INDIVIDUALS: usize = 1000;
pub const NUM_GENERATIONS: usize = 20;
pub const AMOUNT_SURVIVORS: f64 = 0.05;
pub const NUM_SURVIVORS: usize = ((NUM_INDIVIDUALS as f64) * AMOUNT_SURVIVORS) as usize;
pub const NUM_MUTATIONS: usize = NUM_INDIVIDUALS - NUM_SURVIVORS;

pub const MUTATION_SPEED: f64 = 0.4;
