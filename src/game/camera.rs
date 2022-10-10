use super::Vector;

struct Camera {
    position: Vector,
    zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector::new(0, 0),
            zoom: 0.0,
        }
    }
}
