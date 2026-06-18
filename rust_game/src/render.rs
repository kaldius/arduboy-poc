use crate::direction::Direction;
use crate::game::{Game, PlayState};
use crate::grid::{Cell, HEIGHT as GRID_HEIGHT, WIDTH as GRID_WIDTH};
use crate::position::Position;

const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 64;
pub(crate) const FRAMEBUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT / 8;

const TILE_SIZE: i8 = 8;
const GRID_X: i8 = 40;

pub(crate) fn render(game: &Game, framebuffer: &mut [u8]) {
    framebuffer.fill(0);

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let position = Position::new(x as i8, y as i8);
            let screen_x = GRID_X + position.x * TILE_SIZE;
            let screen_y = position.y * TILE_SIZE;

            match game.cell(position) {
                Cell::Empty => {}
                Cell::Wall => draw_wall(framebuffer, screen_x, screen_y),
                Cell::Player => {
                    draw_player(framebuffer, screen_x, screen_y, game.player_direction())
                }
                Cell::Rat => draw_rat(framebuffer, screen_x, screen_y, game.rat_direction()),
            }
        }
    }

    match game.state() {
        PlayState::Playing => {}
        PlayState::Won => draw_status(framebuffer, true),
        PlayState::GameOver => draw_status(framebuffer, false),
    }
}

fn set_pixel(framebuffer: &mut [u8], x: i8, y: i8) {
    if x < 0 || y < 0 {
        return;
    }

    let x = x as usize;
    let y = y as usize;
    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
        return;
    }

    framebuffer[(y / 8) * SCREEN_WIDTH + x] |= 1 << (y & 7);
}

fn fill_rect(framebuffer: &mut [u8], x: i8, y: i8, width: i8, height: i8) {
    for py in y..y + height {
        for px in x..x + width {
            set_pixel(framebuffer, px, py);
        }
    }
}

fn draw_wall(framebuffer: &mut [u8], x: i8, y: i8) {
    fill_rect(framebuffer, x, y, TILE_SIZE, TILE_SIZE);
    for offset in [1, 5] {
        for px in x..x + TILE_SIZE {
            let index = ((y + offset) as usize / 8) * SCREEN_WIDTH + px as usize;
            framebuffer[index] &= !(1 << ((y + offset) & 7));
        }
    }
}

fn draw_player(framebuffer: &mut [u8], x: i8, y: i8, direction: Direction) {
    fill_rect(framebuffer, x + 2, y + 2, 4, 4);
    let (dx, dy) = direction.delta();
    set_pixel(framebuffer, x + 4 + dx * 3, y + 4 + dy * 3);
    set_pixel(framebuffer, x + 4 + dx * 2, y + 4 + dy * 2);
}

fn draw_rat(framebuffer: &mut [u8], x: i8, y: i8, direction: Direction) {
    fill_rect(framebuffer, x + 2, y + 3, 4, 3);
    set_pixel(framebuffer, x + 2, y + 2);
    set_pixel(framebuffer, x + 5, y + 2);
    let (dx, dy) = direction.delta();
    set_pixel(framebuffer, x + 4 + dx * 2, y + 4 + dy * 2);
}

fn draw_status(framebuffer: &mut [u8], won: bool) {
    let x = if won { 4 } else { 116 };
    fill_rect(framebuffer, x, 26, 8, 12);
    if won {
        fill_rect(framebuffer, x + 2, 28, 4, 6);
    } else {
        for i in 0..6 {
            set_pixel(framebuffer, x + 1 + i, 28 + i);
            set_pixel(framebuffer, x + 6 - i, 28 + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_writes_a_single_arduboy_framebuffer() {
        let game = Game::new();
        let mut framebuffer = [0; FRAMEBUFFER_SIZE];
        render(&game, &mut framebuffer);
        assert!(framebuffer.iter().any(|&byte| byte != 0));
    }
}
