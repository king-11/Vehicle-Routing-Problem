use itertools::Itertools;

use crate::{clustering::clustering, model::Route, node_selection::radial_selection};

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub fn route_mutate(
    routes: &mut Vec<Route>,
    distance_matrix: &Vec<Vec<f32>>,
    time_matrix: &Vec<Vec<f32>>,
    rider_matrix: &Vec<Vec<f32>>,
    nodes_mut: usize,
) {
    let selected_nodes = radial_selection(&routes, distance_matrix, nodes_mut, Some(42))
        .iter()
        .map(|&node| node.clone())
        .collect_vec();

    for route in routes.iter_mut() {
        for node in &selected_nodes {
            if let Some((idx, _)) = route
                .nodes
                .iter()
                .find_position(|node_rem| node_rem.index == node.index)
            {
                route.nodes.remove(idx);
            }
        }
    }

    for node in selected_nodes {
        clustering(node, routes, distance_matrix, time_matrix, rider_matrix);
    }
}

pub fn mutate_aux(
    iterations: usize,
    routes: &mut Vec<Route>,
    distance_matrix: &Vec<Vec<f32>>,
    time_matrix: &Vec<Vec<f32>>,
    rider_matrix: &Vec<Vec<f32>>,
    nodes_mut: usize,
) {
    let init_cost = routes
        .iter()
        .map(|route| route.calc_distance(distance_matrix))
        .collect_vec();

    let best_cost = init_cost;

    let mut mutated_routes = routes.clone();

    for _ in 0..iterations {
        route_mutate(
            &mut mutated_routes,
            distance_matrix,
            time_matrix,
            rider_matrix,
            nodes_mut,
        );
        let new_cost = mutated_routes
            .iter()
            .map(|route| route.calc_distance(distance_matrix))
            .collect_vec();
        if new_cost < best_cost {
            *routes = mutated_routes.clone();
        }
    }
}

#[wasm_bindgen]
pub fn invoke_mutation_from_js(
    iterations: usize,
    routes: JsValue,
    distance_matrix: JsValue,
    time_matrix: JsValue,
    rider_matrix: JsValue,
    nodes_mut: usize,
) -> JsValue {
    let mut routes: Vec<Route> = serde_wasm_bindgen::from_value(routes).unwrap();
    let distance_matrix: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(distance_matrix).unwrap();
    let time_matrix: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(time_matrix).unwrap();
    let rider_matrix: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(rider_matrix).unwrap();
    mutate_aux(
        iterations,
        &mut routes,
        &distance_matrix,
        &time_matrix,
        &rider_matrix,
        nodes_mut,
    );
    return serde_wasm_bindgen::to_value(&routes).unwrap();
}
