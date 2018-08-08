struct Spring {
   b1, b2: &RefCell<Body>,
   strength: f64,
   rest_length: f64
}

impl Spring {
    pub handle(&self, b1: &mut Body, b2: &mut Body) {
        let distance = b1.pos - b2.pos;
        let length = distance.norm();
        force = distance / length * (length - self.rest_length) * strength;
        b1.apply_force(-force);
        b2.apply_force(-force);
    }
}
