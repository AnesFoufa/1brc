use super::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub fn solve(lines: Box<dyn Iterator<Item = String>>, nb_workers: usize) {
    let stations = build_stations(lines, nb_workers);
    sort_and_print_stations(stations);
}

fn build_stations(lines: Box<dyn Iterator<Item = String>>, nb_workers: usize) -> Stations {
    let threads = spawn_threads(nb_workers);
    let mut stations;
    let thread_ids = (0..nb_workers).cycle();
    for (line, thread_id) in lines.zip(thread_ids) {
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
    stations
}

fn work(receiver: Receiver<String>) -> Stations {
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

fn spawn_threads(nb_workers: usize) -> Vec<(Sender<String>, JoinHandle<Stations>)> {
    let mut threads = Vec::with_capacity(nb_workers);
    for _ in 0..nb_workers {
        let (tx, rx) = channel();
        let handle = thread::spawn(move || work(rx));
        threads.push((tx, handle))
    }
    threads
}
