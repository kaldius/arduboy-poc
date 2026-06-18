use crate::app::{App, InputMode};
use crate::camera::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use crate::direction::Direction;
use crate::game::PlayState;
use crate::grid::Cell;
use crate::position::Position;

pub(crate) const FRAMEBUFFER_SIZE: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize / 8;

pub(crate) fn render(app: &App, framebuffer: &mut [u8]) {
    framebuffer.fill(0);
    let game = app.game();
    let camera = app.camera();

    for y in 0..game.height() {
        for x in 0..game.width() {
            let position = Position::new(x as i8, y as i8);
            let screen_x = i16::from(position.x) * TILE_SIZE - camera.x;
            let screen_y = i16::from(position.y) * TILE_SIZE - camera.y;

            match game.cell(position) {
                Cell::Empty => {}
                Cell::Wall => draw_wall(framebuffer, screen_x, screen_y),
                Cell::Portal => draw_portal(framebuffer, screen_x, screen_y),
                Cell::Player => {
                    draw_player(framebuffer, screen_x, screen_y, game.player_direction())
                }
                Cell::Rat => draw_rat(
                    framebuffer,
                    screen_x,
                    screen_y,
                    game.rat_direction_at(position),
                ),
            }
        }
    }

    match game.state() {
        PlayState::Playing => {}
        PlayState::Won => draw_status(framebuffer, true),
        PlayState::GameOver => draw_status(framebuffer, false),
    }

    if app.input_mode() == InputMode::Camera {
        draw_camera_mode_indicator(framebuffer);
    }
}

fn set_pixel(framebuffer: &mut [u8], x: i16, y: i16) {
    if x < 0 || y < 0 {
        return;
    }

    let x = x as usize;
    let y = y as usize;
    if x >= SCREEN_WIDTH as usize || y >= SCREEN_HEIGHT as usize {
        return;
    }

    framebuffer[(y / 8) * SCREEN_WIDTH as usize + x] |= 1 << (y & 7);
}

fn fill_rect(framebuffer: &mut [u8], x: i16, y: i16, width: i16, height: i16) {
    for py in y..y + height {
        for px in x..x + width {
            set_pixel(framebuffer, px, py);
        }
    }
}

fn draw_wall(framebuffer: &mut [u8], x: i16, y: i16) {
    fill_rect(framebuffer, x, y, TILE_SIZE, TILE_SIZE);
    for offset in [2, 8] {
        for px in x..x + TILE_SIZE {
            clear_pixel(framebuffer, px, y + offset);
        }
    }
}

fn clear_pixel(framebuffer: &mut [u8], x: i16, y: i16) {
    if x < 0 || y < 0 {
        return;
    }

    let x = x as usize;
    let y = y as usize;
    if x >= SCREEN_WIDTH as usize || y >= SCREEN_HEIGHT as usize {
        return;
    }

    framebuffer[(y / 8) * SCREEN_WIDTH as usize + x] &= !(1 << (y & 7));
}

fn draw_player(framebuffer: &mut [u8], x: i16, y: i16, direction: Direction) {
    fill_rect(framebuffer, x + 3, y + 3, 6, 6);
    let (dx, dy) = direction.delta();
    let dx = i16::from(dx);
    let dy = i16::from(dy);
    set_pixel(framebuffer, x + 6 + dx * 5, y + 6 + dy * 5);
    set_pixel(framebuffer, x + 6 + dx * 4, y + 6 + dy * 4);
}

fn draw_rat(framebuffer: &mut [u8], x: i16, y: i16, direction: Direction) {
    fill_rect(framebuffer, x + 3, y + 5, 6, 4);
    set_pixel(framebuffer, x + 3, y + 3);
    set_pixel(framebuffer, x + 8, y + 3);
    let (dx, dy) = direction.delta();
    set_pixel(
        framebuffer,
        x + 6 + i16::from(dx) * 4,
        y + 6 + i16::from(dy) * 4,
    );
}

fn draw_portal(framebuffer: &mut [u8], x: i16, y: i16) {
    for inset in [1, 3, 5] {
        let size = TILE_SIZE - inset * 2;
        for px in x + inset..x + inset + size {
            set_pixel(framebuffer, px, y + inset);
            set_pixel(framebuffer, px, y + inset + size - 1);
        }
        for py in y + inset..y + inset + size {
            set_pixel(framebuffer, x + inset, py);
            set_pixel(framebuffer, x + inset + size - 1, py);
        }
    }
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

fn draw_camera_mode_indicator(framebuffer: &mut [u8]) {
    for x in 1..9 {
        set_pixel(framebuffer, x, 1);
        set_pixel(framebuffer, x, 8);
    }
    for y in 1..9 {
        set_pixel(framebuffer, 1, y);
        set_pixel(framebuffer, 8, y);
    }
    set_pixel(framebuffer, 4, 3);
    set_pixel(framebuffer, 5, 3);
    set_pixel(framebuffer, 3, 4);
    set_pixel(framebuffer, 6, 4);
    set_pixel(framebuffer, 3, 5);
    set_pixel(framebuffer, 6, 5);
    set_pixel(framebuffer, 4, 6);
    set_pixel(framebuffer, 5, 6);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_writes_a_single_arduboy_framebuffer() {
        let app = App::new();
        let mut framebuffer = [0; FRAMEBUFFER_SIZE];
        render(&app, &mut framebuffer);
        assert!(framebuffer.iter().any(|&byte| byte != 0));
    }

    #[test]
    fn scrolling_changes_the_rendered_frame() {
        let mut app = App::new();
        let mut before = [0; FRAMEBUFFER_SIZE];
        let mut after = [0; FRAMEBUFFER_SIZE];

        render(&app, &mut before);
        app.toggle_camera_mode();
        app.press_direction(Direction::North);
        render(&app, &mut after);

        assert_ne!(before, after);
    }
}
