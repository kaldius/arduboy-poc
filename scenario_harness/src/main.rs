#![allow(dead_code)]

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "../../rust_game/src/direction.rs"]
mod direction;
#[path = "../../rust_game/src/game.rs"]
mod game;
#[path = "../../rust_game/src/grid.rs"]
mod grid;
#[path = "../../rust_game/src/level.rs"]
mod level;
#[path = "../../rust_game/src/position.rs"]
mod position;
mod scenario;

use scenario::{Action, Error, State};

#[derive(Default)]
struct Summary {
    passed: usize,
    failed: Vec<String>,
    unsupported: BTreeMap<String, usize>,
}

fn main() {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../infestation/scenario_tests"));
    let mut fixtures = Vec::new();
    collect_fixtures(&root.join("scenarios"), &mut fixtures);
    collect_fixtures(&root.join("two_player_scenarios"), &mut fixtures);
    fixtures.sort();

    let mut summary = Summary::default();
    for json_path in &fixtures {
        run_fixture(json_path, &mut summary);
    }

    let skipped = summary.unsupported.values().sum::<usize>();
    println!(
        "scenario validation: {}/{} scenarios supported",
        summary.passed + summary.failed.len(),
        fixtures.len()
    );
    println!(
        "  supported: {} passed, {} failed",
        summary.passed,
        summary.failed.len()
    );
    println!("  skipped:   {skipped} unsupported mechanics");
    for (reason, count) in &summary.unsupported {
        println!("    {count:>3}: {reason}");
    }
    for failure in &summary.failed {
        println!("  FAILED: {failure}");
    }

    if !summary.failed.is_empty() {
        std::process::exit(1);
    }
}

fn collect_fixtures(directory: &Path, fixtures: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with("_i.json"))
        {
            fixtures.push(path);
        }
    }
}

fn run_fixture(json_path: &Path, summary: &mut Summary) {
    let name = json_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap()
        .trim_end_matches("_i.json");
    let directory = json_path.parent().unwrap();
    let before_path = directory.join(format!("{name}_1.csv"));
    let after_path = directory.join(format!("{name}_2.csv"));

    let result = (|| {
        let json = fs::read_to_string(json_path)?;
        let before = fs::read_to_string(&before_path)?;
        let expected_csv = fs::read_to_string(&after_path)?;
        let action = parse_action(&json)?;
        let expected_state = parse_state(&json)?;
        let actual = scenario::run(&before, action).map_err(HarnessError::Core)?;

        let actual_csv = normalize_csv(&actual.csv);
        let expected_csv = normalize_csv(&expected_csv);
        if actual_csv != expected_csv || actual.state != expected_state {
            return Err(HarnessError::Mismatch(format!(
                "{name}: expected state {expected_state:?} and grid\n{expected_csv}\n\
                 got state {:?} and grid\n{actual_csv}",
                actual.state
            )));
        }
        Ok(())
    })();

    match result {
        Ok(()) => summary.passed += 1,
        Err(HarnessError::Core(Error::Unsupported(reason))) => {
            *summary.unsupported.entry(reason).or_default() += 1;
        }
        Err(HarnessError::Unsupported(reason)) => {
            *summary.unsupported.entry(reason).or_default() += 1;
        }
        Err(error) => summary.failed.push(error.to_string()),
    }
}

#[derive(Debug)]
enum HarnessError {
    Io(std::io::Error),
    Invalid(String),
    Unsupported(String),
    Core(Error),
    Mismatch(String),
}

impl std::fmt::Display for HarnessError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => error.fmt(formatter),
            Self::Invalid(message) | Self::Unsupported(message) | Self::Mismatch(message) => {
                formatter.write_str(message)
            }
            Self::Core(error) => error.fmt(formatter),
        }
    }
}

impl From<std::io::Error> for HarnessError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

fn parse_action(json: &str) -> Result<Action, HarnessError> {
    if json.contains("\"p1\"") || json.contains("\"p2\"") {
        return Err(HarnessError::Unsupported(
            "two-player actions are not supported".to_string(),
        ));
    }

    match json_string(json, "move")?.as_str() {
        "north" => Ok(Action::MoveNorth),
        "south" => Ok(Action::MoveSouth),
        "east" => Ok(Action::MoveEast),
        "west" => Ok(Action::MoveWest),
        "stall" => Ok(Action::Stall),
        action => Err(HarnessError::Invalid(format!(
            "unsupported action {action:?}"
        ))),
    }
}

fn parse_state(json: &str) -> Result<State, HarnessError> {
    match json_string(json, "state")?.as_str() {
        "playing" => Ok(State::Playing),
        "won" => Ok(State::Won),
        "gameover" => Ok(State::GameOver),
        state => Err(HarnessError::Invalid(format!(
            "unsupported state {state:?}"
        ))),
    }
}

fn json_string(json: &str, key: &str) -> Result<String, HarnessError> {
    let key = format!("\"{key}\"");
    let start = json
        .find(&key)
        .ok_or_else(|| HarnessError::Invalid(format!("missing {key}")))?;
    let value = &json[start + key.len()..];
    let quote = value
        .find('"')
        .ok_or_else(|| HarnessError::Invalid(format!("missing value for {key}")))?;
    let value = &value[quote + 1..];
    let end = value
        .find('"')
        .ok_or_else(|| HarnessError::Invalid(format!("unterminated value for {key}")))?;
    Ok(value[..end].to_string())
}

fn normalize_csv(csv: &str) -> String {
    csv.trim()
        .lines()
        .map(|line| line.split(',').map(str::trim).collect::<Vec<_>>().join(","))
        .collect::<Vec<_>>()
        .join("\n")
}
