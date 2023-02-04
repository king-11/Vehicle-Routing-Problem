use std::f32::MAX;

use itertools::Itertools;

use super::genetic_algorithm::genetic_algorithm;

use super::model::*;

pub fn clustering(node: Node, routes: &mut Vec<Route> ,distance_matrix: &Vec<Vec<f32>>, time_matrix: &Vec<Vec<f32>>) {
	// most priority to distance travelled, add ratio too?
	let mut min_increase = MAX;
	let mut min_rider_index = 0;
	let mut optimal_rider = routes[0].clone();

  for (rider_index, route) in routes.iter().enumerate() {
    let mut temp_route = route.clone();

    // does it matter of GA to have input in a particular order?
    temp_route.nodes.push(node);

    let (cost, new_route) = genetic_algorithm(distance_matrix, time_matrix, &temp_route);

    let nodes = new_route.iter().map(|&idx| temp_route.nodes.iter().find(|node| node.index == idx).unwrap().clone()).collect_vec();
    temp_route.nodes = nodes;

    let increased_cost = cost - route.calc_distance(distance_matrix);
    if increased_cost < min_increase {
      min_increase = increased_cost;
      min_rider_index = rider_index;
      optimal_rider = temp_route;
    }
  }

  dbg!(min_rider_index);
  for (node_i, node) in optimal_rider.nodes.iter().enumerate() {
    print!("{} ",node.index);
  }
  println!("");

  routes[min_rider_index] = optimal_rider;
}
