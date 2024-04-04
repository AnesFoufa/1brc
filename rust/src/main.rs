use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::usize;

#[derive(Debug)]
struct Observation {
    city: String,
    temperature: f32,
}

fn parse_observation(line: &str) -> Option<Observation> {
    let (part1, part2) = line.split_once(";")?;
    let city: String = part1.into();
    let temperature: f32 = part2.parse().ok()?;
    Some(Observation { city, temperature })
}

#[derive(Debug)]
struct Temperatures {
    min: f32,
    max: f32,
    mean: f32,
    nb: i64,
}
impl Default for Temperatures {
    fn default() -> Self {
        Temperatures {
            min: f32::MAX,
            max: f32::MIN,
            mean: 0.0,
            nb: 0,
        }
    }
}

impl Temperatures {
    fn update(&mut self, temperature: f32) {
        self.min = if self.min > temperature {
            temperature
        } else {
            self.min
        };
        self.max = if self.max < temperature {
            temperature
        } else {
            self.max
        };
        self.mean = (self.mean * (self.nb as f32) + temperature) / ((self.nb + 1) as f32);
        self.nb = self.nb + 1;
    }
    fn merge(&mut self, other: &Temperatures) {
        self.min = if self.min > other.min {
            other.min
        } else {
            self.min
        };
        self.max = if self.max > other.max {
            other.max
        } else {
            self.max
        };
        self.mean = (self.mean * (self.nb as f32) + other.mean * (other.nb as f32))
            / ((self.nb + other.nb) as f32);
        self.nb = self.nb + other.nb;
    }
}
type Stations = HashMap<String, Temperatures>;

fn work(receiver: Receiver<String>, _thread_id: usize) -> Stations {
    let mut stations = HashMap::new();
    loop {
        let obs = match receiver.recv() {
            Ok(line) => parse_observation(&line).unwrap(),
            _ => break,
        };
        update_stations(&mut stations, obs);
    }
    stations
}

fn update_stations(stations: &mut Stations, observation: Observation) {
    let temperatures = stations
        .entry(observation.city)
        .or_insert(Temperatures::default());
    temperatures.update(observation.temperature);
}

fn main() -> io::Result<()> {
    let (path, nb_workers, rows) = parse_parameters()?;
    let lines = get_file_lines(&path, rows)?;
    let threads = spawn_thrads(nb_workers);

    let mut stations;
    if nb_workers > 0 {
        let thread_ids = (0..nb_workers).cycle();
        for (maybe_line, thread_id) in lines.zip(thread_ids) {
            let line = maybe_line?;
            threads[thread_id].0.send(line).unwrap();
        }
        let mut threads_iter = threads.into_iter();
        let (tx, handle) = threads_iter.next().unwrap();
        drop(tx);
        stations = handle.join().unwrap();

        for (tx, handle) in threads_iter {
            drop(tx);
            let following_stations = handle.join().unwrap();
            merge_stations(&mut stations, following_stations);
        }
    } else {
        stations = HashMap::new();
        for line in lines {
            let obs = parse_observation(&line?).unwrap();
            update_stations(&mut stations, obs)
        }
    }

    let mut stations_vec: Vec<(String, Temperatures)> = stations.into_iter().collect();
    stations_vec.sort_by(|a, b| a.0.cmp(&b.0));

    for (city, temperatures) in stations_vec.into_iter() {
        println!("{:?} {:?}", city, temperatures)
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

fn spawn_thrads(nb_workers: usize) -> Vec<(Sender<String>, JoinHandle<Stations>)> {
    let mut threads = Vec::with_capacity(nb_workers);
    for tid in 0..nb_workers {
        let (tx, rx) = channel();
        let handle = thread::spawn(move || work(rx, tid));
        threads.push((tx, handle))
    }
    threads
}

fn get_file_lines(
    path: &str,
    rows: Option<usize>,
) -> io::Result<Box<dyn Iterator<Item = io::Result<String>>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines: Box<dyn Iterator<Item = _>> = Box::new(reader.lines());
    if let Some(rs) = rows {
        lines = Box::new(lines.take(rs))
    }
    Ok(lines)
}

fn merge_stations(first_stations: &mut Stations, stations: Stations) {
    for (city, temperatures) in stations.into_iter() {
        match first_stations.entry(city.clone()) {
            Entry::Occupied(mut temperatures_) => temperatures_.get_mut().merge(&temperatures),
            _ => {
                first_stations.insert(city, temperatures);
            }
        }
    }
}
