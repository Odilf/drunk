#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

use lazy_static::lazy_static;
use std::sync::Mutex;

mod game;
use game::Game;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [0x2E4052, 0xBDD9BF, 0xFFC857, 0x012284];
    }
}

#[no_mangle]
fn update() {
    GAME.lock().expect("game_state").update();
}
