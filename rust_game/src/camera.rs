use crate::direction::Direction;
use crate::grid::{HEIGHT as GRID_HEIGHT, WIDTH as GRID_WIDTH};
use crate::position::Position;

pub(crate) const SCREEN_WIDTH: i16 = 128;
pub(crate) const SCREEN_HEIGHT: i16 = 64;
pub(crate) const TILE_SIZE: i16 = 12;

const WORLD_WIDTH: i16 = GRID_WIDTH as i16 * TILE_SIZE;
const WORLD_HEIGHT: i16 = GRID_HEIGHT as i16 * TILE_SIZE;
const SCROLL_STEP: i16 = TILE_SIZE / 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Camera {
    pub(crate) x: i16,
    pub(crate) y: i16,
}

impl Camera {
    pub(crate) const fn following(position: Position) -> Self {
        let mut camera = Self { x: 0, y: 0 };
        camera.center_on(position);
        camera
    }

    pub(crate) fn follow(&mut self, position: Position) {
        self.center_on(position);
    }

    pub(crate) fn scroll(&mut self, direction: Direction) {
        let (dx, dy) = direction.delta();
        self.x = clamp_x(self.x + i16::from(dx) * SCROLL_STEP);
        self.y = clamp_y(self.y + i16::from(dy) * SCROLL_STEP);
    }

    const fn center_on(&mut self, position: Position) {
        let center_x = position.x as i16 * TILE_SIZE + TILE_SIZE / 2;
        let center_y = position.y as i16 * TILE_SIZE + TILE_SIZE / 2;
        self.x = clamp_x(center_x - SCREEN_WIDTH / 2);
        self.y = clamp_y(center_y - SCREEN_HEIGHT / 2);
    }
}

const fn clamp_x(value: i16) -> i16 {
    clamp_axis(value, WORLD_WIDTH, SCREEN_WIDTH)
}

const fn clamp_y(value: i16) -> i16 {
    clamp_axis(value, WORLD_HEIGHT, SCREEN_HEIGHT)
}

const fn clamp_axis(value: i16, world_size: i16, screen_size: i16) -> i16 {
    if world_size <= screen_size {
        -(screen_size - world_size) / 2
    } else if value < 0 {
        0
    } else if value > world_size - screen_size {
        world_size - screen_size
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_centers_a_world_that_is_narrower_than_the_screen() {
        let camera = Camera::following(Position::new(3, 6));
        assert_eq!(camera.x, -28);
    }

    #[test]
    fn camera_scrolls_and_clamps_to_world_bounds() {
        let mut camera = Camera::following(Position::new(3, 6));
        assert_eq!(camera.y, 32);

        for _ in 0..20 {
            camera.scroll(Direction::North);
        }
        assert_eq!(camera.y, 0);

        for _ in 0..20 {
            camera.scroll(Direction::South);
        }
        assert_eq!(camera.y, 32);
    }
}
