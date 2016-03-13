mod network;

use network::Edge;
use network::Network;
use network::nodes;
use std::collections::HashMap;

fn main() {
    for network_provider in vec![designed_network] {
        network_provider(evaluate_network);
    }
}

fn evaluate_network(network: &Network) {
    let inputs = vec![
            (vec![0., 0.], vec![0.]),
            (vec![0., 1.], vec![1.]),
            (vec![1., 0.], vec![1.]),
            (vec![1., 1.], vec![0.]),
        ];

    for (input, target) in inputs {
        println!("Input  {:?}", input);
        println!("Target {:?}", target);

        let mut actual_input = input.clone();
        actual_input.push(1.);
        let (evaled, _) = network.eval(&actual_input, &HashMap::new());
        println!("Result {:?}", evaled);
    }
}

fn designed_network(callback: fn(&Network)) {
    let sensor_nodes = nodes(3);
    let hidden_nodes = nodes(3);
    let output_nodes = nodes(1);

    let sensor1 = &sensor_nodes[0];
    let sensor2 = &sensor_nodes[1];
    let bias = &sensor_nodes[2];

    let many_ones = &hidden_nodes[0];
    let any_ones = &hidden_nodes[1];

    let soft_max = &hidden_nodes[2];

    let output = &output_nodes[0];

    let edges = vec![
      Edge::new(&sensor1, &many_ones, 6.),
      Edge::new(&sensor2, &many_ones, 6.),
      Edge::new(&bias, &many_ones, -8.),

      Edge::new(&sensor2, &any_ones, 10.),
      Edge::new(&sensor1, &any_ones, 10.),
      Edge::new(&bias, &any_ones, -5.),

      Edge::new(&many_ones, &soft_max, -15.),
      Edge::new(&any_ones, &soft_max, 10.),
      Edge::new(&bias, &soft_max, -5.),

      Edge::new(&soft_max, &output, 1.),
    ];

    let network = Network::new(&sensor_nodes, &output_nodes, &edges);

    callback(&network);
}
