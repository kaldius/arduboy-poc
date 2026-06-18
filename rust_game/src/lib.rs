#![cfg_attr(not(test), no_std)]

use core::cell::UnsafeCell;
#[cfg(not(test))]
use core::panic::PanicInfo;

const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 64;
const FRAMEBUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT / 8;

const GRID_WIDTH: usize = 6;
const GRID_HEIGHT: usize = 8;
const CELL_COUNT: usize = GRID_WIDTH * GRID_HEIGHT;
const TILE_SIZE: i8 = 8;
const GRID_X: i8 = 40;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Player,
    Rat,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    East,
    West,
    North,
    South,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlayState {
    Playing,
    Won,
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    x: i8,
    y: i8,
}

impl Position {
    const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    fn offset(self, direction: Direction) -> Self {
        let (dx, dy) = direction.delta();
        Self::new(self.x + dx, self.y + dy)
    }

    fn distance_squared(self, other: Self) -> i16 {
        let dx = i16::from(other.x - self.x);
        let dy = i16::from(other.y - self.y);
        dx * dx + dy * dy
    }
}

impl Direction {
    const fn delta(self) -> (i8, i8) {
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

    const fn opposite(self) -> Self {
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

    const fn is_diagonal(self) -> bool {
        matches!(
            self,
            Self::Northeast | Self::Northwest | Self::Southeast | Self::Southwest
        )
    }

    const fn movement_cost(self) -> i16 {
        if self.is_diagonal() { 2 } else { 1 }
    }

    const fn tie_break_rank(self) -> u8 {
        self as u8
    }

    fn toward(from: Position, to: Position) -> Option<Self> {
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

    const fn x_only(self) -> Option<Self> {
        match self {
            Self::East | Self::Northeast | Self::Southeast => Some(Self::East),
            Self::West | Self::Northwest | Self::Southwest => Some(Self::West),
            Self::North | Self::South => None,
        }
    }

    const fn y_only(self) -> Option<Self> {
        match self {
            Self::North | Self::Northeast | Self::Northwest => Some(Self::North),
            Self::South | Self::Southeast | Self::Southwest => Some(Self::South),
            Self::East | Self::West => None,
        }
    }
}

const E: Cell = Cell::Empty;
const W: Cell = Cell::Wall;
const P: Cell = Cell::Player;
const R: Cell = Cell::Rat;

// Port of infestation/levels/rats.csv.
const INITIAL_GRID: [Cell; CELL_COUNT] = [
    E, E, E, R, E, E, //
    E, W, W, W, W, E, //
    E, W, E, E, E, W, //
    W, E, W, W, W, E, //
    W, E, E, E, E, E, //
    W, E, W, W, W, W, //
    W, E, W, P, E, W, //
    W, E, E, E, E, W, //
];

struct Game {
    grid: [Cell; CELL_COUNT],
    player_position: Position,
    player_direction: Direction,
    rat_position: Option<Position>,
    rat_direction: Direction,
    state: PlayState,
}

impl Game {
    const fn new() -> Self {
        Self {
            grid: INITIAL_GRID,
            player_position: Position::new(3, 6),
            player_direction: Direction::North,
            rat_position: Some(Position::new(3, 0)),
            rat_direction: Direction::South,
            state: PlayState::Playing,
        }
    }

    fn restart(&mut self) {
        *self = Self::new();
    }

    fn index(position: Position) -> Option<usize> {
        if position.x < 0
            || position.x >= GRID_WIDTH as i8
            || position.y < 0
            || position.y >= GRID_HEIGHT as i8
        {
            None
        } else {
            Some(position.y as usize * GRID_WIDTH + position.x as usize)
        }
    }

    fn cell(&self, position: Position) -> Cell {
        Self::index(position)
            .map(|index| self.grid[index])
            .unwrap_or(Cell::Wall)
    }

    fn set_cell(&mut self, position: Position, cell: Cell) {
        if let Some(index) = Self::index(position) {
            self.grid[index] = cell;
        }
    }

    fn act(&mut self, movement: Option<Direction>) {
        if self.state != PlayState::Playing {
            return;
        }

        let player_moved = movement.is_some();
        if let Some(direction) = movement {
            self.player_direction = direction;
            let destination = self.player_position.offset(direction);
            if self.cell(destination) != Cell::Wall {
                self.set_cell(self.player_position, Cell::Empty);
                self.player_position = destination;

                if self.rat_position == Some(destination) {
                    self.rat_position = None;
                }

                self.set_cell(self.player_position, Cell::Player);
            }
        }

        if self.rat_position.is_none() {
            self.state = PlayState::Won;
            return;
        }

        self.move_rat(player_moved);
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

            if destination.x == self.player_position.x
                && destination.y == self.player_position.y
                && direction == self.player_direction.opposite()
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
        self.set_cell(rat, Cell::Empty);
        self.rat_position = Some(destination);
        self.rat_direction = direction;

        if destination.x == self.player_position.x && destination.y == self.player_position.y {
            self.state = PlayState::GameOver;
        }

        self.set_cell(destination, Cell::Rat);
    }

    fn render(&self, framebuffer: &mut [u8]) {
        framebuffer.fill(0);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let position = Position::new(x as i8, y as i8);
                let screen_x = GRID_X + position.x * TILE_SIZE;
                let screen_y = position.y * TILE_SIZE;

                match self.cell(position) {
                    Cell::Empty => {}
                    Cell::Wall => draw_wall(framebuffer, screen_x, screen_y),
                    Cell::Player => {
                        draw_player(framebuffer, screen_x, screen_y, self.player_direction)
                    }
                    Cell::Rat => draw_rat(framebuffer, screen_x, screen_y, self.rat_direction),
                }
            }
        }

        match self.state {
            PlayState::Playing => {}
            PlayState::Won => draw_status(framebuffer, true),
            PlayState::GameOver => draw_status(framebuffer, false),
        }
    }
}

struct GlobalGame(UnsafeCell<Game>);

// The Arduino loop is the only execution context that accesses the game.
unsafe impl Sync for GlobalGame {}

static GAME: GlobalGame = GlobalGame(UnsafeCell::new(Game::new()));

#[unsafe(no_mangle)]
pub extern "C" fn infestation_init() {
    unsafe {
        (&mut *GAME.0.get()).restart();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_up() {
    unsafe {
        (&mut *GAME.0.get()).act(Some(Direction::North));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_down() {
    unsafe {
        (&mut *GAME.0.get()).act(Some(Direction::South));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_left() {
    unsafe {
        (&mut *GAME.0.get()).act(Some(Direction::West));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_right() {
    unsafe {
        (&mut *GAME.0.get()).act(Some(Direction::East));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_a() {
    unsafe {
        (&mut *GAME.0.get()).act(None);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn infestation_press_b() {
    unsafe {
        (&mut *GAME.0.get()).restart();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn infestation_render(framebuffer: *mut u8, length: u16) {
    if framebuffer.is_null() || usize::from(length) < FRAMEBUFFER_SIZE {
        return;
    }

    let framebuffer =
        unsafe { core::slice::from_raw_parts_mut(framebuffer, FRAMEBUFFER_SIZE) };
    unsafe {
        (&*GAME.0.get()).render(framebuffer);
    }
}

fn set_pixel(framebuffer: &mut [u8], x: i8, y: i8) {
    if x < 0 || y < 0 {
        return;
    }

    let x = x as usize;
    let y = y as usize;
    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
        return;
    }
    framebuffer[(y / 8) * SCREEN_WIDTH + x] |= 1 << (y & 7);
}

fn fill_rect(framebuffer: &mut [u8], x: i8, y: i8, width: i8, height: i8) {
    for py in y..y + height {
        for px in x..x + width {
            set_pixel(framebuffer, px, py);
        }
    }
}

fn draw_wall(framebuffer: &mut [u8], x: i8, y: i8) {
    fill_rect(framebuffer, x, y, TILE_SIZE, TILE_SIZE);
    for offset in [1, 5] {
        for px in x..x + TILE_SIZE {
            let index = ((y + offset) as usize / 8) * SCREEN_WIDTH + px as usize;
            framebuffer[index] &= !(1 << ((y + offset) & 7));
        }
    }
}

fn draw_player(framebuffer: &mut [u8], x: i8, y: i8, direction: Direction) {
    fill_rect(framebuffer, x + 2, y + 2, 4, 4);
    let (dx, dy) = direction.delta();
    set_pixel(framebuffer, x + 4 + dx * 3, y + 4 + dy * 3);
    set_pixel(framebuffer, x + 4 + dx * 2, y + 4 + dy * 2);
}

fn draw_rat(framebuffer: &mut [u8], x: i8, y: i8, direction: Direction) {
    fill_rect(framebuffer, x + 2, y + 3, 4, 3);
    set_pixel(framebuffer, x + 2, y + 2);
    set_pixel(framebuffer, x + 5, y + 2);
    let (dx, dy) = direction.delta();
    set_pixel(framebuffer, x + 4 + dx * 2, y + 4 + dy * 2);
}

fn draw_status(framebuffer: &mut [u8], won: bool) {
    let x = if won { 4 } else { 116 };
    fill_rect(framebuffer, x, 26, 8, 12);
    if won {
        fill_rect(framebuffer, x + 2, 28, 4, 6);
    } else {
        for i in 0..6 {
            set_pixel(framebuffer, x + 1 + i, 28 + i);
            set_pixel(framebuffer, x + 6 - i, 28 + i);
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encounter(player_direction: Direction) -> Game {
        let mut game = Game::new();
        game.grid.fill(Cell::Empty);
        game.player_position = Position::new(2, 2);
        game.player_direction = player_direction;
        game.rat_position = Some(Position::new(2, 1));
        game.rat_direction = Direction::South;
        game.state = PlayState::Playing;
        game.set_cell(game.player_position, Cell::Player);
        game.set_cell(game.rat_position.unwrap(), Cell::Rat);
        game
    }

    #[test]
    fn initial_level_matches_rats_map() {
        let game = Game::new();
        assert_eq!(game.player_position, Position::new(3, 6));
        assert_eq!(game.rat_position, Some(Position::new(3, 0)));
        assert_eq!(
            game.grid.iter().filter(|&&cell| cell == Cell::Wall).count(),
            21
        );
    }

    #[test]
    fn blocked_move_changes_facing_but_not_position() {
        let mut game = Game::new();
        game.act(Some(Direction::North));
        assert_eq!(game.player_position, Position::new(3, 6));
        assert_eq!(game.player_direction, Direction::North);
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

    #[test]
    fn render_writes_a_single_arduboy_framebuffer() {
        let game = Game::new();
        let mut framebuffer = [0; FRAMEBUFFER_SIZE];
        game.render(&mut framebuffer);
        assert!(framebuffer.iter().any(|&byte| byte != 0));
    }
}
