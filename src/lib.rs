#![warn(clippy::pedantic)]

use petgraph::visit::EdgeRef;
use petgraph::Graph;

const DEFAULT_EPSILON: f64 = 0.03;

pub type UndirectedGraph = petgraph::graph::UnGraph<(), ()>;
pub type UndirectedEdgeWeightedGraph = petgraph::graph::UnGraph<(), i64>;
pub type UndirectedWeightedGraph = petgraph::graph::UnGraph<i32, i64>;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("kaminpar/include/kaminpar_wrapper.h");

        type PartitionerBuilder;

        fn new_partition_builder() -> UniquePtr<PartitionerBuilder>;
        fn set_threads(self: Pin<&mut PartitionerBuilder>, threads: i32);
        fn set_epsilon(self: Pin<&mut PartitionerBuilder>, epsilon: f64);
        fn set_seed(self: Pin<&mut PartitionerBuilder>, seed: u64);

        fn set_edge_weights(self: Pin<&mut PartitionerBuilder>, edge_weights: Vec<i64>);
        fn set_node_weights(self: Pin<&mut PartitionerBuilder>, node_weights: Vec<i32>);
        fn partition(
            self: Pin<&mut PartitionerBuilder>,
            nodes: Vec<u64>,
            edges: Vec<u32>,
            num_partitions: u32,
        ) -> UniquePtr<CxxVector<u32>>;
    }
}

pub struct PartitionerBuilder {
    threads: Option<std::num::NonZeroUsize>,
    epsilon: f64,
    seed: u64,
}

impl Default for PartitionerBuilder {
    fn default() -> Self {
        Self::with_epsilon(DEFAULT_EPSILON)
    }
}

impl PartitionerBuilder {
    #[must_use]
    pub fn with_epsilon(epsilon: f64) -> Self {
        Self {
            threads: std::thread::available_parallelism().ok(),
            epsilon,
            seed: 1,
        }
    }

    #[must_use]
    pub fn threads(mut self, threads: std::num::NonZeroUsize) -> Self {
        self.threads = Some(threads);
        self
    }

    #[must_use]
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    #[must_use]
    pub fn epsilon(mut self, epsilon: f64) -> Self {
        self.epsilon = epsilon;
        self
    }

    #[must_use]
    pub fn partition<N, E>(
        self,
        graph: Graph<N, E, petgraph::Undirected>,
        num_partitions: u32,
    ) -> Vec<u32> {
        let mut partition_builder = ffi::new_partition_builder();
        if let Some(threads) = self.threads {
            partition_builder
                .pin_mut()
                .set_threads(threads.get() as i32);
        } else {
            partition_builder
                .pin_mut()
                .set_threads(num_partitions as i32);
        }

        let mut nodes: Vec<u64> = Vec::with_capacity(graph.node_count() + 1);
        let mut edges: Vec<u32> = Vec::with_capacity(graph.edge_count() * 2);
        let mut edge_weights: Vec<i64> = Vec::with_capacity(graph.edge_count() * 2);
        let mut node_weights: Vec<i32> = Vec::with_capacity(graph.node_count());

        nodes.push(0);
        for node in graph.node_indices() {
            let mut num_edges_for_node = 0;
            if let Ok(node_weight) = graph.node_weight(node).unwrap().try_into() {
                node_weights.push(node_weight);
            }
            for edge in graph.edges(node) {
                edges.push((edge.target().index() as i32).try_into().unwrap());
                if let Ok(edge_weight) = edge.weight().try_into() {
                    edge_weights.push(edge_weight);
                }
                num_edges_for_node += 1;
            }
            nodes.push(num_edges_for_node);
        }

        if !edge_weights.is_empty() {
            partition_builder.pin_mut().set_edge_weights(edge_weights);
        }

        if !node_weights.is_empty() {
            partition_builder.pin_mut().set_node_weights(node_weights);
        }

        partition_builder.pin_mut().set_epsilon(self.epsilon);
        partition_builder.pin_mut().set_seed(self.seed);
        let output_assignments_cpp =
            partition_builder
                .pin_mut()
                .partition(nodes, edges, num_partitions);
        let output_assignments: Vec<u32> = output_assignments_cpp.iter().copied().collect();
        output_assignments
    }
}
