use itertools::Itertools;
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};

use crate::model::*;

fn random_selection(routes: &Vec<Route>, k: usize, seed: Option<u64>) -> Vec<&Node> {
    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    routes
        .iter()
        .map(|route| &route.nodes)
        .flatten()
        .choose_multiple(&mut prng, k)
}

fn radial_selection(
    routes: &Vec<Route>,
    distance_matrix: Vec<Vec<f32>>,
    k: usize,
    seed: Option<u64>,
) -> Vec<&Node> {
    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    let nodes = routes
        .iter()
        .map(|route| &route.nodes)
        .flatten()
        .collect_vec();

    let selected_node = nodes
        .iter()
        .choose(&mut prng)
        .unwrap();

    nodes
        .iter()
        .filter_map(|&node| {
            if node.index == selected_node.index {
                None
            } else {
                Some((distance_matrix[node.index][selected_node.index], node))
            }
        })
        .sorted_by_key(|(dist, _)| dist.partial_cmp(&0.0).unwrap())
        .map(|(_, node)| node)
        .take(k)
        .collect_vec()
}

fn costly_selection(routes: &Vec<Route>, distance_matrix: Vec<Vec<f32>>, k: usize) -> Vec<&Node> {
    let mut costs = vec![];
    let nodes = routes
        .iter()
        .map(|route| &route.nodes)
        .flatten()
        .collect_vec();

    for (idx, &node) in nodes.iter().enumerate() {
        if idx == 0 {
            continue;
        }

        let mut distance = distance_matrix[nodes[idx - 1].index][node.index];
        let mut new_distance = 0.0;
        if idx + 1 < nodes.len() {
            distance += distance_matrix[node.index][nodes[idx + 1].index];
            new_distance += distance_matrix[nodes[idx - 1].index][nodes[idx + 1].index];
        }
        costs.push((new_distance - distance, node.index));
    }

    costs
        .iter()
        .sorted_by_key(|(dist, _)| dist.partial_cmp(&0.0).unwrap())
        .map(|(_, node_index)| {
            routes
                .iter()
                .map(|route| &route.nodes)
                .flatten()
                .find(|node| node.index == *node_index)
                .unwrap()
        })
        .take(k)
        .collect_vec()
}
