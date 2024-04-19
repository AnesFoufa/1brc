use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Observation {
    pub city: String,
    pub temperature: f32,
}

pub fn parse_observation(line: &str) -> Option<Observation> {
    let (part1, part2) = line.trim().split_once(";")?;
    let city: String = part1.into();
    let temperature: f32 = part2.parse().ok()?;
    Some(Observation { city, temperature })
}

#[derive(Debug, Clone)]
pub struct Temperatures {
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
    pub fn update(&mut self, temperature: f32) {
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
    pub fn merge(&mut self, other: &Temperatures) {
        self.min = if self.min > other.min {
            other.min
        } else {
            self.min
        };
        self.max = if self.max < other.max {
            other.max
        } else {
            self.max
        };
        self.mean = (self.mean * (self.nb as f32) + other.mean * (other.nb as f32))
            / ((self.nb + other.nb) as f32);
        self.nb = self.nb + other.nb;
    }
}

pub type Stations = HashMap<String, Temperatures>;

pub fn update_stations(stations: &mut Stations, observation: Observation) {
    let temperatures = stations
        .entry(observation.city)
        .or_insert(Temperatures::default());
    temperatures.update(observation.temperature);
}

pub fn merge_stations(first_stations: &mut Stations, stations: Stations) {
    for (city, temperatures) in stations.into_iter() {
        match first_stations.entry(city.clone()) {
            Entry::Occupied(mut temperatures_) => temperatures_.get_mut().merge(&temperatures),
            _ => {
                first_stations.insert(city, temperatures);
            }
        }
    }
}
