use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::usize;
pub mod domain;
pub mod solutions;

fn main() -> io::Result<()> {
    let (path, nb_workers, rows) = parse_parameters()?;
    let lines = get_file_lines(&path, rows)?;

    if nb_workers == 0 {
        solutions::baseline::solve(lines);
    } else {
        if env::var("RAYON").is_ok() {
            println!("RAYON");
            solutions::ray::solve(&path, nb_workers)
        } else {
            solutions::threaded::solve(lines, nb_workers);
        }
    }
    Ok(())
}

fn parse_parameters() -> io::Result<(String, usize, Option<usize>)> {
    let path = std::env::args().nth(1).expect("no path given");
    let nb_workers: usize = std::env::args()
        .nth(2)
        .unwrap_or("0".into())
        .parse()
        .expect("expected a positive number");
    let rows = std::env::args().nth(3).and_then(|x| x.parse().ok());
    Ok((path, nb_workers, rows))
}

fn get_file_lines(path: &str, rows: Option<usize>) -> io::Result<Box<dyn Iterator<Item = String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines: Box<dyn Iterator<Item = _>> = Box::new(reader.lines().map(|x| x.unwrap()));
    if let Some(rs) = rows {
        lines = Box::new(lines.take(rs))
    }
    Ok(lines)
}
