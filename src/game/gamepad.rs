use crate::wasm4::*;

fn get_gamepad() -> u8 {
    unsafe { *GAMEPAD1 }
}

pub mod button {
    use super::*;

    pub fn right() -> bool {
        get_gamepad() & BUTTON_RIGHT != 0
    }

    pub fn left() -> bool {
        get_gamepad() & BUTTON_LEFT != 0
    }

    pub fn up() -> bool {
        get_gamepad() & BUTTON_UP != 0
    }

    pub fn down() -> bool {
        get_gamepad() & BUTTON_DOWN != 0
    }

    pub fn x() -> bool {
        get_gamepad() & BUTTON_1 != 0
    }

    pub fn z() -> bool {
        get_gamepad() & BUTTON_2 != 0
    }
}

pub struct Gamepad {
    pub x: f32,
    pub y: f32,
    target_x: f32,
    target_y: f32,
}

const EASING: f32 = 0.3;

impl Gamepad {
    pub fn new() -> Self {
        Gamepad {
            target_x: 0.0,
            target_y: 0.0,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.update_targets();

        self.x += (self.target_x - self.x) * EASING;
        self.y += (self.target_y - self.y) * EASING;
    }

    fn update_targets(&mut self) {
        self.target_x = if button::right() {
            1.0
        } else if button::left() {
            -1.0
        } else {
            0.0
        };

        self.target_y = if button::down() {
            1.0
        } else if button::up() {
            -1.0
        } else {
            0.0
        };
    }
}
