use std::fmt;
use std::string::{String, ToString};
use std::vec::Vec;

use crate::direction::Direction;
use crate::game::{Game, PlayState};
use crate::grid::{Cell, MAX_CELL_COUNT, MAX_HEIGHT, MAX_WIDTH};
use crate::level::MAX_RATS;
use crate::position::Position;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    Stall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Playing,
    Won,
    GameOver,
}

pub struct Outcome {
    pub csv: String,
    pub state: State,
}

#[derive(Debug)]
pub enum Error {
    Invalid(String),
    Unsupported(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) | Self::Unsupported(message) => formatter.write_str(message),
        }
    }
}

pub fn run(before: &str, action: Action) -> Result<Outcome, Error> {
    let rows: Vec<Vec<&str>> = before
        .trim()
        .lines()
        .map(|line| line.split(',').map(str::trim).collect())
        .collect();
    let height = rows.len();
    let width = rows.first().map(Vec::len).unwrap_or(0);

    if width == 0 || height == 0 {
        return Err(Error::Invalid("empty grid".to_string()));
    }
    if width > MAX_WIDTH || height > MAX_HEIGHT {
        return Err(Error::Unsupported(format!(
            "grid {width}x{height} exceeds {MAX_WIDTH}x{MAX_HEIGHT}"
        )));
    }
    if rows.iter().any(|row| row.len() != width) {
        return Err(Error::Invalid(
            "grid rows have different widths".to_string(),
        ));
    }

    let mut cells = [Cell::Empty; MAX_CELL_COUNT];
    let mut player = None;
    let mut rat_positions = [Position::new(0, 0); MAX_RATS];
    let mut rat_directions = [Direction::South; MAX_RATS];
    let mut rat_count = 0;

    for (y, row) in rows.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            let position = Position::new(x as i8, y as i8);
            let cell = match *symbol {
                "." => Cell::Empty,
                "#" => Cell::Wall,
                "w" => Cell::Web,
                "R" => {
                    if rat_count == MAX_RATS {
                        return Err(Error::Unsupported(format!("more than {MAX_RATS} rats")));
                    }
                    rat_positions[rat_count] = position;
                    rat_count += 1;
                    Cell::Rat
                }
                "▲" => player_cell(&mut player, position, Direction::North)?,
                "▼" => player_cell(&mut player, position, Direction::South)?,
                "►" => player_cell(&mut player, position, Direction::East)?,
                "◄" => player_cell(&mut player, position, Direction::West)?,
                unsupported => {
                    return Err(Error::Unsupported(format!(
                        "unsupported cell symbol {unsupported:?}"
                    )));
                }
            };
            cells[y * width + x] = cell;
        }
    }

    let (player_position, player_direction) =
        player.ok_or_else(|| Error::Unsupported("requires one player".to_string()))?;

    for index in 0..rat_count {
        rat_directions[index] =
            Direction::toward(rat_positions[index], player_position).unwrap_or(Direction::South);
    }

    let mut game = Game::from_scenario(
        width as u8,
        height as u8,
        cells,
        player_position,
        player_direction,
        rat_positions,
        rat_directions,
        rat_count as u8,
    );
    game.act(match action {
        Action::MoveNorth => Some(Direction::North),
        Action::MoveSouth => Some(Direction::South),
        Action::MoveEast => Some(Direction::East),
        Action::MoveWest => Some(Direction::West),
        Action::Stall => None,
    });

    Ok(Outcome {
        csv: to_csv(&game),
        state: match game.state() {
            PlayState::Playing => State::Playing,
            PlayState::Won => State::Won,
            PlayState::GameOver => State::GameOver,
        },
    })
}

fn player_cell(
    player: &mut Option<(Position, Direction)>,
    position: Position,
    direction: Direction,
) -> Result<Cell, Error> {
    if player.replace((position, direction)).is_some() {
        return Err(Error::Unsupported(
            "multiple players are not supported".to_string(),
        ));
    }
    Ok(Cell::Player)
}

fn to_csv(game: &Game) -> String {
    let mut csv = String::new();
    for y in 0..game.height() {
        for x in 0..game.width() {
            if x != 0 {
                csv.push(',');
            }
            let position = Position::new(x as i8, y as i8);
            csv.push_str(match game.cell(position) {
                Cell::Empty => ".",
                Cell::Wall => "#",
                Cell::Rat => "R",
                Cell::Web => "w",
                Cell::Player => match game.player_direction() {
                    Direction::North => "▲",
                    Direction::South => "▼",
                    Direction::East => "►",
                    Direction::West => "◄",
                    _ => unreachable!(),
                },
                Cell::Portal => unreachable!(),
            });
        }
        csv.push('\n');
    }
    csv
}
