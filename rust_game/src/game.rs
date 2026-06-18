use crate::direction::Direction;
#[cfg(feature = "scenario-harness")]
use crate::grid::MAX_CELL_COUNT;
use crate::grid::{Cell, Grid};
use crate::level::{self, LevelId, Portal, MAX_PORTALS, MAX_RATS};
use crate::position::Position;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlayState {
    Playing,
    Won,
    GameOver,
}

#[derive(Clone, Copy)]
struct Rat {
    position: Position,
    direction: Direction,
    alive: bool,
}

impl Rat {
    const EMPTY: Self = Self {
        position: Position::new(0, 0),
        direction: Direction::South,
        alive: false,
    };
}

pub(crate) struct Game {
    level_id: LevelId,
    grid: Grid,
    player_position: Position,
    player_direction: Direction,
    rats: [Rat; MAX_RATS],
    initial_rat_count: u8,
    portals: [Portal; MAX_PORTALS],
    portal_count: u8,
    state: PlayState,
}

impl Game {
    pub(crate) const fn new(level_id: LevelId) -> Self {
        let level = level::load(level_id);
        let mut rats = [Rat::EMPTY; MAX_RATS];
        let mut index = 0;
        while index < level.rat_count as usize {
            rats[index] = Rat {
                position: level.rats[index].position,
                direction: level.rats[index].direction,
                alive: true,
            };
            index += 1;
        }

        Self {
            level_id: level.id,
            grid: level.grid,
            player_position: level.player_position,
            player_direction: level.player_direction,
            rats,
            initial_rat_count: level.rat_count,
            portals: level.portals,
            portal_count: level.portal_count,
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

        if self.rat_count() == 0 {
            if self.level_id != LevelId::Intro && self.initial_rat_count != 0 {
                self.state = PlayState::Won;
            }
            return;
        }

        self.move_rats(player_moved);
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

    pub(crate) fn rat_direction_at(&self, position: Position) -> Direction {
        self.rats
            .iter()
            .find(|rat| rat.alive && rat.position == position)
            .map(|rat| rat.direction)
            .unwrap_or(Direction::South)
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

    pub(crate) fn portal_destination(&self) -> Option<LevelId> {
        self.portals[..self.portal_count as usize]
            .iter()
            .find(|portal| portal.position == self.player_position)
            .map(|portal| portal.destination)
    }

    #[cfg(feature = "scenario-harness")]
    pub(crate) fn from_scenario(
        width: u8,
        height: u8,
        cells: [Cell; MAX_CELL_COUNT],
        player_position: Position,
        player_direction: Direction,
        rat_positions: [Position; MAX_RATS],
        rat_directions: [Direction; MAX_RATS],
        rat_count: u8,
    ) -> Self {
        let mut rats = [Rat::EMPTY; MAX_RATS];
        for index in 0..rat_count as usize {
            rats[index] = Rat {
                position: rat_positions[index],
                direction: rat_directions[index],
                alive: true,
            };
        }

        Self {
            level_id: LevelId::Rats,
            grid: Grid::new(width, height, cells),
            player_position,
            player_direction,
            rats,
            initial_rat_count: rat_count,
            portals: [Portal {
                position: Position::new(0, 0),
                destination: LevelId::Intro,
            }; MAX_PORTALS],
            portal_count: 0,
            state: PlayState::Playing,
        }
    }

    fn move_player(&mut self, direction: Direction) {
        self.player_direction = direction;
        let destination = self.player_position.offset(direction);
        if self.cell(destination) == Cell::Wall {
            return;
        }

        self.restore_underlying_cell(self.player_position);
        self.player_position = destination;
        self.kill_rat_at(destination);
        self.grid.set_cell(self.player_position, Cell::Player);
    }

    fn move_rats(&mut self, player_moved: bool) {
        let mut order = [usize::MAX; MAX_RATS];
        let mut order_len = 0;

        for index in 0..MAX_RATS {
            if self.rats[index].alive {
                order[order_len] = index;
                order_len += 1;
            }
        }

        // Infestation resolves nearest rats first, then position order.
        for i in 0..order_len {
            let mut best = i;
            for candidate in i + 1..order_len {
                let candidate_rat = self.rats[order[candidate]];
                let best_rat = self.rats[order[best]];
                let candidate_key = (
                    candidate_rat
                        .position
                        .distance_squared(self.player_position),
                    candidate_rat.position.y,
                    candidate_rat.position.x,
                );
                let best_key = (
                    best_rat.position.distance_squared(self.player_position),
                    best_rat.position.y,
                    best_rat.position.x,
                );
                if candidate_key < best_key {
                    best = candidate;
                }
            }
            order.swap(i, best);
        }

        for &index in &order[..order_len] {
            if self.state == PlayState::Playing && self.rats[index].alive {
                self.move_rat(index, player_moved);
            }
        }
    }

    fn move_rat(&mut self, index: usize, player_moved: bool) {
        let rat_position = self.rats[index].position;
        let Some(face_direction) = Direction::toward(rat_position, self.player_position) else {
            self.state = PlayState::GameOver;
            return;
        };

        let mut directions = [None; 3];
        directions[0] = Some(face_direction);
        if face_direction.is_diagonal() {
            directions[1] = face_direction.x_only();
            directions[2] = face_direction.y_only();
        }

        let mut best_direction = None;
        let mut best_score = rat_position.distance_squared(self.player_position);
        let mut best_cost = 0;
        let mut best_player_still = true;
        let mut best_rank = u8::MAX;

        for direction in directions.into_iter().flatten() {
            let destination = rat_position.offset(direction);
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
            self.rats[index].direction = face_direction;
            return;
        };

        let destination = rat_position.offset(direction);
        self.restore_underlying_cell(rat_position);
        self.rats[index].position = destination;
        self.rats[index].direction = direction;

        if destination == self.player_position {
            self.state = PlayState::GameOver;
        }

        self.grid.set_cell(destination, Cell::Rat);
    }

    fn restore_underlying_cell(&mut self, position: Position) {
        let cell = if self.portals[..self.portal_count as usize]
            .iter()
            .any(|portal| portal.position == position)
        {
            Cell::Portal
        } else {
            Cell::Empty
        };
        self.grid.set_cell(position, cell);
    }

    fn kill_rat_at(&mut self, position: Position) {
        if let Some(rat) = self
            .rats
            .iter_mut()
            .find(|rat| rat.alive && rat.position == position)
        {
            rat.alive = false;
        }
    }

    fn rat_count(&self) -> u8 {
        self.rats.iter().filter(|rat| rat.alive).count() as u8
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
        game.rats = [Rat::EMPTY; MAX_RATS];
        game.rats[0] = Rat {
            position: Position::new(2, 1),
            direction: Direction::South,
            alive: true,
        };
        game.initial_rat_count = 1;
        game.state = PlayState::Playing;
        game.grid.set_cell(game.player_position, Cell::Player);
        game.grid.set_cell(game.rats[0].position, Cell::Rat);
        game
    }

    #[test]
    fn initial_level_matches_rats_map() {
        let game = Game::new(LevelId::Rats);
        assert_eq!(game.player_position, Position::new(3, 6));
        assert_eq!(game.rat_count(), 1);
        assert_eq!(game.rats[0].position, Position::new(3, 0));
        assert_eq!(game.grid.count(Cell::Wall), 21);
    }

    #[test]
    fn intro_has_three_active_rats() {
        let game = Game::new(LevelId::Intro);
        assert_eq!(game.rat_count(), 3);
    }

    #[test]
    fn more_rats_level_matches_original_layout() {
        let game = Game::new(LevelId::MoreRats);
        assert_eq!((game.width(), game.height()), (7, 7));
        assert_eq!(game.player_position, Position::new(3, 5));
        assert_eq!(game.rat_count(), 12);
        assert_eq!(game.grid.count(Cell::Wall), 12);
    }

    #[test]
    fn intro_rats_take_turns() {
        let mut game = Game::new(LevelId::Intro);
        game.act(None);

        assert!(game
            .rats
            .iter()
            .any(|rat| rat.alive && rat.position == Position::new(6, 10)));
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
        assert_eq!(game.portal_destination(), Some(LevelId::Rats));
        game.act(Some(Direction::South));

        assert_eq!(game.cell(portal), Cell::Portal);
    }

    #[test]
    fn player_can_kill_one_of_multiple_rats() {
        let mut game = encounter(Direction::South);
        game.rats[1] = Rat {
            position: Position::new(4, 4),
            direction: Direction::North,
            alive: true,
        };
        game.initial_rat_count = 2;
        game.grid.set_cell(game.rats[1].position, Cell::Rat);

        game.act(Some(Direction::North));

        assert_eq!(game.rat_count(), 1);
        assert_eq!(game.state, PlayState::Playing);
    }

    #[test]
    fn multiple_rats_move_sequentially_without_overlapping() {
        let mut game = Game::new(LevelId::Intro);
        game.grid.fill(Cell::Empty);
        game.player_position = Position::new(4, 4);
        game.rats = [Rat::EMPTY; MAX_RATS];
        game.rats[0] = Rat {
            position: Position::new(1, 1),
            direction: Direction::Southeast,
            alive: true,
        };
        game.rats[1] = Rat {
            position: Position::new(2, 1),
            direction: Direction::Southeast,
            alive: true,
        };
        game.initial_rat_count = 2;
        game.grid.set_cell(game.player_position, Cell::Player);
        game.grid.set_cell(game.rats[0].position, Cell::Rat);
        game.grid.set_cell(game.rats[1].position, Cell::Rat);

        game.act(None);

        assert_ne!(game.rats[0].position, game.rats[1].position);
        assert_ne!(game.rats[0].position, Position::new(1, 1));
        assert_ne!(game.rats[1].position, Position::new(2, 1));
    }

    #[test]
    fn sword_blocks_a_rat_attacking_from_the_front() {
        let mut game = encounter(Direction::North);
        game.act(None);
        assert_eq!(game.state, PlayState::Playing);
        assert_eq!(game.rats[0].position, Position::new(2, 1));
    }

    #[test]
    fn rat_attacking_from_the_side_causes_game_over() {
        let mut game = encounter(Direction::East);
        game.act(None);
        assert_eq!(game.state, PlayState::GameOver);
        assert_eq!(game.rats[0].position, Position::new(2, 2));
    }

    #[test]
    fn player_moving_into_a_rat_wins() {
        let mut game = encounter(Direction::South);
        game.act(Some(Direction::North));
        assert_eq!(game.state, PlayState::Won);
        assert_eq!(game.rat_count(), 0);
        assert_eq!(game.player_position, Position::new(2, 1));
    }
}
