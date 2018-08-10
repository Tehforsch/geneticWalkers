pub const DT: f64 = 0.01;
pub const WALL_RESTITUTION: f64 = 0.0;
pub const BAUMGARTE_CORRECTION_STRENGTH: f64 = 1.0;
pub const GRAVITY: f64 = 200.0;
pub const WALL_FRICTION: f64 = 1.0;

pub const MIN_STRENGTH: f64 = 50.0;
pub const MAX_STRENGTH: f64 = 100.0;
pub const BASE_FREQUENCY: f64 = 3.0; 
pub const MIN_PHASE_SHIFT: f64 = 0.0;
pub const MAX_PHASE_SHIFT: f64 = 6.28;
pub const MIN_AMPLITUDE: f64 = 0.0;
pub const MAX_AMPLITUDE: f64 = 10.0;

pub const MIN_LENGTH: f64 = 1.0;

pub const EVAL_TIME: f64 = 10.0;

pub const MUTATION_RATE_DECAY: f64 = 0.99;
pub const INITIAL_MUTATION_RATE: f64 = 0.3;

pub const NUM_INDIVIDUALS: usize = 1000;
pub const NUM_GENERATIONS: usize = 200;
pub const AMOUNT_SURVIVORS: f64 = 0.10;
pub const NUM_SURVIVORS: usize = ((NUM_INDIVIDUALS as f64) * AMOUNT_SURVIVORS) as usize;
pub const NUM_MUTATIONS: usize = NUM_INDIVIDUALS - NUM_SURVIVORS;
