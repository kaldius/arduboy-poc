ARDUINO_CLI ?= arduino-cli
FQBN ?= arduboy:avr:arduboy
SKETCH ?= Game
BUILD_DIR ?= build
ARDUBOY2_LIB ?= ../arduboy2
WARNINGS ?= default
PORT ?=

.PHONY: build upload list-boards clean

build:
	$(ARDUINO_CLI) compile --fqbn $(FQBN) --library $(ARDUBOY2_LIB) --warnings $(WARNINGS) --build-path $(BUILD_DIR)/$(SKETCH) $(SKETCH)

upload: build
ifndef PORT
	$(error Set PORT=/dev/cu.usbmodemXXXX. Run 'make list-boards' to find the Arduboy port)
endif
	$(ARDUINO_CLI) upload --fqbn $(FQBN) --port $(PORT) --input-dir $(BUILD_DIR)/$(SKETCH)

list-boards:
	$(ARDUINO_CLI) board list

clean:
	rm -rf $(BUILD_DIR)
