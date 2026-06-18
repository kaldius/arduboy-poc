use crate::camera::Camera;
use crate::direction::Direction;
use crate::game::{Game, PlayState};
use crate::level::LevelId;

// Tradeoff knob: each undo action costs one byte of SRAM.
pub(crate) const MAX_UNDO_HISTORY: usize = 160;
const _: () = assert!(MAX_UNDO_HISTORY <= u8::MAX as usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InputMode {
    Player,
    Camera,
}

pub(crate) struct App {
    game: Game,
    undo: UndoHistory,
    camera: Camera,
    input_mode: InputMode,
    completed_levels: u8,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TurnAction {
    Empty,
    Stall,
    MoveEast,
    MoveWest,
    MoveNorth,
    MoveSouth,
}

impl TurnAction {
    const fn from_movement(movement: Option<Direction>) -> Self {
        match movement {
            None => Self::Stall,
            Some(Direction::East) => Self::MoveEast,
            Some(Direction::West) => Self::MoveWest,
            Some(Direction::North) => Self::MoveNorth,
            Some(Direction::South) => Self::MoveSouth,
            Some(Direction::Northeast)
            | Some(Direction::Northwest)
            | Some(Direction::Southeast)
            | Some(Direction::Southwest) => Self::Stall,
        }
    }

    const fn movement(self) -> Option<Direction> {
        match self {
            Self::Empty | Self::Stall => None,
            Self::MoveEast => Some(Direction::East),
            Self::MoveWest => Some(Direction::West),
            Self::MoveNorth => Some(Direction::North),
            Self::MoveSouth => Some(Direction::South),
        }
    }
}

struct UndoHistory {
    actions: [TurnAction; MAX_UNDO_HISTORY],
    len: u8,
    baseline_game: Game,
    baseline_completed_levels: u8,
}

impl UndoHistory {
    const fn new() -> Self {
        let game = Game::new(LevelId::Intro);
        Self {
            actions: [TurnAction::Empty; MAX_UNDO_HISTORY],
            len: 0,
            baseline_game: game,
            baseline_completed_levels: 0,
        }
    }

    fn reset(&mut self, game: Game, completed_levels: u8) {
        self.len = 0;
        self.baseline_game = game;
        self.baseline_completed_levels = completed_levels;
    }

    const fn available(&self) -> bool {
        self.len != 0
    }

    fn record(&mut self, action: TurnAction) {
        if usize::from(self.len) == MAX_UNDO_HISTORY {
            self.advance_baseline();
            self.discard_oldest_action();
        }

        self.actions[usize::from(self.len)] = action;
        self.len += 1;
    }

    fn undo(&mut self) -> Option<(Game, u8)> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        Some(self.replay())
    }

    fn replay(&self) -> (Game, u8) {
        let mut game = self.baseline_game;
        let mut completed_levels = self.baseline_completed_levels;
        let mut index = 0;
        while index < self.len {
            apply_action(
                &mut game,
                self.actions[index as usize],
                &mut completed_levels,
            );
            index += 1;
        }
        (game, completed_levels)
    }

    fn advance_baseline(&mut self) {
        apply_action(
            &mut self.baseline_game,
            self.actions[0],
            &mut self.baseline_completed_levels,
        );
    }

    fn discard_oldest_action(&mut self) {
        let mut index = 1;
        while index < MAX_UNDO_HISTORY {
            self.actions[index - 1] = self.actions[index];
            index += 1;
        }
        self.actions[MAX_UNDO_HISTORY - 1] = TurnAction::Empty;
        self.len -= 1;
    }
}

fn apply_action(game: &mut Game, action: TurnAction, completed_levels: &mut u8) {
    game.act(action.movement());
    if game.state() == PlayState::Won {
        *completed_levels |= game.level_id().completion_mask();
    }
}

impl App {
    pub(crate) const fn new() -> Self {
        let game = Game::new(LevelId::Intro);
        let camera = Camera::following(game.player_position(), game.width(), game.height());
        Self {
            game,
            undo: UndoHistory::new(),
            camera,
            input_mode: InputMode::Player,
            completed_levels: 0,
        }
    }

    pub(crate) fn restart(&mut self) {
        self.game.restart();
        self.clear_undo();
        self.reset_camera();
        self.input_mode = InputMode::Player;
    }

    pub(crate) fn press_direction(&mut self, direction: Direction) {
        match self.input_mode {
            InputMode::Player => {
                self.record_undo_action(TurnAction::from_movement(Some(direction)));
                self.game.act(Some(direction));
                self.record_completion();
                self.camera.follow(self.game.player_position());
            }
            InputMode::Camera => self.camera.scroll(direction),
        }
    }

    pub(crate) fn press_a(&mut self) {
        if self.input_mode != InputMode::Player {
            return;
        }

        if let Some(destination) = self.game.portal_destination() {
            self.load_level(destination);
        } else {
            self.record_undo_action(TurnAction::Stall);
            self.game.act(None);
            self.record_completion();
            self.camera.follow(self.game.player_position());
        }
    }

    pub(crate) fn press_b(&mut self) {
        self.undo();
    }

    pub(crate) fn undo(&mut self) {
        if self.input_mode == InputMode::Player
            && self.game.level_id() != LevelId::Intro
            && self.undo.available()
        {
            if let Some((game, completed_levels)) = self.undo.undo() {
                self.game = game;
                self.completed_levels = completed_levels;
                self.reset_camera();
            }
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

    pub(crate) fn exit_level(&mut self) {
        if self.game.level_id() != LevelId::Intro {
            self.load_level(LevelId::Intro);
        }
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

    fn load_level(&mut self, level_id: LevelId) {
        self.game.load(level_id);
        self.clear_undo();
        self.reset_camera();
        self.input_mode = InputMode::Player;
    }

    fn reset_camera(&mut self) {
        self.camera = Camera::following(
            self.game.player_position(),
            self.game.width(),
            self.game.height(),
        );
    }

    fn record_undo_action(&mut self, action: TurnAction) {
        if self.game.level_id() != LevelId::Intro {
            self.undo.record(action);
        }
    }

    fn clear_undo(&mut self) {
        self.undo.reset(self.game, self.completed_levels);
    }

    fn record_completion(&mut self) {
        if self.game.state() == PlayState::Won {
            self.completed_levels |= self.game.level_id().completion_mask();
        }
    }

    pub(crate) fn is_completed(&self, level_id: LevelId) -> bool {
        self.completed_levels & level_id.completion_mask() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;

    #[test]
    fn starts_in_intro() {
        let app = App::new();
        assert_eq!(app.game.level_id(), LevelId::Intro);
        assert_eq!(app.game.player_position(), Position::new(4, 10));
    }

    #[test]
    fn camera_mode_moves_camera_without_moving_player() {
        let mut app = App::new();
        let player_before = app.game.player_position();
        let camera_before = app.camera;

        app.toggle_camera_mode();
        app.press_direction(Direction::North);

        assert_eq!(app.game.player_position(), player_before);
        assert!(app.camera.y < camera_before.y);
    }

    #[test]
    fn exiting_camera_mode_returns_to_player() {
        let mut app = App::new();
        app.toggle_camera_mode();
        app.press_direction(Direction::North);
        app.press_direction(Direction::North);
        app.toggle_camera_mode();

        assert_eq!(app.input_mode, InputMode::Player);
        assert_eq!(
            app.camera,
            Camera::following(
                app.game.player_position(),
                app.game.width(),
                app.game.height()
            )
        );
    }

    #[test]
    fn camera_mode_suppresses_stall_and_undo() {
        let mut app = App::new();
        app.load_level(LevelId::Rats);
        let initial_position = app.game.player_position();
        app.press_direction(Direction::East);
        let player_before = app.game.player_position();

        app.toggle_camera_mode();
        app.press_a();
        app.press_b();

        assert_eq!(app.input_mode, InputMode::Camera);
        assert_eq!(app.game.player_position(), player_before);
        assert_ne!(player_before, initial_position);
    }

    #[test]
    fn a_enters_rats_level_from_intro_portal() {
        let mut app = App::new();
        app.press_direction(Direction::North);
        assert_eq!(app.game.portal_destination(), Some(LevelId::Rats));

        app.press_a();

        assert_eq!(app.game.level_id(), LevelId::Rats);
        assert_eq!(app.game.player_position(), Position::new(3, 6));
    }

    #[test]
    fn a_enters_more_rats_from_second_intro_portal() {
        let mut app = App::new();
        app.press_direction(Direction::North);
        app.press_direction(Direction::North);
        app.press_direction(Direction::North);
        assert_eq!(app.game.portal_destination(), Some(LevelId::MoreRats));

        app.press_a();

        assert_eq!(app.game.level_id(), LevelId::MoreRats);
        assert_eq!(app.game.player_position(), Position::new(3, 5));
    }

    #[test]
    fn a_enters_both_trapped_rat_portals() {
        let mut app = App::new();
        for _ in 0..3 {
            app.press_direction(Direction::North);
        }
        app.press_direction(Direction::East);
        assert_eq!(app.game.portal_destination(), Some(LevelId::TrappedRat));
        app.press_a();
        assert_eq!(app.game.level_id(), LevelId::TrappedRat);

        app.exit_level();
        for _ in 0..3 {
            app.press_direction(Direction::North);
        }
        app.press_direction(Direction::East);
        app.press_direction(Direction::East);
        assert_eq!(app.game.portal_destination(), Some(LevelId::TrappedRat2));
        app.press_a();
        assert_eq!(app.game.level_id(), LevelId::TrappedRat2);
    }

    #[test]
    fn a_enters_webs_portal() {
        let mut app = App::new();
        for _ in 0..5 {
            app.press_direction(Direction::North);
        }
        assert_eq!(app.game.portal_destination(), Some(LevelId::Webs));

        app.press_a();

        assert_eq!(app.game.level_id(), LevelId::Webs);
        assert_eq!(app.game.player_position(), Position::new(2, 6));
    }

    #[test]
    fn up_down_chord_returns_to_intro() {
        let mut app = App::new();
        app.press_direction(Direction::North);
        app.press_a();
        assert_eq!(app.game.level_id(), LevelId::Rats);

        app.exit_level();

        assert_eq!(app.game.level_id(), LevelId::Intro);
        assert_eq!(app.game.player_position(), Position::new(4, 10));
    }

    #[test]
    fn undo_restores_previous_puzzle_state() {
        let mut app = App::new();
        app.load_level(LevelId::Rats);

        let before = app.game.player_position();
        app.press_direction(Direction::East);
        assert_ne!(app.game.player_position(), before);

        app.undo();

        assert_eq!(app.game.player_position(), before);
    }

    #[test]
    fn undo_can_walk_back_multiple_turns() {
        let mut app = App::new();
        app.load_level(LevelId::Rats);

        let start = app.game.player_position();
        app.press_direction(Direction::East);
        let after_first_turn = app.game.player_position();
        app.press_direction(Direction::West);
        assert_eq!(app.game.player_position(), start);

        app.undo();
        assert_eq!(app.game.player_position(), after_first_turn);

        app.undo();
        assert_eq!(app.game.player_position(), start);
    }

    #[test]
    fn b_undoes_previous_puzzle_turn() {
        let mut app = App::new();
        app.load_level(LevelId::Rats);

        let before = app.game.player_position();
        app.press_direction(Direction::East);
        assert_ne!(app.game.player_position(), before);

        app.press_b();

        assert_eq!(app.game.player_position(), before);
    }

    #[test]
    fn completed_level_portal_is_tracked() {
        let mut app = App::new();
        app.load_level(LevelId::Rats);
        assert!(!app.is_completed(LevelId::Rats));

        app.game.force_win();
        app.record_completion();

        assert!(app.is_completed(LevelId::Rats));
    }

    #[test]
    fn undo_reverts_completion_from_winning_turn() {
        let mut app = App::new();
        app.load_level(LevelId::Rats);
        app.record_undo_action(TurnAction::Stall);
        app.game.force_win();
        app.record_completion();
        assert!(app.is_completed(LevelId::Rats));

        app.undo();

        assert!(!app.is_completed(LevelId::Rats));
    }
}
