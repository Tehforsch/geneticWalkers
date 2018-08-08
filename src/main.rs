extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{Button,EventLoop, Input, OpenGL, PistonWindow, WindowSettings,Motion, Key};
use opengl_graphics::GlGraphics;
use game::Game;
use physics::point::Point;

mod physics;
mod game;
mod tools;
mod render;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut settings = WindowSettings::new(
        "Amazing Grame", [1920 as u32, 1080 as u32])
        .opengl(opengl).samples(8).fullscreen(false);
    let mut window: PistonWindow = settings.build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);
    let mut game = game::Game::new();

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
