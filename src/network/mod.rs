extern crate rand;

use std::f64::consts::E;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Network {
    sensors: Vec<Node>,
    outputs: Vec<Node>,
    edges: Vec<Edge>,
}

impl Network {
    pub fn new(sensors: Vec<Node>, outputs: Vec<Node>, edges: Vec<Edge>) -> Network {
        Network {
            sensors: sensors,
            outputs: outputs,
            edges: edges,
        }
    }

    pub fn eval(&self,
                inputs: &Vec<f64>,
                previous_activations: &HashMap<Node, f64>)
                -> (Vec<f64>, HashMap<Node, f64>) {
        let mut currently_calculating = HashSet::new();
        let mut these_activations = HashMap::new();
        let outputs = self.outputs
                          .iter()
                          .map(|output| {
                              self.get_value(output,
                                             inputs,
                                             previous_activations,
                                             &mut these_activations,
                                             &mut currently_calculating)
                          })
                          .collect();
        (outputs, these_activations)
    }

    fn get_value(&self,
                 node: &Node,
                 inputs: &Vec<f64>,
                 previous_activations: &HashMap<Node, f64>,
                 these_activations: &mut HashMap<Node, f64>,
                 currently_calculating: &mut HashSet<Node>)
                 -> f64 {

        if currently_calculating.contains(node) {
            return *previous_activations.get(node).unwrap_or(&0.);
        }

        if self.sensors.contains(node) {
            return inputs[self.sensors.iter().position(|n| n == node).unwrap()];
        }

        currently_calculating.insert(*node);
        let total_input = self.edges
                              .iter()
                              .filter(|edge| &edge.destination == node)
                              .map(|edge| {
                                  edge.weight *
                                  self.get_value(&edge.source,
                                                 inputs,
                                                 previous_activations,
                                                 these_activations,
                                                 currently_calculating)
                              })
                              .fold(0., |acc, weight| acc + weight);
        currently_calculating.remove(node);

        match self.outputs.contains(node) {
            true => total_input,
            false => {
                let result = sigmoid(total_input);
                these_activations.insert(*node, result);
                result
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Node(usize);

impl Node {
    fn new() -> Node {
        Node(rand::random::<usize>())
    }
}

pub struct Edge {
    source: Node,
    destination: Node,
    weight: f64,
}

impl Edge {
    pub fn new(source: Node, destination: Node, weight: f64) -> Edge {
        Edge {
            source: source,
            destination: destination,
            weight: weight,
        }
    }
}

pub fn nodes(count: usize) -> Vec<Node> {
    (0..).take(count).map(|_| Node::new()).collect()
}

fn sigmoid(value: f64) -> f64 {
    1. / (1. + E.powf(-value))
}

#[cfg(test)]
mod test {
    use super::nodes;
    use super::Edge;
    use super::Network;
    use std::collections::HashMap;

    #[test]
    fn test_simple_network() {
        let sensors = nodes(1);
        let outputs = nodes(1);
        let edges = vec![Edge::new(sensors[0], outputs[0], 1.)];
        let network = Network::new(sensors, outputs, edges);
        let previous_activations = &HashMap::new();

        let (evaled, _) = network.eval(&vec![1.], previous_activations);

        assert_eq!(evaled[0], 1.);
    }

    #[test]
    fn test_weight_of_two() {
        let sensors = nodes(1);
        let outputs = nodes(1);
        let edges = vec![Edge::new(sensors[0], outputs[0], 2.)];
        let network = Network::new(sensors, outputs, edges);
        let previous_activations = &HashMap::new();

        let (evaled, _) = network.eval(&vec![1.], previous_activations);

        assert_eq!(evaled[0], 2.);
    }

    #[test]
    fn test_hidden_node() {
        let sensors = nodes(1);
        let hiddens = nodes(1);
        let outputs = nodes(1);
        let edges = vec![
            Edge::new(sensors[0], hiddens[0], 1.),
            Edge::new(hiddens[0], outputs[0], 1.),
        ];
        let network = Network::new(sensors, outputs, edges);
        let previous_activations = &HashMap::new();

        let (evaled, _) = network.eval(&vec![1.], previous_activations);

        assert!((evaled[0] - 0.73105).abs() < 0.00001);
    }

    #[test]
    fn test_recurrence() {
        let sensors = nodes(1);
        let hiddens = nodes(1);
        let outputs = nodes(1);
        let edges = vec![
            Edge::new(sensors[0], hiddens[0], 1.),
            Edge::new(hiddens[0], hiddens[0], 1.),
            Edge::new(hiddens[0], outputs[0], 1.),
        ];

        let network = Network::new(sensors, outputs, edges);
        let mut previous_activations = HashMap::new();
        previous_activations.insert(hiddens[0], 1.);

        let (evaled, _) = network.eval(&vec![1.], &previous_activations);

        assert!((evaled[0] - 0.88079).abs() < 0.00001);
    }

    #[test]
    fn test_recurrence_with_remove_currently_calculating() {
        let sensors = nodes(1);
        let hiddens = nodes(1);
        let outputs = nodes(2);
        let edges = vec![
            Edge::new(sensors[0], hiddens[0], 1.),
            Edge::new(hiddens[0], hiddens[0], 1.),
            Edge::new(hiddens[0], outputs[0], 1.),
            Edge::new(hiddens[0], outputs[1], 1.),
        ];
        let network = Network::new(sensors, outputs, edges);
        let previous_activations = HashMap::new();

        let (evaled, _) = network.eval(&vec![1.], &previous_activations);

        assert_eq!(evaled[0], evaled[1]);
    }

    #[test]
    fn test_get_activations() {
        let sensors = nodes(1);
        let hiddens = nodes(1);
        let outputs = nodes(1);
        let edges = vec![
            Edge::new(sensors[0], hiddens[0], 1.),
            Edge::new(hiddens[0], outputs[0], 1.),
        ];
        let network = Network::new(sensors, outputs, edges);
        let previous_activations = &HashMap::new();

        let (_, activations) = network.eval(&vec![1.], previous_activations);

        let activation = activations.get(&hiddens[0]);
        assert!(activation.is_some());
        assert!((activation.unwrap() - 0.73105).abs() < 0.00001);
    }
}
