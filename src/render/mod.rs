mod draw;

use opengl_graphics::GlGraphics;
use piston_window::{self, Context, Transformed};

use self::draw::circle;
use self::draw::line;
use physics::body::Body;
use game::Game;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn render(context: Context, gl: &mut GlGraphics, game: &Game) {
    piston_window::clear(BLACK, gl);
    for body in game.physics.bodies.iter() {
        render_circle(context, gl, &body);
    }
    // for spring in game.springs.iter() {
    //     render_spring(context, gl, game.sim.get_body(spring.body1), game.sim.get_body(spring.body2));
    // }
}

fn render_circle(context: Context, gl: &mut GlGraphics, body: &Body) {
    circle(body.pos, body.radius, WHITE, context, gl);
}

// fn render_spring(context: Context, gl: &mut GlGraphics, body1: &Body, body2: &Body) {
//     line(body1.pos, body2.pos, WHITE, context, gl);
// }
