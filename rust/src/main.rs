use std::env;
use std::io::{self};
use std::usize;

pub mod domain;
pub mod solutions;

fn main() -> io::Result<()> {
    let (path, nb_workers) = parse_parameters()?;

    if nb_workers == 0 {
        solutions::baseline::solve(&path)?;
    } else {
        if env::var("RAYON").is_ok() {
            println!("RAYON");
            solutions::ray::solve(&path, nb_workers)
        } else {
            solutions::threaded::solve(&path, nb_workers);
        }
    }
    Ok(())
}

fn parse_parameters() -> io::Result<(String, usize)> {
    let path = std::env::args().nth(1).expect("no path given");
    let nb_workers: usize = std::env::args()
        .nth(2)
        .unwrap_or("0".into())
        .parse()
        .expect("expected a positive number");
    Ok((path, nb_workers))
}

