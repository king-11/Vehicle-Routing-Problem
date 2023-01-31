use std::f32::MAX;

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

    // pass by reference here, temp_route will have best arrangement.

    genetic_algorithm(distance_matrix, time_matrix, &temp_route);

    let increased_cost = temp_route.calc_distance(distance_matrix) - route.calc_distance(distance_matrix);
    if increased_cost < min_increase {
      min_increase = increased_cost;
      min_rider_index = rider_index;
      optimal_rider = temp_route;
    }
  }

  routes[min_rider_index] = optimal_rider;
}
