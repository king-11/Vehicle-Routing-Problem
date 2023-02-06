use itertools::Itertools;

use crate::{model::Route, node_selection::radial_selection, clustering::clustering};

pub fn route_mutate(routes: &mut Vec<Route>, distance_matrix: &Vec<Vec<f32>>, time_matrix: &Vec<Vec<f32>>, rider_matrix: &Vec<Vec<f32>>, nodes_mut: usize) {
  let selected_nodes = radial_selection(&routes, distance_matrix, nodes_mut, Some(42)).iter().map(|&node| node.clone()).collect_vec();

  for route in routes.iter_mut() {
    for node in &selected_nodes {
      if let Some((idx, _)) = route.nodes.iter().find_position(|node_rem| node_rem.index == node.index) {
        route.nodes.remove(idx);
      }
    }
  }

  for node in selected_nodes {
    clustering(node, routes, distance_matrix, time_matrix, rider_matrix);
  }
}
