use super::*;
use std::fs::{metadata, File};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::thread::spawn;
use std::{u64, usize};

pub fn solve(path: &str, nb_workers: usize) {
    let file_size = metadata(path).unwrap().len();
    let chunks = get_chunks(file_size, nb_workers);
    let handles = chunks.into_iter().map(|(start, end)| {
        let owned_path = path.into();
        spawn(move || process_chunk(owned_path, start, end))
    });
    let mut stations = HashMap::new();
    for handle in handles {
       merge_stations(&mut stations, handle.join().unwrap())
    }
    sort_and_print_stations(stations);
}

fn process_chunk(path: String, start: u64, end: u64) -> Stations {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    let ascii_line_sep = '\n'.to_ascii_lowercase() as u8;
    let mut read_bytes = 0;
    let mut stations = HashMap::new();

    if start > 0 {
        let mut t_buf = [0; 1];
        reader.seek(SeekFrom::Start(start - 1)).unwrap();
        reader.read_exact(&mut t_buf).unwrap();
        if t_buf[0] != ascii_line_sep {
            let nb: u64 = reader.read_until(ascii_line_sep ,&mut buf).unwrap() as u64;
            read_bytes = read_bytes + nb;
        }
        else {
            reader.seek(SeekFrom::Start(start)).unwrap();
        }
    }
    while read_bytes < end - start {
        buf.clear();
        let nb = reader.read_until(ascii_line_sep, &mut buf).unwrap() as u64;
        read_bytes = read_bytes + nb;
        let line = std::str::from_utf8(&buf).unwrap();
        if let Some(observation) = parse_observation(line){
            update_stations(&mut stations, observation)
        }
    }
    stations
}

fn get_chunks(file_size: u64, nb_workers: usize) -> Vec<(u64, u64)> {
    let mut res = Vec::new();
    let chunk_size = file_size / (nb_workers as u64);
    for i in (0..(file_size + 1)).step_by(chunk_size as usize) {
        let mut end = i + chunk_size;
        if end > file_size {
            end = file_size
        }
        res.push((i, end))
    }
    res
}

