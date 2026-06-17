#include <Arduboy2.h>

Arduboy2 arduboy;

constexpr uint8_t PLAYER_SIZE = 5;
constexpr uint8_t TARGET_SIZE = 3;

int8_t playerX = (WIDTH - PLAYER_SIZE) / 2;
int8_t playerY = (HEIGHT - PLAYER_SIZE) / 2;
uint8_t targetX = 20;
uint8_t targetY = 20;
uint16_t score = 0;

void placeTarget() {
  targetX = random(0, WIDTH - TARGET_SIZE);
  targetY = random(9, HEIGHT - TARGET_SIZE);
}

void setup() {
  arduboy.begin();
  arduboy.setFrameRate(30);
  arduboy.initRandomSeed();
  placeTarget();
}

void updateGame() {
  if (arduboy.pressed(LEFT_BUTTON) && playerX > 0) {
    --playerX;
  }

  if (arduboy.pressed(RIGHT_BUTTON) && playerX < WIDTH - PLAYER_SIZE) {
    ++playerX;
  }

  if (arduboy.pressed(UP_BUTTON) && playerY > 9) {
    --playerY;
  }

  if (arduboy.pressed(DOWN_BUTTON) && playerY < HEIGHT - PLAYER_SIZE) {
    ++playerY;
  }

  if (arduboy.justPressed(B_BUTTON)) {
    score = 0;
    playerX = (WIDTH - PLAYER_SIZE) / 2;
    playerY = (HEIGHT - PLAYER_SIZE) / 2;
    placeTarget();
  }

  Rect player = { playerX, playerY, PLAYER_SIZE, PLAYER_SIZE };
  Rect target = { targetX, targetY, TARGET_SIZE, TARGET_SIZE };

  if (arduboy.collide(player, target)) {
    ++score;
    placeTarget();
  }
}

void drawGame() {
  arduboy.clear();

  arduboy.setCursor(0, 0);
  arduboy.print(F("Score "));
  arduboy.print(score);

  arduboy.drawFastHLine(0, 8, WIDTH);
  arduboy.fillRect(targetX, targetY, TARGET_SIZE, TARGET_SIZE);
  arduboy.drawRect(playerX, playerY, PLAYER_SIZE, PLAYER_SIZE);

  arduboy.display();
}

void loop() {
  if (!arduboy.nextFrame()) {
    return;
  }

  arduboy.pollButtons();
  updateGame();
  drawGame();
}
