extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{Button,EventLoop, Input, OpenGL, PistonWindow, WindowSettings,Motion, Key};
use opengl_graphics::GlGraphics;
use game::Game;
use physics::body::Body;
use physics::Physics;
use physics::point::Point;
use physics::spring::Spring;
use tools::library::Library;
use game::configuration::Configuration;
use game::configuration::Skeleton;

mod physics;
mod game;
mod tools;
mod render;
mod constants;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut settings = WindowSettings::new(
        "Amazing Grame", [1920 as u32, 1080 as u32])
        .opengl(opengl).samples(8).fullscreen(false);
    let mut window: PistonWindow = settings.build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);

    let skeleton = get_skeleton();
    let mut game = Game::new(Configuration::new_random(skeleton));

    while let Some(e) = window.next() {
        match e {
            Input::Update(_) => {
                game.timestep();
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |context, gl| render::render(context, gl, &game));
            }

            _ => {}
        }
    }
}

fn get_skeleton() -> Skeleton {
    Skeleton {
        body_positions: vec![
            Point::new(100.0, 100.0),
            Point::new(500.0, 100.0),
            Point::new(100.0, 500.0),
            Point::new(500.0, 500.0),
        ],
        springs: vec![
            (0, 1),
            (0, 2)
        ]
    }
}
