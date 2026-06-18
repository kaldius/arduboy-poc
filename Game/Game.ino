#include <Arduboy2.h>

Arduboy2 arduboy;

extern "C" void infestation_init();
extern "C" void infestation_press_up();
extern "C" void infestation_press_down();
extern "C" void infestation_press_left();
extern "C" void infestation_press_right();
extern "C" void infestation_press_a();
extern "C" void infestation_press_b();
extern "C" void infestation_render(uint8_t *framebuffer, uint16_t length);

void setup() {
  arduboy.begin();
  arduboy.setFrameRate(30);
  infestation_init();
}

void dispatchButtonPresses() {
  if (arduboy.justPressed(UP_BUTTON)) infestation_press_up();
  if (arduboy.justPressed(DOWN_BUTTON)) infestation_press_down();
  if (arduboy.justPressed(LEFT_BUTTON)) infestation_press_left();
  if (arduboy.justPressed(RIGHT_BUTTON)) infestation_press_right();
  if (arduboy.justPressed(A_BUTTON)) infestation_press_a();
  if (arduboy.justPressed(B_BUTTON)) infestation_press_b();
}

void loop() {
  if (!arduboy.nextFrame()) {
    return;
  }

  arduboy.pollButtons();
  dispatchButtonPresses();
  infestation_render(arduboy.getBuffer(), WIDTH * HEIGHT / 8);
  arduboy.display();
}
