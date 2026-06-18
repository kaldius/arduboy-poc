use crate::direction::Direction;
use crate::grid::{Cell, Grid, MAX_CELL_COUNT};
use crate::position::Position;

const E: Cell = Cell::Empty;
const W: Cell = Cell::Wall;
const O: Cell = Cell::Portal;
const P: Cell = Cell::Player;
const R: Cell = Cell::Rat;
pub(crate) const MAX_RATS: usize = 4;

#[derive(Clone, Copy)]
pub(crate) struct RatSpawn {
    pub(crate) position: Position,
    pub(crate) direction: Direction,
}

impl RatSpawn {
    const EMPTY: Self = Self {
        position: Position::new(0, 0),
        direction: Direction::South,
    };

    const fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LevelId {
    Intro,
    Rats,
}

pub(crate) struct Level {
    pub(crate) id: LevelId,
    pub(crate) grid: Grid,
    pub(crate) player_position: Position,
    pub(crate) player_direction: Direction,
    pub(crate) rats: [RatSpawn; MAX_RATS],
    pub(crate) rat_count: u8,
    pub(crate) portal_position: Option<Position>,
}

pub(crate) const fn load(id: LevelId) -> Level {
    match id {
        LevelId::Intro => intro(),
        LevelId::Rats => rats(),
    }
}

const fn intro() -> Level {
    // Port of infestation/levels/intro.csv. The rats portal at (4, 9) is the
    // only active portal. Unsupported web and black-hole cells are walls.
    let source = [
        W, W, W, W, E, W, R, E, E, //
        W, E, E, E, E, W, E, E, E, //
        W, E, E, W, E, W, E, E, E, //
        W, E, E, W, E, W, E, E, E, //
        W, E, E, W, E, W, E, E, E, //
        W, E, E, W, E, W, E, E, E, //
        W, E, E, W, E, W, W, W, W, //
        E, E, E, W, E, E, E, E, E, //
        E, W, W, W, E, W, W, W, E, //
        E, W, R, W, O, W, R, W, E, //
        E, W, W, W, P, W, E, E, E, //
    ];

    Level {
        id: LevelId::Intro,
        grid: Grid::new(9, 11, pad_cells(source)),
        player_position: Position::new(4, 10),
        player_direction: Direction::North,
        rats: [
            RatSpawn::new(Position::new(6, 0), Direction::Southwest),
            RatSpawn::new(Position::new(2, 9), Direction::Southeast),
            RatSpawn::new(Position::new(6, 9), Direction::Southwest),
            RatSpawn::EMPTY,
        ],
        rat_count: 3,
        portal_position: Some(Position::new(4, 9)),
    }
}

const fn rats() -> Level {
    let source = [
        E, E, E, R, E, E, //
        E, W, W, W, W, E, //
        E, W, E, E, E, W, //
        W, E, W, W, W, E, //
        W, E, E, E, E, E, //
        W, E, W, W, W, W, //
        W, E, W, P, E, W, //
        W, E, E, E, E, W, //
    ];

    Level {
        id: LevelId::Rats,
        grid: Grid::new(6, 8, pad_cells(source)),
        player_position: Position::new(3, 6),
        player_direction: Direction::North,
        rats: [
            RatSpawn::new(Position::new(3, 0), Direction::South),
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
        ],
        rat_count: 1,
        portal_position: None,
    }
}

const fn pad_cells<const N: usize>(source: [Cell; N]) -> [Cell; MAX_CELL_COUNT] {
    let mut cells = [Cell::Empty; MAX_CELL_COUNT];
    let mut index = 0;
    while index < N {
        cells[index] = source[index];
        index += 1;
    }
    cells
}
