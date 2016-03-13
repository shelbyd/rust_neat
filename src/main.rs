mod network;

use network::Edge;
use network::Network;
use network::nodes;
use std::collections::HashMap;

fn main() {
    let sensor_nodes = nodes(2);
    let hidden_nodes = nodes(1);
    let output_nodes = nodes(1);
    let edges = vec![
      Edge::new(&sensor_nodes[0], &hidden_nodes[0], 1.),
      Edge::new(&sensor_nodes[1], &hidden_nodes[0], 1.),
      Edge::new(&hidden_nodes[0], &output_nodes[0], 1.43),
    ];

    let network = Network::new(&sensor_nodes, &output_nodes, &edges);
    let inputs = &vec![1., 1.];
    let (evaled, activations) = network.eval(inputs, &HashMap::new());
    println!("{:?}", evaled);
    println!("{:?}", activations);
}
