mod city;
mod evolutionary_operators;
mod fitness;

use crate::city::*;
use crate::evolutionary_operators::*;
use crate::fitness::*;

use rand::prelude::*;

fn main() {
    let city_count = 100;
    let population_size = 100;
    let generation_count = 20000;
    let mutation_probability = 0.1;

    let selection_odds = generate_selection_odds(population_size);

    let cities: Vec<City> = generate_cities(city_count);
    println!("cities:");
    for city in &cities {
        println!("{}", city);
    }

    let mut population = generate_initial_population(city_count, population_size);

    population.sort_unstable_by(|specimen1, specimen2| fitness(&cities, specimen1).partial_cmp(&fitness(&cities, specimen2)).unwrap());
    println!("sorted initial population:");
    for specimen in &population {
        println!("{:?}, fitness: {}", specimen, fitness(&cities, &specimen));
    }
    println!("best fitness: {}", best_fitness(&population, &cities));
    println!("average fitness: {}", average_fitness(&population, &cities));
    println!("==========================================================");

    for _i in 0..generation_count {
        population.sort_unstable_by(|specimen1, specimen2| fitness(&cities, specimen1).partial_cmp(&fitness(&cities, specimen2)).unwrap());
        let mut offsprings = Vec::new();
        for _j in 0..(population_size/4) {
            let (index1, index2) = selection(&selection_odds);
            let parent1 = &population[index1];
            let parent2 = &population[index2];
            let (offspring1, offspring2) = cycle_crossover(parent1, parent2);
            let (mutated_offspring1, mutated_offspring2) = (mutate(&offspring1, mutation_probability), mutate(&offspring2, mutation_probability));
            offsprings.push(mutated_offspring1);
            offsprings.push(mutated_offspring2);
        }
        let mut new_population = Vec::new();
        new_population.extend_from_slice(&population[0..10]);
        new_population.append(offsprings.as_mut());
        population = new_population;

        if _i % 1000 == 999 {
            println!("current progress: finished generation {} out of {}", _i+1, generation_count);
            println!("best fitness: {}", best_fitness(&population, &cities));
            println!("average fitness: {}", average_fitness(&population, &cities));
            println!("==========================================================");
        }
    }

    population.sort_unstable_by(|specimen1, specimen2| fitness(&cities, specimen1).partial_cmp(&fitness(&cities, specimen2)).unwrap());
    println!("sorted final population:");
    for specimen in &population {
        println!("{:?}, fitness: {}", specimen, fitness(&cities, &specimen));
    }
    println!("best fitness: {}", best_fitness(&population, &cities));
    println!("average fitness: {}", average_fitness(&population, &cities));
    println!("==========================================================");
}



fn generate_initial_population(city_count: usize, population_size: usize) -> Vec<Vec<usize>> {
    let mut population: Vec<Vec<usize>> = Vec::new();

    for _i in 0..population_size {
        let mut specimen: Vec<usize> = (0..city_count).collect();
        specimen.shuffle(&mut rand::thread_rng());
        population.push(specimen);
    }

    return population;
}