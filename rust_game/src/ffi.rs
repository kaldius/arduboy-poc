use core::cell::UnsafeCell;

use crate::app::App;
use crate::direction::Direction;
use crate::render::{self, FRAMEBUFFER_SIZE};

struct GlobalApp(UnsafeCell<App>);

// The Arduino loop is the only execution context that accesses the game.
unsafe impl Sync for GlobalApp {}

static APP: GlobalApp = GlobalApp(UnsafeCell::new(App::new()));

fn with_app_mut(action: impl FnOnce(&mut App)) {
    unsafe {
        action(&mut *APP.0.get());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_init() {
    with_app_mut(App::restart);
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_up() {
    with_app_mut(|app| app.press_direction(Direction::North));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_down() {
    with_app_mut(|app| app.press_direction(Direction::South));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_left() {
    with_app_mut(|app| app.press_direction(Direction::West));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_right() {
    with_app_mut(|app| app.press_direction(Direction::East));
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_a() {
    with_app_mut(App::press_a);
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_b() {
    with_app_mut(App::press_b);
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_ab() {
    with_app_mut(App::toggle_camera_mode);
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_up_down() {
    with_app_mut(App::exit_level);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn infestation_render(framebuffer: *mut u8, length: u16) {
    if framebuffer.is_null() || usize::from(length) < FRAMEBUFFER_SIZE {
        return;
    }

    let framebuffer = unsafe { core::slice::from_raw_parts_mut(framebuffer, FRAMEBUFFER_SIZE) };
    unsafe {
        render::render(&*APP.0.get(), framebuffer);
    }
}
