use core::cell::UnsafeCell;

use crate::direction::Direction;
use crate::game::Game;
use crate::render::{self, FRAMEBUFFER_SIZE};

struct GlobalGame(UnsafeCell<Game>);

// The Arduino loop is the only execution context that accesses the game.
unsafe impl Sync for GlobalGame {}

static GAME: GlobalGame = GlobalGame(UnsafeCell::new(Game::new()));

fn with_game_mut(action: impl FnOnce(&mut Game)) {
    unsafe {
        action(&mut *GAME.0.get());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_init() {
    with_game_mut(Game::restart);
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_up() {
    with_game_mut(|game| game.act(Some(Direction::North)));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_down() {
    with_game_mut(|game| game.act(Some(Direction::South)));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_left() {
    with_game_mut(|game| game.act(Some(Direction::West)));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_right() {
    with_game_mut(|game| game.act(Some(Direction::East)));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_a() {
    with_game_mut(|game| game.act(None));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_b() {
    with_game_mut(Game::restart);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn infestation_render(framebuffer: *mut u8, length: u16) {
    if framebuffer.is_null() || usize::from(length) < FRAMEBUFFER_SIZE {
        return;
    }

    let framebuffer = unsafe { core::slice::from_raw_parts_mut(framebuffer, FRAMEBUFFER_SIZE) };
    unsafe {
        render::render(&*GAME.0.get(), framebuffer);
    }
}
