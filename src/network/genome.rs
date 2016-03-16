use network::Network;
use network::Node;
use network::Edge;

pub struct Genome {
    input_count: usize,
    output_count: usize,
    genes: Vec<Gene>,
}

impl Genome {
    pub fn new(input_count: usize, output_count: usize, genes: Vec<Gene>) -> Genome {
        Genome {
            input_count: input_count,
            output_count: output_count,
            genes: genes,
        }
    }

    pub fn to_network(&self) -> Network {
        let total_inputs = self.input_count + 1;
        Network {
            sensors: (0..).take(total_inputs).map(|i| Node::new(i)).collect(),
            outputs: (total_inputs..)
                         .take(self.output_count)
                         .map(|i| Node::new(i))
                         .collect(),
            edges: self.genes
                       .iter()
                       .map(|gene| {
                           Edge::new(Node::new(gene.in_index),
                                     Node::new(gene.out_index),
                                     gene.weight)
                       })
                       .collect(),
        }
    }
}

pub struct Gene {
    in_index: usize,
    out_index: usize,
    weight: f64,
}

impl Gene {
    pub fn new(in_index: usize, out_index: usize, weight: f64, innovation_number: usize) -> Gene {
        Gene {
            in_index: in_index,
            out_index: out_index,
            weight: weight,
        }
    }
}

#[cfg(test)]
mod test {
    use network::Edge;
    use network::Node;

    use super::Genome;
    use super::Gene;

    #[test]
    fn test_simple_from_genome() {
        let genes = vec![
                        Gene::new(0, 2, 0.7, 1),
                        Gene::new(1, 2, 0.5, 2),
                    ];
        let genome = Genome::new(1, 1, genes);
        let network = genome.to_network();

        assert_eq!(network.edges.len(), 2);
        assert_eq!(network.edges[0], Edge::new(Node::new(0), Node::new(2), 0.7));
        assert_eq!(network.edges[1], Edge::new(Node::new(1), Node::new(2), 0.5));

        // One sensor is for the bias node.
        assert_eq!(network.sensors.len(), 2);
        assert_eq!(network.sensors[0], Node::new(0));
        assert_eq!(network.sensors[1], Node::new(1));

        assert_eq!(network.outputs.len(), 1);
        assert_eq!(network.outputs[0], Node::new(2));
    }
}
