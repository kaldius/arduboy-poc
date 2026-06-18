#include <Arduboy2.h>

Arduboy2 arduboy;

extern "C" void infestation_init();
extern "C" void infestation_press_up();
extern "C" void infestation_press_down();
extern "C" void infestation_press_left();
extern "C" void infestation_press_right();
extern "C" void infestation_press_a();
extern "C" void infestation_press_b();
extern "C" void infestation_press_ab();
extern "C" void infestation_press_up_down();
extern "C" void infestation_press_left_right();
extern "C" void infestation_render(uint8_t *framebuffer, uint16_t length);

void setup() {
  arduboy.begin();
  arduboy.setFrameRate(30);
  infestation_init();
}

void dispatchButtonPresses() {
  const bool abPressed =
      arduboy.pressed(A_BUTTON) && arduboy.pressed(B_BUTTON);
  const bool abJustPressed =
      arduboy.justPressed(A_BUTTON) || arduboy.justPressed(B_BUTTON);

  if (abPressed && abJustPressed) {
    infestation_press_ab();
    return;
  }

  const bool upDownPressed =
      arduboy.pressed(UP_BUTTON) && arduboy.pressed(DOWN_BUTTON);
  const bool upDownJustPressed =
      arduboy.justPressed(UP_BUTTON) || arduboy.justPressed(DOWN_BUTTON);

  if (upDownPressed && upDownJustPressed) {
    infestation_press_up_down();
    return;
  }

  const bool leftRightPressed =
      arduboy.pressed(LEFT_BUTTON) && arduboy.pressed(RIGHT_BUTTON);
  const bool leftRightJustPressed =
      arduboy.justPressed(LEFT_BUTTON) || arduboy.justPressed(RIGHT_BUTTON);

  if (leftRightPressed && leftRightJustPressed) {
    infestation_press_left_right();
    return;
  }

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
