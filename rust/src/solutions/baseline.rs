use super::*;

pub fn solve(lines: Box<dyn Iterator<Item = String>>) {
    let stations = build_stations(lines);
    sort_and_print_stations(stations);
}

fn build_stations(lines: Box<dyn Iterator<Item = String>>) -> Stations {
    let mut stations = HashMap::new();
    for line in lines {
        let obs = parse_observation(&line).unwrap();
        update_stations(&mut stations, obs)
    }
    stations
}
