use crate::camera::Camera;
use crate::direction::Direction;
use crate::game::Game;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InputMode {
    Player,
    Camera,
}

pub(crate) struct App {
    game: Game,
    camera: Camera,
    input_mode: InputMode,
}

impl App {
    pub(crate) const fn new() -> Self {
        let game = Game::new();
        let camera = Camera::following(game.player_position());
        Self {
            game,
            camera,
            input_mode: InputMode::Player,
        }
    }

    pub(crate) fn restart(&mut self) {
        self.game.restart();
        self.camera.follow(self.game.player_position());
        self.input_mode = InputMode::Player;
    }

    pub(crate) fn press_direction(&mut self, direction: Direction) {
        match self.input_mode {
            InputMode::Player => {
                self.game.act(Some(direction));
                self.camera.follow(self.game.player_position());
            }
            InputMode::Camera => self.camera.scroll(direction),
        }
    }

    pub(crate) fn press_a(&mut self) {
        if self.input_mode == InputMode::Player {
            self.game.act(None);
            self.camera.follow(self.game.player_position());
        }
    }

    pub(crate) fn press_b(&mut self) {
        if self.input_mode == InputMode::Player {
            self.restart();
        }
    }

    pub(crate) fn toggle_camera_mode(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Player => InputMode::Camera,
            InputMode::Camera => {
                self.camera.follow(self.game.player_position());
                InputMode::Player
            }
        };
    }

    pub(crate) fn game(&self) -> &Game {
        &self.game
    }

    pub(crate) fn camera(&self) -> Camera {
        self.camera
    }

    pub(crate) fn input_mode(&self) -> InputMode {
        self.input_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;

    #[test]
    fn camera_mode_moves_camera_without_moving_player() {
        let mut app = App::new();
        let player_before = app.game.player_position();

        app.toggle_camera_mode();
        app.press_direction(Direction::North);

        assert_eq!(app.game.player_position(), player_before);
        assert_eq!(app.camera.y, 26);
    }

    #[test]
    fn exiting_camera_mode_returns_to_player() {
        let mut app = App::new();
        app.toggle_camera_mode();
        app.press_direction(Direction::North);
        app.press_direction(Direction::North);
        app.toggle_camera_mode();

        assert_eq!(app.input_mode, InputMode::Player);
        assert_eq!(app.camera, Camera::following(app.game.player_position()));
    }

    #[test]
    fn camera_mode_suppresses_stall_and_restart() {
        let mut app = App::new();
        app.press_direction(Direction::East);
        let player_before = app.game.player_position();
        let rat_before = app.game.rat_position();

        app.toggle_camera_mode();
        app.press_a();
        app.press_b();

        assert_eq!(app.input_mode, InputMode::Camera);
        assert_eq!(app.game.player_position(), player_before);
        assert_eq!(app.game.rat_position(), rat_before);
        assert_ne!(player_before, Position::new(3, 6));
    }
}
