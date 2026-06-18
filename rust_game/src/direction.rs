use crate::position::Position;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
    East,
    West,
    North,
    South,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl Direction {
    pub(crate) const fn delta(self) -> (i8, i8) {
        match self {
            Self::East => (1, 0),
            Self::West => (-1, 0),
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::Northeast => (1, -1),
            Self::Northwest => (-1, -1),
            Self::Southeast => (1, 1),
            Self::Southwest => (-1, 1),
        }
    }

    pub(crate) const fn opposite(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::West => Self::East,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::Northeast => Self::Southwest,
            Self::Northwest => Self::Southeast,
            Self::Southeast => Self::Northwest,
            Self::Southwest => Self::Northeast,
        }
    }

    pub(crate) const fn is_diagonal(self) -> bool {
        matches!(
            self,
            Self::Northeast | Self::Northwest | Self::Southeast | Self::Southwest
        )
    }

    pub(crate) const fn movement_cost(self) -> i16 {
        if self.is_diagonal() {
            2
        } else {
            1
        }
    }

    pub(crate) const fn tie_break_rank(self) -> u8 {
        self as u8
    }

    pub(crate) fn toward(from: Position, to: Position) -> Option<Self> {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        match (dx.signum(), dy.signum()) {
            (1, 0) => Some(Self::East),
            (-1, 0) => Some(Self::West),
            (0, -1) => Some(Self::North),
            (0, 1) => Some(Self::South),
            (1, -1) => Some(Self::Northeast),
            (-1, -1) => Some(Self::Northwest),
            (1, 1) => Some(Self::Southeast),
            (-1, 1) => Some(Self::Southwest),
            _ => None,
        }
    }

    pub(crate) const fn x_only(self) -> Option<Self> {
        match self {
            Self::East | Self::Northeast | Self::Southeast => Some(Self::East),
            Self::West | Self::Northwest | Self::Southwest => Some(Self::West),
            Self::North | Self::South => None,
        }
    }

    pub(crate) const fn y_only(self) -> Option<Self> {
        match self {
            Self::North | Self::Northeast | Self::Northwest => Some(Self::North),
            Self::South | Self::Southeast | Self::Southwest => Some(Self::South),
            Self::East | Self::West => None,
        }
    }
}
