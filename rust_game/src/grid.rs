use crate::position::Position;

pub(crate) const MAX_WIDTH: usize = 10;
pub(crate) const MAX_HEIGHT: usize = 13;
pub(crate) const MAX_CELL_COUNT: usize = MAX_WIDTH * MAX_HEIGHT;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Cell {
    Empty,
    Wall,
    Portal,
    Player,
    Rat,
}

pub(crate) struct Grid {
    cells: [Cell; MAX_CELL_COUNT],
    width: u8,
    height: u8,
}

impl Grid {
    pub(crate) const fn new(width: u8, height: u8, cells: [Cell; MAX_CELL_COUNT]) -> Self {
        Self {
            cells,
            width,
            height,
        }
    }

    pub(crate) const fn width(&self) -> u8 {
        self.width
    }

    pub(crate) const fn height(&self) -> u8 {
        self.height
    }

    pub(crate) fn cell(&self, position: Position) -> Cell {
        self.index(position)
            .map(|index| self.cells[index])
            .unwrap_or(Cell::Wall)
    }

    pub(crate) fn set_cell(&mut self, position: Position, cell: Cell) {
        if let Some(index) = self.index(position) {
            self.cells[index] = cell;
        }
    }

    fn index(&self, position: Position) -> Option<usize> {
        if position.x < 0
            || position.x >= self.width as i8
            || position.y < 0
            || position.y >= self.height as i8
        {
            None
        } else {
            Some(position.y as usize * self.width as usize + position.x as usize)
        }
    }

    #[cfg(test)]
    pub(crate) fn fill(&mut self, cell: Cell) {
        self.cells.fill(cell);
    }

    #[cfg(test)]
    pub(crate) fn count(&self, cell: Cell) -> usize {
        self.cells[..self.width as usize * self.height as usize]
            .iter()
            .filter(|&&candidate| candidate == cell)
            .count()
    }
}
