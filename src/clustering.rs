use std::f32::MAX;

use itertools::Itertools;

use super::genetic_algorithm::genetic_algorithm;

use super::model::*;

use serde_wasm_bindgen::*;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn invoke_clustering_from_js(
    routes: JsValue,
    node: JsValue,
    distance_matrix: JsValue,
    time_matrix: JsValue,
    rider_matrix: JsValue,
) -> JsValue {
    let mut routes: Vec<Route> = serde_wasm_bindgen::from_value(routes).unwrap();
    let node: Node = serde_wasm_bindgen::from_value(node).unwrap();
    let distance_matrix: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(distance_matrix).unwrap();
    let time_matrix: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(time_matrix).unwrap();
    let rider_matrix: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(rider_matrix).unwrap();
    clustering(
        node,
        &mut routes,
        &distance_matrix,
        &time_matrix,
        &rider_matrix,
    );
    return serde_wasm_bindgen::to_value(&routes).unwrap();
}

pub fn clustering(
    node: Node,
    routes: &mut Vec<Route>,
    distance_matrix: &Vec<Vec<f32>>,
    time_matrix: &Vec<Vec<f32>>,
    rider_matrix: &Vec<Vec<f32>>,
) {
    // most priority to distance travelled, add ratio too?
    let mut min_increase = MAX;
    let mut min_rider_index = 0;
    let mut optimal_rider = routes[0].clone();

    for (rider_index, route) in routes.iter().enumerate() {
        let mut temp_route = route.clone();

        // does it matter of GA to have input in a particular order?
        temp_route.nodes.push(node);
        let (cost, new_route) = genetic_algorithm(
            distance_matrix,
            time_matrix,
            rider_matrix,
            rider_index,
            &temp_route,
        );

        let nodes = new_route
            .iter()
            .map(|&idx| {
                temp_route
                    .nodes
                    .iter()
                    .find(|node| node.index == idx)
                    .unwrap()
                    .clone()
            })
            .collect_vec();
        temp_route.nodes = nodes;

        let increased_cost = cost - route.calc_distance(distance_matrix);
        if increased_cost < min_increase {
            min_increase = increased_cost;
            min_rider_index = rider_index;
            optimal_rider = temp_route;
        }
    }

    routes[min_rider_index] = optimal_rider;
}
