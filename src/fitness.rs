use crate::city::City;

// helper function for calculating fitness
// calculates distance between two cities
fn distance(cities: &Vec<City>, index1: usize, index2: usize) -> f32{
    let city1 = cities.get(index1).unwrap();
    let city2 = cities.get(index2).unwrap();

    return ((city1.x - city2.x).powf(2.0) + (city1.y - city2.y).powf(2.0)).sqrt();
}

// calculates total distance of a route
// i.e. the sum of distances between pairs of cities adjacent on said route
pub fn fitness(cities: &Vec<City>, route: &Vec<usize>) -> f32 {
    let mut total_fitness = 0.0;

    for i in 0..(route.len() - 1) {
        total_fitness += distance(cities, route[i], route[i+1]);
    }
    total_fitness += distance(cities, route[route.len() - 1], route[0]);

    return total_fitness;
}

pub fn average_fitness(population: &Vec<Vec<usize>>, cities: &Vec<City>) -> f32 {
    let mut total_fitness = 0.0;

    for i in 0..population.len() {
        total_fitness += fitness(cities, &population[i]);
    }

    return total_fitness / population.len() as f32;
}

pub fn best_fitness(population: &Vec<Vec<usize>>, cities: &Vec<City>) -> f32 {
    let mut best_fitness = fitness(cities, &population[0]);

    for i in 1..population.len() {
        let fitness = fitness(cities, &population[i]);
        if fitness < best_fitness {
            best_fitness = fitness;
        }
    }

    return best_fitness;
}