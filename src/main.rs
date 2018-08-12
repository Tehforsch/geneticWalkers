extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{EventLoop, Input, OpenGL, PistonWindow, WindowSettings,Button,Key};
use opengl_graphics::GlGraphics;
use game::Game;

mod physics;
mod game;
mod tools;
mod render;
mod constants;

fn main() {
    let mut game = Game::new();

    let opengl = OpenGL::V3_2;

    let settings = WindowSettings::new(
        "Amazing Grame", [1920 as u32, 1080 as u32])
        .opengl(opengl).samples(8).fullscreen(false);
    let mut window: PistonWindow = settings.build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);


    while let Some(e) = window.next() {
        match e {
            Input::Update(_) => {
                game.timestep();
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |context, gl| game.render(context, gl));
            }

            Input::Press(Button::Keyboard(key)) => {
                match key {
                    Key::R => {
                        game.reset();
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

