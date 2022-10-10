use crate::wasm4::*;
use nalgebra::Vector2;

use player::Player;

use self::gamepad::Gamepad;

pub type Vector = Vector2<i32>;

mod map;
mod player;
pub mod gamepad;

pub struct Game {
    frame: i32,
    player: Player,
    gamepad: Gamepad
}

impl Game {
    pub fn new() -> Self {
        Game { 
            frame: 0, 
            player: Player::new(),
            gamepad: Gamepad::new(),
        }
    }

    pub fn update(&mut self) {
        self.frame += 1;
        self.gamepad.update();
        self.player.update(&self.gamepad);
    }
}
