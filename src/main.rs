mod network;

use network::Network;

use network::genome::Genome;
use network::genome::Gene;

use std::collections::HashMap;

fn main() {
    evaluate_network(&designed_network());
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

        let (evaled, _) = network.eval_with_bias(&input, &HashMap::new());
        println!("Result {:?}", evaled);
    }
}

fn designed_network() -> Network {
    let many_ones = 4;
    let any_ones = 5;
    let softmax = 6;

    let genes = vec![
        Gene::new(0, many_ones, 6., 0),
        Gene::new(1, many_ones, 6., 1),
        Gene::new(2, many_ones, -8., 2),

        Gene::new(0, any_ones, 10., 3),
        Gene::new(1, any_ones, 10., 4),
        Gene::new(2, any_ones, -5., 5),

        Gene::new(many_ones, softmax, -15., 6),
        Gene::new(any_ones, softmax, 10., 7),
        Gene::new(2, softmax, -5., 8),

        Gene::new(softmax, 3, 1., 9),
    ];
    Genome::new(2, 1, genes).to_network()
}
