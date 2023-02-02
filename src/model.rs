use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DeliveryType {
    Pickup,
    Delivery,
}

#[derive(Clone, Copy)]
pub struct Node {
    pub delivery_type: DeliveryType,
    pub index: usize,
}

#[derive(Clone)]
pub struct Route {
    pub nodes: Vec<Node>,
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
