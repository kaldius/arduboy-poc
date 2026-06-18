# Arduboy Game

Terminal-first Arduboy project using the local Arduboy2 checkout at `../arduboy2`.

The sketch is an early port of Infestation's `rats` level. Arduboy2/C++ handles
hardware setup, button edge detection, and display upload. The `no_std`,
allocation-free Rust static library owns game state, resolves turns, and writes
directly into Arduboy2's framebuffer.

## Controls

- D-pad: move and face the player, then resolve the rat's turn
- A: stall for one turn
- B: restart the level
- A+B: toggle camera mode; while active, the D-pad scrolls without taking turns.
  Press A+B again to return the camera to the player.
- Left+Right: undo one turn in an active puzzle.
- In the intro, stand on the portal above the player and press A to enter the
  Rats level.
- Continue two cells north to the next portal and press A to enter More Rats.
- The two portals immediately east of More Rats enter Trapped Rat and
  Trapped Rat 2.
- Continue north from More Rats to enter Webs.
- Up+Down: exit the active puzzle and return to the intro.

## Toolchain

Installed locally:

- `arduino-cli` 1.5.1
- `arduino:avr` 1.8.8
- `arduboy:avr` 1.1.0
- Rust nightly with `rust-src`

The default board target is `arduboy:avr:arduboy`. If needed, the sketch can also be built as a Leonardo-compatible target with `FQBN=arduino:avr:leonardo`.

## Commands

```sh
make build
make test
make scenario-test
make list-boards
make upload PORT=/dev/cu.usbmodemXXXX
```

`make scenario-test` runs the host-only core against the original Infestation
scenario fixtures in `../infestation/scenario_tests`. Unsupported mechanics are
reported separately from behavioral failures. The harness is a separate crate
and is not linked into the Arduboy build.

For stricter compiler warnings:

```sh
make build WARNINGS=all
```

To build with the Leonardo profile:

```sh
make build FQBN=arduino:avr:leonardo
```

If setting up on another machine, install the CLI and board packages:

```sh
brew install arduino-cli
arduino-cli core update-index --additional-urls https://arduboy.github.io/board-support/package_arduboy_index.json
arduino-cli core install arduino:avr
arduino-cli core install arduboy:avr --additional-urls https://arduboy.github.io/board-support/package_arduboy_index.json
rustup toolchain install nightly --component rust-src
```

The Rust crate points Cargo at Arduino CLI's bundled AVR GCC:

`/Users/quantf/Library/Arduino15/packages/arduino/tools/avr-gcc/7.3.0-atmel3.6.1-arduino7/bin/avr-gcc`

## Arduboy2 Notes

- Include the library with `#include <Arduboy2.h>` and create an `Arduboy2 arduboy;` object.
- Call `arduboy.begin()` in `setup()`.
- Use `arduboy.setFrameRate(...)` and return early when `!arduboy.nextFrame()`.
- Call `arduboy.pollButtons()` once per rendered frame before using `justPressed()` or `justReleased()`.
- Draw into the screen buffer, then call `arduboy.display()`.
- The display is 128x64 pixels, monochrome.
- Arduboy2 reserves the beginning of EEPROM; game saves should start at `EEPROM_STORAGE_SPACE_START`.
