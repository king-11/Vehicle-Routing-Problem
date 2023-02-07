use std::collections::BTreeMap;

use serde_derive::{Serialize, Deserialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use serde_wasm_bindgen::*;

use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum DeliveryType {
    Pickup,
    Delivery,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Node {
    pub delivery_type: DeliveryType,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Route {
    pub nodes: Vec<Node>,
}

#[wasm_bindgen]
pub fn send_routes_to_js(indexes: Vec<usize>, del_type: Vec<usize>) -> JsValue{
    let mut all_nodes : Vec<Node> = Vec::new();
    for i in 0..indexes.len() {
        // Delivery
        if del_type[i] == 1 {
            let temp_node = Node{
                delivery_type : DeliveryType::Delivery,
                index: indexes[i],
            };
            all_nodes.push(temp_node);
        }
        else {
            let temp_node = Node{
                delivery_type : DeliveryType::Pickup,
                index: indexes[i],
            };
            all_nodes.push(temp_node);
        }
    }
    let route = Route {
        nodes:all_nodes,
    };
    return serde_wasm_bindgen::to_value(&route).unwrap();
}

#[wasm_bindgen]
pub fn receive_routes_from_js(val: JsValue) {
    let routes: Route = serde_wasm_bindgen::from_value(val).unwrap();
}

impl Route {
    pub fn calc_distance(&self, distance_matrix: &Vec<Vec<f32>>) -> f32 {
        self.nodes
            .windows(2)
            .map(|vals| {
                if let [val1, val2] = vals {
                    distance_matrix[val1.index][val2.index]
                } else {
                    unreachable!()
                }
            })
            .sum()
    }

    pub fn calc_num_delivery(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| node.delivery_type == DeliveryType::Delivery)
            .count()
    }

    pub fn isFeasible(&self, item_size: &BTreeMap<usize, i32>, bag_size: i32) -> bool {
        let mut current_weight = self
            .nodes
            .iter()
            .filter_map(|node| {
                if node.delivery_type == DeliveryType::Delivery {
                    Some(item_size.get(&node.index).unwrap())
                } else {
                    None
                }
            })
            .sum::<i32>();

        if current_weight > bag_size {
            return false;
        };

        for node in self.nodes.iter() {
            let node_weight = *item_size.get(&node.index).unwrap();
            match node.delivery_type {
                DeliveryType::Delivery => {
                    current_weight -= node_weight;
                }
                DeliveryType::Pickup => {
                    if node_weight + current_weight > bag_size {
                        return false;
                    }

                    current_weight += node_weight
                }
            }
        }

        true
    }
}
