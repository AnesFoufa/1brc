use crate::domain::*;
use std::collections::HashMap;

pub mod baseline;
pub mod threaded;
pub mod ray;

fn sort_and_print_stations(stations: Stations) {
    let mut stations_vec: Vec<_> = stations.into_iter().collect();
    stations_vec.sort_by(|a, b| a.0.cmp(&b.0));

    for (city, temperatures) in stations_vec.into_iter() {
        println!("{:?} {:?}", city, temperatures)
    }
}


pub type FileRows = Box<dyn Iterator<Item = String>>;
