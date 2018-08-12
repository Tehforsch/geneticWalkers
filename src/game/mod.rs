extern crate rand;

pub mod simulation;
pub mod configuration;

use self::simulation::Simulation;
use self::configuration::{Configuration, Skeleton};
use physics::point::Point;
use opengl_graphics::GlGraphics;
use ::render;
use piston_window::Context;
use ::constants::{NUM_SURVIVORS,NUM_INDIVIDUALS,NUM_MUTATIONS,NUM_GENERATIONS,FINAL_MUTATION_RATE,INITIAL_MUTATION_RATE};
use self::rand::{thread_rng, Rng};

pub struct Game {
    simulation: Simulation,
    rerun_skeleton: Skeleton,
    rerun_configuration: Configuration,
}

impl Game {
    pub fn timestep(&mut self) {
        self.simulation.timestep();
    }

    pub fn reset(&mut self) { 
        self.simulation = Simulation::new(&self.rerun_skeleton, &self.rerun_configuration);
    }

    pub fn new() -> Game {
        let skeleton = Game::get_skeleton();
        let mut configurations: Vec<Configuration> = (0..NUM_INDIVIDUALS)
            .map(|_| Configuration::new_random(&skeleton))
            .collect();

        let mut mutation_rate = INITIAL_MUTATION_RATE;
        for i in 0..NUM_GENERATIONS {
            Game::optimize_configurations(&skeleton, &mut configurations, mutation_rate);
            println!("Generation {}, best score: {}, mutation rate: {}", i, configurations.iter().map(|conf| conf.score).max().unwrap(), mutation_rate);
            mutation_rate *= (FINAL_MUTATION_RATE / INITIAL_MUTATION_RATE).powf(1.0 / (NUM_GENERATIONS as f64));
        }

        Game {
            simulation: Simulation::new(&skeleton, &configurations[0]),
            rerun_skeleton: skeleton.clone(),
            rerun_configuration: configurations[0].clone(),
        }
    }

    fn optimize_configurations(skeleton: &Skeleton, configurations: &mut Vec<Configuration>, mutation_rate: f64) {
        for conf in configurations.iter_mut() {
            conf.evaluate(skeleton);
        }
        configurations.sort_unstable_by_key(|conf| -conf.score);
        // Remvove non-survivors
        configurations.drain(NUM_SURVIVORS..);

        let mut rng = thread_rng();
        for _ in 1..(NUM_MUTATIONS) {
            let mut new_individual = {
                let survivor = rng.choose(configurations).unwrap();
                survivor.mutate(mutation_rate)
            };
            new_individual.evaluate(skeleton);
            configurations.push(new_individual);
        }
    }

    fn get_skeleton() -> Skeleton {
        // Skeleton {
        //     body_positions: vec![
        //         Point::new(0.0, 700.0),
        //         Point::new(100.0, 700.0),
        //         Point::new(0.0, 800.0),
        //         Point::new(100.0, 800.0),
        //     ],
        //     springs: vec![
        //         (0, 1),
        //         (0, 2),
        //         (0, 3),
        //         (1, 2),
        //         (1, 3),
        //         (2, 3),
        //     ]
        // }
        Skeleton {
            body_positions: vec![
                Point::new(0.0, 700.0),
                Point::new(0.0, 800.0),
                Point::new(100.0, 800.0),
                Point::new(100.0, 700.0),
                Point::new(50.0, 750.0),
                Point::new(150.0, 750.0),
            ],
            springs: vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0),
                (0, 4),
                (1, 4),
                (2, 4),
                (3, 4),
                (2, 5),
                (3, 5),
            ]
        }
    }

    pub fn render(&self, context: Context, gl: &mut GlGraphics) {
        render::render(context, gl, &self.simulation)
    }
}
