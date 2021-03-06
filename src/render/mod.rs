mod draw;

use opengl_graphics::GlGraphics;
use piston_window::{self, Context};

use self::draw::circle;
use self::draw::line;
use physics::body::Body;
use game::simulation::Simulation;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub fn render(context: Context, gl: &mut GlGraphics, sim: &Simulation) {
    piston_window::clear(BLACK, gl);
    for body in sim.physics.bodies.iter() {
        render_circle(context, gl, &body);
    }
    for spring in sim.physics.springs.iter() {
        render_spring(context, gl, &spring.b1.borrow(), &spring.b2.borrow());
    }
}

fn render_circle(context: Context, gl: &mut GlGraphics, body: &Body) {
    let color = match body.has_friction {
        true => RED,
        false => WHITE
    };
    circle(body.pos, body.radius, color, context, gl);
}

fn render_spring(context: Context, gl: &mut GlGraphics, body1: &Body, body2: &Body) {
    line(body1.pos, body2.pos, WHITE, context, gl);
}
