use super::genetic_operators::*;

use super::model::*;

use std::{cmp::Ordering, f32::consts::E};

use rand::seq::SliceRandom;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

pub fn genetic_algorithm(
    distance_matrix: &Vec<Vec<f32>>,
    time_matrix: &Vec<Vec<f32>>,
    route: &Route,
) -> f32 {
    let population_size = 200;
    let iterations = 5000;
    let mutation_probability = 0.1;
    let between = Uniform::from(0.0..1.0);
    let mut prng = thread_rng();

    let distance = |path: &Vec<usize>| {
        path.windows(2)
            .map(|idxs| {
                if let &[idx1, idx2] = idxs {
                    distance_matrix[idx1][idx2]
                } else {
                    unreachable!()
                }
            })
            .sum::<f32>()
    };

    let time_taken = |path: &Vec<usize>| {
        path.windows(2)
            .map(|idxs| {
                if let &[idx1, idx2] = idxs {
                    time_matrix[idx1][idx2]
                } else {
                    unreachable!()
                }
            })
            .sum::<f32>()
    };

    let fitness_function = |path: &Vec<usize>| {
        let distance_travelleing = distance(path);
        let time_taken = time_taken(path);

        E.powf(-distance_travelleing) * (1.0 / 1.0 + time_taken)
    };

    let mut population = (0..population_size)
        .map(|_| {
            let mut path = route
                .nodes
                .iter()
                .map(|node| node.index)
                .collect::<Vec<usize>>();
            path.shuffle(&mut prng);
            path
        })
        .collect::<Vec<Vec<usize>>>();

    let get_fitness_values = |population: &Vec<Vec<usize>>| {
        population
            .iter()
            .map(|a| fitness_function(a))
            .collect::<Vec<f32>>()
    };

    let mut best_now = population[0].clone();

    for _ in 0..iterations {
        let idxs = random_selection(population_size, 2, None);
        let (mut child1, mut child2) =
            partially_mapped_crossover(&population[idxs[0]], &population[idxs[1]], None);

        if between.sample(&mut prng) < mutation_probability {
            scramble_mutation(&mut child1, None);
        }
        if between.sample(&mut prng) < mutation_probability {
            scramble_mutation(&mut child2, None);
        }

        population.push(child1);
        population.push(child2);

        let mut fitness_values = get_fitness_values(&population);
        linear_scaling(&mut fitness_values, 1.2);

        let selected_idx = stochastic_universal_selection(&fitness_values, population_size, None);

        population = selected_idx
            .iter()
            .map(|&a| population[a].clone())
            .collect::<Vec<Vec<usize>>>();

        let best = population
            .iter()
            .max_by(|a, b| {
                fitness_function(a)
                    .partial_cmp(&fitness_function(b))
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap();

        if fitness_function(best) > fitness_function(&best_now) {
            best_now = best.clone();
        }
    }
    //
    distance(&best_now)
}
