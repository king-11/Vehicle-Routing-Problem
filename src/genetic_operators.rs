use std::cmp::Ordering;

use std::cmp::min;

use rand::{prelude::IteratorRandom, rngs::StdRng, Rng, SeedableRng};

use std::mem::swap;

use rand::seq::SliceRandom;

use std::collections::HashMap;

// pub fn replace_with_position(vec: &mut Vec<usize>) {
//     let mut map = HashMap::new();
//     let mut map_num = HashMap::new();
//     let mut vec_sort: Vec<usize> = vec.to_vec();
//     vec_sort.sort();

//     for index in 0..vec.len() {
//         if !map.contains_key(&vec_sort[index]) {
//             map.insert(vec_sort[index], index);
//         }
//     }

//     for index in 0..vec.len() {
//         if !map_num.contains_key(&vec[index]) {
//             map_num.insert(vec[index], 0);
//         }
//         vec_sort[index] = map[&vec[index]] + map_num[&vec[index]];
//         *map_num.get_mut(&vec[index]).unwrap() += 1;
//     }

//     *vec = vec_sort;
// }



pub fn stochastic_universal_selection(
    fitness_values: &Vec<f32>,
    num_parents: usize,
    seed: Option<u64>,
) -> Vec<usize> {
    // dbg!(fitness_values);
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

    // dbg!(sum_of_fitness);

    let fitness_step = sum_of_fitness / num_parents as f32;

    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    // dbg!(fitness_step);

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
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();

    let maximum_fitness = fitness_values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
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

// pub fn partially_mapped_crossover(
//     parent1: &Vec<usize>,
//     parent2: &Vec<usize>,
//     seed: Option<u64>,
// ) -> (Vec<usize>, Vec<usize>) {
//     let n = parent1.len();
//     let mut prng = match seed {
//         Some(seed) => StdRng::seed_from_u64(seed),
//         None => StdRng::from_entropy(),
//     };

//     let mut selected = (0..n).map(|x| x).choose_multiple(&mut prng, 2);
//     selected.sort();
//     let (mut idx1, mut idx2) = (parent1.clone(), parent2.clone());
//     let (mut exp1, mut exp2) = (parent1.clone(), parent2.clone());
//     replace_with_position(&mut exp1);
//     replace_with_position(&mut exp2);

//     // dbg!(parent1.to_vec());
//     // dbg!(parent2.to_vec());
//     // dbg!(exp1.to_vec());
//     // dbg!(exp2.to_vec());

//     for i in 0..n {
//         idx1[exp1[i]] = i;
//         idx2[exp2[i]] = i;
//     }
//     let (mut child1, mut child2) = (parent1.clone(), parent2.clone());

//     for i in selected[0]..=selected[1] {
//         let (val1, val2) = (parent1[i], parent2[i]);
//         let (vald1, vald2) = (exp1[i], exp2[i]);

//         child1[i] = val2;
//         child1[idx1[vald2]] = val1;
//         child2[i] = val1;
//         child2[idx2[vald1]] = val2;

//         let temp = (idx1[vald1], idx1[vald2]);
//         idx1[vald1] = temp.1;
//         idx1[vald2] = temp.0;
//         let temp = (idx2[vald1], idx2[vald2]);
//         idx2[vald1] = temp.1;
//         idx2[vald2] = temp.0;
//     }
//     dbg!(child1.to_vec());
//     dbg!(child2.to_vec());
//     (child1, child2)
// }

pub fn partially_mapped_crossover(
    parent1: &Vec<usize>,
    parent2: &Vec<usize>,
    seed: Option<u64>,
) -> (Vec<usize>, Vec<usize>) {
    let len = parent1.len();
    let crossover_point = rand::thread_rng().gen_range(0..len);

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for i in 0..crossover_point {
        if !map1.contains_key(&parent1[i]) {
            map1.insert(parent1[i],0);
        }
        if !map2.contains_key(&parent2[i]) {
            map2.insert(parent2[i],0);
        }
    }

    let mut child1 = parent1[0..crossover_point].to_vec();

    
    for i in 0..parent2.len() {
        if !map1.contains_key(&parent2[i]) {
            child1.push(parent2[i]);
        }
    }

    let mut child2 = parent2[0..crossover_point].to_vec();
    // child2.extend_from_slice(&parent1[crossover_point..len]);

    for i in 0..parent1.len() {
        if !map2.contains_key(&parent1[i]) {
            child2.push(parent1[i]);
        }
    }

    (child1, child2)
}

