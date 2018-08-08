use physics::Physics;
use ::tools::library::SubLibrary;

pub struct Game {
    pub physics: Physics
}

impl Game {
    pub fn timestep(&mut self) {
        self.physics.timestep();
    }

    pub fn new() -> Game {
        let mut physics = Physics::new();
        Game {
            physics: physics
        }
    }
}
