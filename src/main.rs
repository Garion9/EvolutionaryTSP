mod city;
mod evolutionary_operators;
mod fitness;

use crate::city::*;
use crate::evolutionary_operators::*;
use crate::fitness::*;

use rand::prelude::*;
use plotters::prelude::*;

fn main() {
    let city_count: usize = 100;
    let population_size: usize = 100;
    let generation_count: i32 = 20000;
    let mutation_probability: f32 = 0.05;

    let mut best_fitnesses: Vec<f32> = Vec::new();
    let mut average_fitnesses: Vec<f32> = Vec::new();

    let plot_save_location: &str = "images/tsp(2).png";

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

        best_fitnesses.push(best_fitness(&population, &cities));
        average_fitnesses.push(average_fitness(&population, &cities));

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

    plot_evolution(best_fitnesses, average_fitnesses, plot_save_location);
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


fn plot_evolution(best_fitnesses: Vec<f32>, average_fitnesses: Vec<f32>, save_location: &str) {
    let root_area = BitMapBackend::new(save_location, (600, 400))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Travelling Salesman Problem (Evolutionary Algorithm)", ("sans-serif", 30))
        .build_cartesian_2d(0..(average_fitnesses.len()+10), (average_fitnesses.iter().copied().fold(f32::INFINITY, &f32::min) as usize - 10)..(average_fitnesses.iter().copied().fold(f32::NEG_INFINITY, &f32::max) as usize + 10))
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((0..(average_fitnesses.len())).map(|i:usize| (i, best_fitnesses[i] as usize)), &GREEN)
    ).unwrap();
    ctx.draw_series(
        LineSeries::new((0..(average_fitnesses.len())).map(|i:usize| (i, average_fitnesses[i] as usize)), &BLUE)
    ).unwrap();
}