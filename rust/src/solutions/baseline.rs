use super::*;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

pub fn solve(path: &str) -> Result<()> {
    let lines = get_file_lines(path)?;
    let stations = build_stations(lines);
    sort_and_print_stations(stations);
    Ok(())
}

type FileRows = Box<dyn Iterator<Item = String>>;
fn get_file_lines(path: &str) -> Result<FileRows> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: FileRows = Box::new(reader.lines().map(|x| x.unwrap()));
    Ok(lines)
}
fn build_stations(lines: FileRows) -> Stations {
    let mut stations = HashMap::new();
    for line in lines {
        if let Some(obs) = parse_observation(&line) {
            update_stations(&mut stations, obs);
        }
    }
    stations
}
