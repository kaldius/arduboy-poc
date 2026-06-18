use crate::position::Position;

pub(crate) const WIDTH: usize = 6;
pub(crate) const HEIGHT: usize = 8;
const CELL_COUNT: usize = WIDTH * HEIGHT;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Cell {
    Empty,
    Wall,
    Player,
    Rat,
}

pub(crate) struct Grid {
    cells: [Cell; CELL_COUNT],
}

impl Grid {
    pub(crate) const fn new(cells: [Cell; CELL_COUNT]) -> Self {
        Self { cells }
    }

    pub(crate) fn cell(&self, position: Position) -> Cell {
        Self::index(position)
            .map(|index| self.cells[index])
            .unwrap_or(Cell::Wall)
    }

    pub(crate) fn set_cell(&mut self, position: Position, cell: Cell) {
        if let Some(index) = Self::index(position) {
            self.cells[index] = cell;
        }
    }

    fn index(position: Position) -> Option<usize> {
        if position.x < 0
            || position.x >= WIDTH as i8
            || position.y < 0
            || position.y >= HEIGHT as i8
        {
            None
        } else {
            Some(position.y as usize * WIDTH + position.x as usize)
        }
    }

    #[cfg(test)]
    pub(crate) fn fill(&mut self, cell: Cell) {
        self.cells.fill(cell);
    }

    #[cfg(test)]
    pub(crate) fn count(&self, cell: Cell) -> usize {
        self.cells
            .iter()
            .filter(|&&candidate| candidate == cell)
            .count()
    }
}
