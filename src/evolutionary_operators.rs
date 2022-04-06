use rand::prelude::*;

// crossover
pub fn cycle_crossover(parent1: &Vec<usize>, parent2: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut child1: Vec<usize> = parent1.clone();
    let mut child2: Vec<usize> = parent2.clone();

    let mut rng = rand::thread_rng();
    let length = parent1.len();

    let mut pos = rng.gen_range(0..length);
    let mut prev_pos = pos;

    loop {
        std::mem::swap(&mut child1[pos], &mut child2[pos]);
        prev_pos = pos;
        pos = find_next_occurrence(&child1, pos);
        if pos == prev_pos {
            break;
        }
    }

    return (child1, child2);
}

// helper function for crossover
fn find_next_occurrence(sequence: &Vec<usize>, index: usize) -> usize {
    let length = sequence.len();
    let mut i: usize = (index + 1) % length;
    loop {
        if sequence[i] == sequence[index] {
            return i;
        }
        i = (i + 1) % length;
    }
}

// mutation
pub fn mutate(genotype: &Vec<usize>, probability: f32) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut mutated_genotype = genotype.clone();

    if rng.gen::<f32>() <= probability {
        let length = genotype.len();
        let pos1 = rng.gen_range(0..length);
        let pos2 = rng.gen_range(0..length);
        mutated_genotype.swap(pos1, pos2);
    }

    return mutated_genotype;
}


// selection
pub fn selection(selection_odds: &Vec<usize>) -> (usize, usize){
    let mut rng = rand::thread_rng();

    let index1 = selection_odds[rng.gen_range(0..selection_odds.len())];
    let index2 = selection_odds[rng.gen_range(0..selection_odds.len())];

    return (index1, index2);
}

// helper function for selection
pub fn generate_selection_odds(population_size: usize) -> Vec<usize> {
    let mut selection_odds: Vec<usize> = Vec::new();

    for i in 0..(population_size/2) {
        let mut odds = vec![i; (population_size/2) - i];
        selection_odds.append(&mut odds);
    }

    return selection_odds;
}