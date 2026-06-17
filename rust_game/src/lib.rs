#![no_std]

use core::panic::PanicInfo;

const WIDTH: u8 = 128;
const HEIGHT: u8 = 64;
const HEADER_HEIGHT: u8 = 9;
const PLAYER_SIZE: u8 = 5;
const TARGET_SIZE: u8 = 3;

const LEFT_BUTTON: u8 = 1 << 0;
const RIGHT_BUTTON: u8 = 1 << 1;
const UP_BUTTON: u8 = 1 << 2;
const DOWN_BUTTON: u8 = 1 << 3;
const B_BUTTON: u8 = 1 << 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct GameState {
    pub player_x: u8,
    pub player_y: u8,
    pub target_x: u8,
    pub target_y: u8,
    pub score: u16,
}

static mut STATE: GameState = GameState {
    player_x: (WIDTH - PLAYER_SIZE) / 2,
    player_y: (HEIGHT - PLAYER_SIZE) / 2,
    target_x: 20,
    target_y: 20,
    score: 0,
};

static mut PREVIOUS_BUTTONS: u8 = 0;
static mut RNG_STATE: u16 = 0xace1;

fn next_random() -> u8 {
    unsafe {
        RNG_STATE = RNG_STATE.wrapping_mul(2053).wrapping_add(13849);
        (RNG_STATE >> 8) as u8
    }
}

fn place_target(mut state: GameState) -> GameState {
    state.target_x = next_random() % (WIDTH - TARGET_SIZE);
    state.target_y = HEADER_HEIGHT + (next_random() % (HEIGHT - HEADER_HEIGHT - TARGET_SIZE));
    state
}

fn intersects(state: GameState) -> bool {
    state.player_x < state.target_x + TARGET_SIZE
        && state.player_x + PLAYER_SIZE > state.target_x
        && state.player_y < state.target_y + TARGET_SIZE
        && state.player_y + PLAYER_SIZE > state.target_y
}

#[no_mangle]
pub unsafe extern "C" fn game_init(seed: u16, out: *mut GameState) {
    unsafe {
        RNG_STATE = seed | 1;
        PREVIOUS_BUTTONS = 0;
        STATE = GameState {
            player_x: (WIDTH - PLAYER_SIZE) / 2,
            player_y: (HEIGHT - PLAYER_SIZE) / 2,
            target_x: 20,
            target_y: 20,
            score: 0,
        };
        STATE = place_target(STATE);
        if !out.is_null() {
            *out = STATE;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn game_update(buttons: u8, out: *mut GameState) {
    unsafe {
        let just_pressed = buttons & !PREVIOUS_BUTTONS;
        let mut state = STATE;

        if buttons & LEFT_BUTTON != 0 && state.player_x > 0 {
            state.player_x -= 1;
        }

        if buttons & RIGHT_BUTTON != 0 && state.player_x < WIDTH - PLAYER_SIZE {
            state.player_x += 1;
        }

        if buttons & UP_BUTTON != 0 && state.player_y > HEADER_HEIGHT {
            state.player_y -= 1;
        }

        if buttons & DOWN_BUTTON != 0 && state.player_y < HEIGHT - PLAYER_SIZE {
            state.player_y += 1;
        }

        if just_pressed & B_BUTTON != 0 {
            state.score = 0;
            state.player_x = (WIDTH - PLAYER_SIZE) / 2;
            state.player_y = (HEIGHT - PLAYER_SIZE) / 2;
            state = place_target(state);
        }

        if intersects(state) {
            state.score = state.score.wrapping_add(1);
            state = place_target(state);
        }

        PREVIOUS_BUTTONS = buttons;
        STATE = state;
        if !out.is_null() {
            *out = state;
        }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
