ARDUINO_CLI ?= arduino-cli
FQBN ?= arduboy:avr:arduboy
SKETCH ?= Game
BUILD_DIR ?= build
ARDUBOY2_LIB ?= ../arduboy2
WARNINGS ?= default
PORT ?=
RUST_LIB ?= rust_game/target/avr-none/release/libarduboy_game_rust.a

.PHONY: build upload list-boards clean

$(RUST_LIB): rust_game/src/lib.rs rust_game/Cargo.toml rust_game/.cargo/config.toml rust_game/rust-toolchain.toml
	cd rust_game && cargo build --release

build: $(RUST_LIB)
	$(ARDUINO_CLI) compile --fqbn $(FQBN) --library $(ARDUBOY2_LIB) --warnings $(WARNINGS) --build-path $(BUILD_DIR)/$(SKETCH) --build-property "compiler.libraries.ldflags=$(abspath $(RUST_LIB))" $(SKETCH)

upload: build
ifndef PORT
	$(error Set PORT=/dev/cu.usbmodemXXXX. Run 'make list-boards' to find the Arduboy port)
endif
	$(ARDUINO_CLI) upload --fqbn $(FQBN) --port $(PORT) --input-dir $(BUILD_DIR)/$(SKETCH)

list-boards:
	$(ARDUINO_CLI) board list

clean:
	rm -rf $(BUILD_DIR)
	cd rust_game && cargo clean
