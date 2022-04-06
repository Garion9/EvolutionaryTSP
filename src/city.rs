use std::fmt;
use rand::prelude::*;

#[derive(Clone)]
pub struct City {
    pub x: f32,
    pub y: f32
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn generate_cities(city_count: usize) -> Vec<City> {
    let mut cities: Vec<City> = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..city_count {
        let city = City {
            x: rng.gen::<f32>() * 100.0,
            y: rng.gen::<f32>() * 100.0
        };
        cities.push(city);
    }
    return cities;
}