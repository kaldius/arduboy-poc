#include <Arduboy2.h>

Arduboy2 arduboy;

constexpr uint8_t PLAYER_SIZE = 5;
constexpr uint8_t TARGET_SIZE = 3;

constexpr uint8_t RUST_LEFT_BUTTON = 1 << 0;
constexpr uint8_t RUST_RIGHT_BUTTON = 1 << 1;
constexpr uint8_t RUST_UP_BUTTON = 1 << 2;
constexpr uint8_t RUST_DOWN_BUTTON = 1 << 3;
constexpr uint8_t RUST_A_BUTTON = 1 << 4;
constexpr uint8_t RUST_B_BUTTON = 1 << 5;

struct GameState {
  uint8_t playerX;
  uint8_t playerY;
  uint8_t targetX;
  uint8_t targetY;
  uint16_t score;
};

extern "C" void game_init(uint16_t seed, GameState *state);
extern "C" void game_update(uint8_t buttons, GameState *state);

GameState game;

void setup() {
  arduboy.begin();
  arduboy.setFrameRate(30);
  arduboy.initRandomSeed();
  game_init(random(), &game);
}

uint8_t readButtons() {
  uint8_t buttons = 0;

  if (arduboy.pressed(LEFT_BUTTON)) buttons |= RUST_LEFT_BUTTON;
  if (arduboy.pressed(RIGHT_BUTTON)) buttons |= RUST_RIGHT_BUTTON;
  if (arduboy.pressed(UP_BUTTON)) buttons |= RUST_UP_BUTTON;
  if (arduboy.pressed(DOWN_BUTTON)) buttons |= RUST_DOWN_BUTTON;
  if (arduboy.pressed(A_BUTTON)) buttons |= RUST_A_BUTTON;
  if (arduboy.pressed(B_BUTTON)) buttons |= RUST_B_BUTTON;

  return buttons;
}

void drawGame() {
  arduboy.clear();

  arduboy.setCursor(0, 0);
  arduboy.print(F("Score "));
  arduboy.print(game.score);

  arduboy.drawFastHLine(0, 8, WIDTH);
  arduboy.fillRect(game.targetX, game.targetY, TARGET_SIZE, TARGET_SIZE);
  arduboy.drawRect(game.playerX, game.playerY, PLAYER_SIZE, PLAYER_SIZE);

  arduboy.display();
}

void loop() {
  if (!arduboy.nextFrame()) {
    return;
  }

  game_update(readButtons(), &game);
  drawGame();
}
