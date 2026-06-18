use crate::direction::Direction;
use crate::grid::{Cell, Grid};
use crate::level::{self, LevelId};
use crate::position::Position;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlayState {
    Playing,
    Won,
    GameOver,
}

pub(crate) struct Game {
    level_id: LevelId,
    grid: Grid,
    player_position: Position,
    player_direction: Direction,
    rat_position: Option<Position>,
    rat_direction: Direction,
    portal_position: Option<Position>,
    state: PlayState,
}

impl Game {
    pub(crate) const fn new(level_id: LevelId) -> Self {
        let level = level::load(level_id);
        Self {
            level_id: level.id,
            grid: level.grid,
            player_position: level.player_position,
            player_direction: level.player_direction,
            rat_position: level.rat_position,
            rat_direction: level.rat_direction,
            portal_position: level.portal_position,
            state: PlayState::Playing,
        }
    }

    pub(crate) fn restart(&mut self) {
        *self = Self::new(self.level_id);
    }

    pub(crate) fn load(&mut self, level_id: LevelId) {
        *self = Self::new(level_id);
    }

    pub(crate) fn act(&mut self, movement: Option<Direction>) {
        if self.state != PlayState::Playing {
            return;
        }

        let player_moved = movement.is_some();
        if let Some(direction) = movement {
            self.move_player(direction);
        }

        if self.level_id == LevelId::Intro {
            return;
        }

        if self.rat_position.is_none() {
            self.state = PlayState::Won;
            return;
        }

        self.move_rat(player_moved);
    }

    pub(crate) fn cell(&self, position: Position) -> Cell {
        self.grid.cell(position)
    }

    pub(crate) fn player_direction(&self) -> Direction {
        self.player_direction
    }

    pub(crate) const fn player_position(&self) -> Position {
        self.player_position
    }

    pub(crate) fn rat_direction(&self) -> Direction {
        self.rat_direction
    }

    pub(crate) fn state(&self) -> PlayState {
        self.state
    }

    pub(crate) const fn level_id(&self) -> LevelId {
        self.level_id
    }

    pub(crate) const fn width(&self) -> u8 {
        self.grid.width()
    }

    pub(crate) const fn height(&self) -> u8 {
        self.grid.height()
    }

    pub(crate) fn is_on_portal(&self) -> bool {
        self.portal_position == Some(self.player_position)
    }

    fn move_player(&mut self, direction: Direction) {
        self.player_direction = direction;
        let destination = self.player_position.offset(direction);
        let destination_cell = self.cell(destination);
        if destination_cell == Cell::Wall
            || (self.level_id == LevelId::Intro && destination_cell == Cell::Rat)
        {
            return;
        }

        let old_cell = if self.portal_position == Some(self.player_position) {
            Cell::Portal
        } else {
            Cell::Empty
        };
        self.grid.set_cell(self.player_position, old_cell);
        self.player_position = destination;

        if self.rat_position == Some(destination) {
            self.rat_position = None;
        }

        self.grid.set_cell(self.player_position, Cell::Player);
    }

    fn move_rat(&mut self, player_moved: bool) {
        let rat = self.rat_position.unwrap();
        let Some(face_direction) = Direction::toward(rat, self.player_position) else {
            self.state = PlayState::GameOver;
            return;
        };

        let mut directions = [None; 3];
        directions[0] = Some(face_direction);
        if face_direction.is_diagonal() {
            directions[1] = face_direction.x_only();
            directions[2] = face_direction.y_only();
        }

        // The stay option matches Infestation's rat move ordering.
        let mut best_direction = None;
        let mut best_score = rat.distance_squared(self.player_position);
        let mut best_cost = 0;
        let mut best_player_still = true;
        let mut best_rank = u8::MAX;

        for direction in directions.into_iter().flatten() {
            let destination = rat.offset(direction);
            if matches!(self.cell(destination), Cell::Wall | Cell::Rat) {
                continue;
            }

            if destination == self.player_position && direction == self.player_direction.opposite()
            {
                continue;
            }

            let score = destination.distance_squared(self.player_position);
            let cost = direction.movement_cost();
            let player_still = !player_moved;
            let rank = direction.tie_break_rank();

            if (score, cost, player_still, rank)
                < (best_score, best_cost, best_player_still, best_rank)
            {
                best_direction = Some(direction);
                best_score = score;
                best_cost = cost;
                best_player_still = player_still;
                best_rank = rank;
            }
        }

        let Some(direction) = best_direction else {
            self.rat_direction = face_direction;
            return;
        };

        let destination = rat.offset(direction);
        self.grid.set_cell(rat, Cell::Empty);
        self.rat_position = Some(destination);
        self.rat_direction = direction;

        if destination == self.player_position {
            self.state = PlayState::GameOver;
        }

        self.grid.set_cell(destination, Cell::Rat);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encounter(player_direction: Direction) -> Game {
        let mut game = Game::new(LevelId::Rats);
        game.grid.fill(Cell::Empty);
        game.player_position = Position::new(2, 2);
        game.player_direction = player_direction;
        game.rat_position = Some(Position::new(2, 1));
        game.rat_direction = Direction::South;
        game.state = PlayState::Playing;
        game.grid.set_cell(game.player_position, Cell::Player);
        game.grid.set_cell(game.rat_position.unwrap(), Cell::Rat);
        game
    }

    #[test]
    fn initial_level_matches_rats_map() {
        let game = Game::new(LevelId::Rats);
        assert_eq!(game.player_position, Position::new(3, 6));
        assert_eq!(game.rat_position, Some(Position::new(3, 0)));
        assert_eq!(game.grid.count(Cell::Wall), 21);
    }

    #[test]
    fn blocked_move_changes_facing_but_not_position() {
        let mut game = Game::new(LevelId::Rats);
        game.act(Some(Direction::North));
        assert_eq!(game.player_position, Position::new(3, 6));
        assert_eq!(game.player_direction, Direction::North);
    }

    #[test]
    fn intro_portal_is_restored_after_player_leaves() {
        let mut game = Game::new(LevelId::Intro);
        let portal = Position::new(4, 9);

        game.act(Some(Direction::North));
        assert!(game.is_on_portal());
        game.act(Some(Direction::South));

        assert_eq!(game.cell(portal), Cell::Portal);
    }

    #[test]
    fn sword_blocks_a_rat_attacking_from_the_front() {
        let mut game = encounter(Direction::North);
        game.act(None);
        assert_eq!(game.state, PlayState::Playing);
        assert_eq!(game.rat_position, Some(Position::new(2, 1)));
    }

    #[test]
    fn rat_attacking_from_the_side_causes_game_over() {
        let mut game = encounter(Direction::East);
        game.act(None);
        assert_eq!(game.state, PlayState::GameOver);
        assert_eq!(game.rat_position, Some(Position::new(2, 2)));
    }

    #[test]
    fn player_moving_into_a_rat_wins() {
        let mut game = encounter(Direction::South);
        game.act(Some(Direction::North));
        assert_eq!(game.state, PlayState::Won);
        assert_eq!(game.rat_position, None);
        assert_eq!(game.player_position, Position::new(2, 1));
    }
}
