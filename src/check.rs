use super::clustering::clustering;
use super::model::*;

fn check()->usize {
    let v1 = vec![0.0,1.0,1.0,1.0];
    let v2 = vec![0.1,0.0,1.0,5.0]; 
    let v3 = vec![2.0,0.1,0.0,1.0];
    let v4 = vec![0.1,2.0,3.0,0.0];
    let mut v : Vec<Vec<f32>> = Vec::new();
    v.push(v1);
    v.push(v2);
    v.push(v3);
    v.push(v4);
    
    let mut node1 = Node {
        delivery_type: DeliveryType::Pickup,
        index: 0,
    };
    let mut node2 = Node {
        delivery_type: DeliveryType::Delivery,
        index: 1,
    };
    let mut node3 = Node {
        delivery_type: DeliveryType::Delivery,
        index: 2,
    };
    let mut node4 = Node {
        delivery_type: DeliveryType::Delivery,
        index: 3,
    };

    let mut routes: Vec<Route> = Vec::new();
    // let mut rider1 = Route {
    //     nodes: vec![node1,node1],
    // };
    // let mut rider2 = Route {
    //     nodes: vec![node1, node1],
    // };
    let mut rider1 = Route {
        nodes: vec![node1],
    };
    let mut rider2 = Route {
        nodes: vec![node1],
    };
    routes.push(rider1);
    routes.push(rider2);
    // clustering(node1,&mut routes,&v,&v);
    clustering(node2,&mut routes,&v,&v);
    clustering(node3,&mut routes,&v,&v);
    clustering(node4,&mut routes,&v,&v);

    for (rider_index, route) in routes.iter().enumerate() {
        for (node_index, loc) in route.nodes.iter().enumerate() {
            print!("{}",loc.index);
            print!(" ");
        }
        println!("");
    }
    return 64;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run1() {
        check();
    }
}
