use std::{cmp::min, collections::BTreeSet};

use itertools::Itertools;
use rand::{prelude::IteratorRandom, rngs::StdRng, Rng, SeedableRng};

use std::mem::swap;

use rand::seq::SliceRandom;

pub fn stochastic_universal_selection(
    fitness_values: &Vec<f32>,
    num_parents: usize,
    seed: Option<u64>,
) -> Vec<usize> {
    let sum_of_fitness = fitness_values.iter().sum::<f32>();
    let mut fitness_scale: Vec<f32> = Vec::new();
    let mut back: f32 = 0.0;
    for (idx, &val) in fitness_values.iter().enumerate() {
        if idx == 0 {
            back = val;
            fitness_scale.push(back);
        } else {
            back = val + back;
            fitness_scale.push(back);
        }
    }

    let fitness_step = sum_of_fitness / num_parents as f32;

    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    let random_inital = prng.gen_range(0.0..fitness_step);

    let mut current_offset = 0usize;
    let mut selected_indices: Vec<usize> = Vec::new();
    for i in 0..num_parents {
        while fitness_scale[current_offset] < i as f32 * fitness_step + random_inital {
            current_offset += 1;
        }
        selected_indices.push(current_offset);
    }

    selected_indices
}

pub fn random_selection(
    population_size: usize,
    num_parents: usize,
    seed: Option<u64>,
) -> Vec<usize> {
    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    (0..population_size)
        .map(|x| x)
        .choose_multiple(&mut prng, min(num_parents, population_size))
}

pub fn linear_scaling(fitness_values: &mut Vec<f32>, scaling_factor: f32) {
    let minimum_fitness = fitness_values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let maximum_fitness = fitness_values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let average_fitness = fitness_values.iter().sum::<f32>() / (fitness_values.len() as f32);

    if average_fitness == 0.0 {
        for x in fitness_values {
            *x = 1.0;
        }
        return;
    }
    let mut a = (average_fitness * (scaling_factor - 1.0)) / (maximum_fitness - average_fitness);
    let mut b = (average_fitness * (maximum_fitness - scaling_factor * average_fitness))
        / (maximum_fitness - average_fitness);

    if *minimum_fitness <= -1.0 * b / a {
        a = average_fitness / (average_fitness - minimum_fitness);
        b = -1.0 * minimum_fitness * average_fitness / (average_fitness - minimum_fitness);
    }

    let linear_function = |x: f32| a * x + b;
    for x in fitness_values {
        *x = linear_function(*x);
    }
}

pub fn scramble_mutation<T>(individual: &mut Vec<T>, seed: Option<u64>) {
    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    let length_of_individual = individual.len();

    let mut idx1 = prng.gen_range(0..length_of_individual);
    let mut idx2 = prng.gen_range(0..length_of_individual);

    if idx2 < idx1 {
        swap(&mut idx1, &mut idx2);
    }

    let slice = &mut individual[idx1..=idx2];
    slice.shuffle(&mut prng);
}

pub fn order_crossover(
    parent1: &Vec<usize>,
    parent2: &Vec<usize>,
    seed: Option<u64>,
) -> (Vec<usize>, Vec<usize>) {
    let n = parent1.len();
    let mut prng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    let mut selected = (0..n).map(|x| x).choose_multiple(&mut prng, 2);
    selected.sort();

    let (mut child1, mut child2) = (parent1.clone(), parent2.clone());
    let (mut set1, mut set2) = (BTreeSet::new(), BTreeSet::new());

    for i in selected[0]..=selected[1] {
        set1.insert(child1[i]);
        set2.insert(child2[i]);
    }

    let mut current_idx = 0;
    for i in 0..n {
        if (selected[0]..=selected[1]).contains(&i) {
            continue;
        }

        while set1.contains(&parent2[current_idx]) {
            current_idx += 1;
        }

        if current_idx < n {
            set1.insert(parent2[current_idx]);
            child1[i] = parent2[current_idx];
        }
    }


    let mut current_idx = 0;
    for i in 0..n {
        if (selected[0]..=selected[1]).contains(&i) {
            continue;
        }
        while set2.contains(&parent1[current_idx]) {
            current_idx += 1;
        }

        if current_idx < n {
            set2.insert(parent1[current_idx]);
            child2[i] = parent1[current_idx];
        }
    }

    (child1, child2)
}

#[cfg(test)]
mod tests {
    use super::order_crossover;

    #[test]
    fn run_order() {
        let parent1 = vec![1, 3, 4, 6, 0, 2, 7, 5];
        let parent2 = vec![2, 3, 4, 0, 7, 6, 1, 5];
        let (child1, child2) = order_crossover(&parent1, &parent2, Some(42));

        dbg!(child1);
    }
}
