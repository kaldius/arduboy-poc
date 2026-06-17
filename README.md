# Arduboy Game

Terminal-first Arduboy project using the local Arduboy2 checkout at `../arduboy2`.

## Toolchain

Installed locally:

- `arduino-cli` 1.5.1
- `arduino:avr` 1.8.8
- `arduboy:avr` 1.1.0

The default board target is `arduboy:avr:arduboy`. If needed, the sketch can also be built as a Leonardo-compatible target with `FQBN=arduino:avr:leonardo`.

## Commands

```sh
make build
make list-boards
make upload PORT=/dev/cu.usbmodemXXXX
```

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
```

## Arduboy2 Notes

- Include the library with `#include <Arduboy2.h>` and create an `Arduboy2 arduboy;` object.
- Call `arduboy.begin()` in `setup()`.
- Use `arduboy.setFrameRate(...)` and return early when `!arduboy.nextFrame()`.
- Call `arduboy.pollButtons()` once per rendered frame before using `justPressed()` or `justReleased()`.
- Draw into the screen buffer, then call `arduboy.display()`.
- The display is 128x64 pixels, monochrome.
- Arduboy2 reserves the beginning of EEPROM; game saves should start at `EEPROM_STORAGE_SPACE_START`.
