use crate::direction::Direction;
use crate::grid::{Cell, Grid, MAX_CELL_COUNT};
use crate::position::Position;

const E: Cell = Cell::Empty;
const W: Cell = Cell::Wall;
const O: Cell = Cell::Portal;
const P: Cell = Cell::Player;
const R: Cell = Cell::Rat;
const S: Cell = Cell::Web;
pub(crate) const MAX_RATS: usize = 13;
pub(crate) const MAX_PORTALS: usize = 5;

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
    MoreRats,
    TrappedRat,
    TrappedRat2,
    Webs,
}

#[derive(Clone, Copy)]
pub(crate) struct Portal {
    pub(crate) position: Position,
    pub(crate) destination: LevelId,
}

impl Portal {
    const EMPTY: Self = Self {
        position: Position::new(0, 0),
        destination: LevelId::Intro,
    };

    const fn new(position: Position, destination: LevelId) -> Self {
        Self {
            position,
            destination,
        }
    }
}

pub(crate) struct Level {
    pub(crate) id: LevelId,
    pub(crate) grid: Grid,
    pub(crate) player_position: Position,
    pub(crate) player_direction: Direction,
    pub(crate) rats: [RatSpawn; MAX_RATS],
    pub(crate) rat_count: u8,
    pub(crate) portals: [Portal; MAX_PORTALS],
    pub(crate) portal_count: u8,
}

pub(crate) const fn load(id: LevelId) -> Level {
    match id {
        LevelId::Intro => intro(),
        LevelId::Rats => rats(),
        LevelId::MoreRats => more_rats(),
        LevelId::TrappedRat => trapped_rat(),
        LevelId::TrappedRat2 => trapped_rat_2(),
        LevelId::Webs => webs(),
    }
}

const fn intro() -> Level {
    // Port of infestation/levels/intro.csv. Unsupported black-hole cells are
    // walls until that mechanic is implemented.
    let source = [
        W, W, W, W, E, W, R, E, E, //
        W, E, E, E, E, W, E, E, E, //
        W, E, E, W, O, W, E, E, E, //
        W, E, E, W, E, W, E, E, E, //
        W, E, E, W, E, W, E, E, E, //
        W, E, E, W, E, W, E, E, E, //
        W, E, E, W, E, W, W, W, W, //
        E, E, E, W, O, O, O, E, E, //
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
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
        ],
        rat_count: 3,
        portals: [
            Portal::new(Position::new(4, 9), LevelId::Rats),
            Portal::new(Position::new(4, 7), LevelId::MoreRats),
            Portal::new(Position::new(5, 7), LevelId::TrappedRat),
            Portal::new(Position::new(6, 7), LevelId::TrappedRat2),
            Portal::new(Position::new(4, 5), LevelId::Webs),
        ],
        portal_count: 5,
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
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
        ],
        rat_count: 1,
        portals: [Portal::EMPTY; MAX_PORTALS],
        portal_count: 0,
    }
}

const fn more_rats() -> Level {
    let source = [
        E, E, E, E, E, W, R, //
        R, W, E, E, E, W, R, //
        R, W, E, E, E, W, R, //
        R, W, E, E, E, W, R, //
        R, W, E, E, E, W, R, //
        R, W, E, P, E, W, R, //
        R, W, E, E, E, E, E, //
    ];

    Level {
        id: LevelId::MoreRats,
        grid: Grid::new(7, 7, pad_cells(source)),
        player_position: Position::new(3, 5),
        player_direction: Direction::North,
        rats: [
            RatSpawn::new(Position::new(6, 0), Direction::Southwest),
            RatSpawn::new(Position::new(0, 1), Direction::Southeast),
            RatSpawn::new(Position::new(6, 1), Direction::Southwest),
            RatSpawn::new(Position::new(0, 2), Direction::Southeast),
            RatSpawn::new(Position::new(6, 2), Direction::Southwest),
            RatSpawn::new(Position::new(0, 3), Direction::Southeast),
            RatSpawn::new(Position::new(6, 3), Direction::Southwest),
            RatSpawn::new(Position::new(0, 4), Direction::Southeast),
            RatSpawn::new(Position::new(6, 4), Direction::Southwest),
            RatSpawn::new(Position::new(0, 5), Direction::East),
            RatSpawn::new(Position::new(6, 5), Direction::West),
            RatSpawn::new(Position::new(0, 6), Direction::Northeast),
            RatSpawn::EMPTY,
        ],
        rat_count: 12,
        portals: [Portal::EMPTY; MAX_PORTALS],
        portal_count: 0,
    }
}

const fn trapped_rat() -> Level {
    let source = [
        E, E, E, E, R, E, W, E, E, E, //
        E, E, E, E, W, E, W, E, E, E, //
        W, W, W, W, E, W, E, W, E, E, //
        E, W, W, E, E, W, E, E, W, E, //
        E, W, W, E, E, E, W, W, W, E, //
        E, W, W, W, W, E, E, W, W, E, //
        E, E, E, E, W, W, W, W, W, R, //
        R, W, E, E, E, E, E, E, W, R, //
        R, W, E, E, E, E, E, E, W, R, //
        R, W, E, E, P, E, E, E, W, R, //
        R, W, E, E, E, E, E, E, W, R, //
        R, W, E, E, E, E, E, E, W, R, //
        R, W, E, E, E, E, E, E, E, E, //
    ];

    trapped_rat_level(LevelId::TrappedRat, source)
}

const fn trapped_rat_2() -> Level {
    let source = [
        E, E, W, E, R, E, W, W, E, E, //
        E, W, E, W, W, E, W, E, W, E, //
        E, W, W, E, E, W, W, W, E, E, //
        E, W, W, E, E, W, W, E, W, E, //
        E, W, E, E, W, W, E, W, W, E, //
        E, W, W, W, W, E, E, E, W, E, //
        E, E, E, E, W, W, W, W, W, R, //
        R, W, E, E, E, E, E, E, W, R, //
        R, W, E, E, E, E, E, E, W, R, //
        R, W, E, E, P, E, E, E, W, R, //
        R, W, E, E, E, E, E, E, W, R, //
        R, W, E, E, E, E, E, E, W, R, //
        R, W, E, W, W, E, E, E, E, E, //
    ];

    trapped_rat_level(LevelId::TrappedRat2, source)
}

const fn trapped_rat_level(id: LevelId, source: [Cell; 10 * 13]) -> Level {
    Level {
        id,
        grid: Grid::new(10, 13, pad_cells(source)),
        player_position: Position::new(4, 9),
        player_direction: Direction::North,
        rats: [
            RatSpawn::new(Position::new(4, 0), Direction::South),
            RatSpawn::new(Position::new(9, 6), Direction::Southwest),
            RatSpawn::new(Position::new(0, 7), Direction::Southeast),
            RatSpawn::new(Position::new(9, 7), Direction::Southwest),
            RatSpawn::new(Position::new(0, 8), Direction::Southeast),
            RatSpawn::new(Position::new(9, 8), Direction::Southwest),
            RatSpawn::new(Position::new(0, 9), Direction::East),
            RatSpawn::new(Position::new(9, 9), Direction::West),
            RatSpawn::new(Position::new(0, 10), Direction::Northeast),
            RatSpawn::new(Position::new(9, 10), Direction::Northwest),
            RatSpawn::new(Position::new(0, 11), Direction::Northeast),
            RatSpawn::new(Position::new(9, 11), Direction::Northwest),
            RatSpawn::new(Position::new(0, 12), Direction::Northeast),
        ],
        rat_count: 13,
        portals: [Portal::EMPTY; MAX_PORTALS],
        portal_count: 0,
    }
}

const fn webs() -> Level {
    let source = [
        S, E, R, S, S, S, S, S, S, S, S, S, //
        S, E, E, S, S, W, S, S, S, S, S, S, //
        S, W, W, S, S, W, S, S, R, R, S, S, //
        S, W, W, S, S, W, S, S, R, R, S, S, //
        R, S, S, S, S, W, S, S, R, S, S, S, //
        S, W, W, W, W, W, S, S, R, R, S, S, //
        S, E, P, S, S, W, S, S, S, S, S, S, //
        W, W, W, W, W, W, S, S, S, S, S, S, //
    ];

    Level {
        id: LevelId::Webs,
        grid: Grid::new(12, 8, pad_cells(source)),
        player_position: Position::new(2, 6),
        player_direction: Direction::North,
        rats: [
            RatSpawn::new(Position::new(2, 0), Direction::South),
            RatSpawn::new(Position::new(8, 2), Direction::Southwest),
            RatSpawn::new(Position::new(9, 2), Direction::Southwest),
            RatSpawn::new(Position::new(8, 3), Direction::Southwest),
            RatSpawn::new(Position::new(9, 3), Direction::Southwest),
            RatSpawn::new(Position::new(0, 4), Direction::Southeast),
            RatSpawn::new(Position::new(8, 4), Direction::Southwest),
            RatSpawn::new(Position::new(8, 5), Direction::Northwest),
            RatSpawn::new(Position::new(9, 5), Direction::Northwest),
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
            RatSpawn::EMPTY,
        ],
        rat_count: 9,
        portals: [Portal::EMPTY; MAX_PORTALS],
        portal_count: 0,
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
