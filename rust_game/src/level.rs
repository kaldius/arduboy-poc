use crate::direction::Direction;
use crate::grid::{Cell, Grid, HEIGHT, WIDTH};
use crate::position::Position;

const E: Cell = Cell::Empty;
const W: Cell = Cell::Wall;
const P: Cell = Cell::Player;
const R: Cell = Cell::Rat;

// Port of infestation/levels/rats.csv.
const CELLS: [Cell; WIDTH * HEIGHT] = [
    E, E, E, R, E, E, //
    E, W, W, W, W, E, //
    E, W, E, E, E, W, //
    W, E, W, W, W, E, //
    W, E, E, E, E, E, //
    W, E, W, W, W, W, //
    W, E, W, P, E, W, //
    W, E, E, E, E, W, //
];

pub(crate) const PLAYER_POSITION: Position = Position::new(3, 6);
pub(crate) const PLAYER_DIRECTION: Direction = Direction::North;
pub(crate) const RAT_POSITION: Position = Position::new(3, 0);
pub(crate) const RAT_DIRECTION: Direction = Direction::South;

pub(crate) const fn grid() -> Grid {
    Grid::new(CELLS)
}
