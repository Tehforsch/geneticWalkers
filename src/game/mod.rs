extern crate rand;

pub mod simulation;
pub mod configuration;

use self::simulation::Simulation;
use self::configuration::{Configuration, Skeleton};
use physics::point::Point;
use opengl_graphics::GlGraphics;
use ::render;
use piston_window::Context;
use ::constants::{NUM_SURVIVORS,NUM_INDIVIDUALS,NUM_MUTATIONS,NUM_GENERATIONS};
use self::rand::{thread_rng, Rng};

pub struct Game {
    simulation: Simulation
}

impl Game {
    pub fn timestep(&mut self) {
        self.simulation.timestep();
    }

    pub fn new() -> Game {
        let skeleton = Game::get_skeleton();
        let mut configurations: Vec<Configuration> = (1..NUM_INDIVIDUALS)
            .map(|_| Configuration::new_random(&skeleton))
            .collect();

        for i in 1..NUM_GENERATIONS {
            Game::optimize_configurations(&skeleton, &mut configurations);
            println!("Generation {}, best score: {}", i, configurations.iter().map(|conf| conf.score).max().unwrap());
        }

        Game {
            simulation: Simulation::new(&skeleton, &configurations[0])
        }
    }

    fn optimize_configurations(skeleton: &Skeleton, configurations: &mut Vec<Configuration>) {
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
                survivor.mutate()
            };
            new_individual.evaluate(skeleton);
            configurations.push(new_individual);
        }
    }

    fn get_skeleton() -> Skeleton {
        Skeleton {
            body_positions: vec![
                Point::new(100.0, 400.0),
                Point::new(500.0, 400.0),
                Point::new(100.0, 800.0),
                Point::new(500.0, 800.0),
            ],
            springs: vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 2),
                (1, 3),
                (2, 3),
            ]
        }
    }

    pub fn render(&self, context: Context, gl: &mut GlGraphics) {
        render::render(context, gl, &self.simulation)
    }
}
