use std::{
    fs::File,
    io::{BufRead, BufReader},
};
extern crate rayon;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use super::*;
pub fn solve(path: &str, nb_workers: usize) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    ThreadPoolBuilder::new()
        .num_threads(nb_workers)
        .build_global()
        .unwrap();

    let stations = reader
        .lines()
        .par_bridge()
        .map(|line| parse_observation(&line.unwrap()).unwrap())
        .fold(
            || HashMap::new(),
            |mut s: Stations, o: Observation| {
                update_stations(&mut s, o);
                s
            },
        )
        .reduce(
            || HashMap::new(),
            |mut m1, m2| {
                merge_stations(&mut m1, m2);
                m1
            },
        );
    sort_and_print_stations(stations);
}
pub fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let chunk_size = 3;
    let chunks: Vec<_> = data.par_chunks(chunk_size).collect();

    for chunk in chunks {
        println!("{:?}", chunk);
    }
}
