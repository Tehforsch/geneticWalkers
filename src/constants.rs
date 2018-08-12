pub const DT: f64 = 0.01;
pub const WALL_RESTITUTION: f64 = 0.0;
pub const BAUMGARTE_CORRECTION_STRENGTH: f64 = 1.0;
pub const GRAVITY: f64 = 200.0;
pub const WALL_FRICTION: f64 = 1.0;

pub const MIN_STRENGTH: f64 = 50.0;
pub const MAX_STRENGTH: f64 = 300.0;
pub const BASE_FREQUENCY: f64 = 3.0; 
pub const MIN_PHASE_SHIFT: f64 = 0.0;
pub const MAX_PHASE_SHIFT: f64 = 6.28;
pub const MIN_AMPLITUDE: f64 = 0.0;
pub const MAX_AMPLITUDE: f64 = 10.0;

pub const MIN_LENGTH: f64 = 1.0;

pub const EVAL_TIME: f64 = 30.0;

pub const NUM_INDIVIDUALS: usize = 1000;
pub const NUM_GENERATIONS: usize = 20;
pub const AMOUNT_SURVIVORS: f64 = 0.02;
pub const NUM_SURVIVORS: usize = ((NUM_INDIVIDUALS as f64) * AMOUNT_SURVIVORS) as usize;
pub const NUM_MUTATIONS: usize = NUM_INDIVIDUALS - NUM_SURVIVORS;

pub const FINAL_MUTATION_RATE: f64 = 0.02;
pub const INITIAL_MUTATION_RATE: f64 = 0.3;
// pub MUTATION_RATE_DECAY: f64 = (FINAL_MUTATION_RATE / INITIAL_MUTATION_RATE).powf(1.0 / (NUM_GENERATIONS as f64));
