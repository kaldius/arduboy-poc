#![cfg_attr(not(test), no_std)]

mod app;
mod camera;
mod direction;
mod ffi;
mod game;
mod grid;
mod level;
mod position;
mod render;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
