use crate::direction::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Position {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

impl Position {
    pub(crate) const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub(crate) fn offset(self, direction: Direction) -> Self {
        let (dx, dy) = direction.delta();
        Self::new(self.x + dx, self.y + dy)
    }

    pub(crate) fn distance_squared(self, other: Self) -> i16 {
        let dx = i16::from(other.x - self.x);
        let dy = i16::from(other.y - self.y);
        dx * dx + dy * dy
    }
}
