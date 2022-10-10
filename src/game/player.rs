use nalgebra::Vector2;

use crate::{game::Vector, wasm4::{trace, rect}};

use super::gamepad::{button, Gamepad};

const PLAYER_SPEED: f32 = 3.0;

pub struct Player { 
    position: Vector2<f32>,
}

impl Player {
    pub fn new() -> Self {
        Player { position: Vector2::new(80.0, 80.0) }
    }

    pub fn update(&mut self, gamepad: &Gamepad) {
        self.mov(gamepad);
        self.draw();
    }

    fn mov(&mut self, gamepad: &Gamepad) {
        let x = gamepad.x;

        self.position += Vector2::new(x, 0.0) * PLAYER_SPEED;
    }

    fn draw(&self) {
        let x = self.position.x;
        let y = self.position.y;

        rect(x as i32 - 2, y as i32 - 2, 4, 4);
    }
}
