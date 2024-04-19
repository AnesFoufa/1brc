use clap::{Parser, Subcommand};
use solutions::{baseline, plrs, ray, threaded};
use std::io::{self};
use std::usize;

pub mod domain;
pub mod solutions;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    path: String,
}

#[derive(Subcommand)]
enum Commands {
    Rayon { nb_threads: usize },
    Polars,
    Multithreaded { nb_threads: usize },
    Sequential,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Sequential => baseline::solve(&cli.path),
        Commands::Polars => Ok(plrs::solve(&cli.path)),
        Commands::Multithreaded { nb_threads } => Ok(threaded::solve(&cli.path, nb_threads)),
        Commands::Rayon { nb_threads } => Ok(ray::solve(&cli.path, nb_threads)),
    }
}
