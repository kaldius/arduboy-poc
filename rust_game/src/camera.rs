use crate::direction::Direction;
use crate::position::Position;

pub(crate) const SCREEN_WIDTH: i16 = 128;
pub(crate) const SCREEN_HEIGHT: i16 = 64;
pub(crate) const TILE_SIZE: i16 = 12;

const SCROLL_STEP: i16 = TILE_SIZE / 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Camera {
    pub(crate) x: i16,
    pub(crate) y: i16,
    world_width: i16,
    world_height: i16,
}

impl Camera {
    pub(crate) const fn following(position: Position, grid_width: u8, grid_height: u8) -> Self {
        let mut camera = Self {
            x: 0,
            y: 0,
            world_width: grid_width as i16 * TILE_SIZE,
            world_height: grid_height as i16 * TILE_SIZE,
        };
        camera.center_on(position);
        camera
    }

    pub(crate) fn follow(&mut self, position: Position) {
        self.center_on(position);
    }

    pub(crate) fn scroll(&mut self, direction: Direction) {
        let (dx, dy) = direction.delta();
        self.x = clamp_axis(
            self.x + i16::from(dx) * SCROLL_STEP,
            self.world_width,
            SCREEN_WIDTH,
        );
        self.y = clamp_axis(
            self.y + i16::from(dy) * SCROLL_STEP,
            self.world_height,
            SCREEN_HEIGHT,
        );
    }

    const fn center_on(&mut self, position: Position) {
        let center_x = position.x as i16 * TILE_SIZE + TILE_SIZE / 2;
        let center_y = position.y as i16 * TILE_SIZE + TILE_SIZE / 2;
        self.x = clamp_axis(center_x - SCREEN_WIDTH / 2, self.world_width, SCREEN_WIDTH);
        self.y = clamp_axis(
            center_y - SCREEN_HEIGHT / 2,
            self.world_height,
            SCREEN_HEIGHT,
        );
    }
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
        let camera = Camera::following(Position::new(3, 6), 6, 8);
        assert_eq!(camera.x, -28);
    }

    #[test]
    fn camera_scrolls_and_clamps_to_world_bounds() {
        let mut camera = Camera::following(Position::new(3, 6), 6, 8);
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
