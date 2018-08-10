use physics::body::Body;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Spring {
    pub strength: f64,
    pub rest_length: f64,
    pub amplitude: f64,
    pub frequency: f64,
    pub b1: Rc<RefCell<Body>>,
    pub b2: Rc<RefCell<Body>>,
}

impl Spring {
    pub fn timestep(&self, time: f64) {
        self.handle(&mut self.b1.borrow_mut(), &mut self.b2.borrow_mut(), time);
    }

    pub fn handle(&self, b1: &mut Body, b2: &mut Body, time: f64) {
        let distance = b1.pos - b2.pos;
        let length = distance.norm() + self.amplitude * (time * self.frequency).sin();
        let force = distance / length * (length - self.rest_length) * self.strength;
        b1.apply_force(-force);
        b2.apply_force(force);
    }
}
