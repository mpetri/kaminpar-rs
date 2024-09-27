#![warn(clippy::pedantic)]

use petgraph::visit::EdgeRef;
use petgraph::Graph;
use thiserror::Error;

const DEFAULT_EPSILON: f64 = 0.03;

#[derive(Error, Debug)]
pub enum KaminParError {
    #[error("node weight missing")]
    NodeWeightMissing,
    #[error("error converting nodeindex to u32")]
    NodeIdConversionError(#[from] std::num::TryFromIntError),
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("kaminpar/include/kaminpar_wrapper.h");

        type PartitionerBuilder;

        fn new_partition_builder() -> UniquePtr<PartitionerBuilder>;
        fn set_threads(self: Pin<&mut PartitionerBuilder>, threads: i32);
        fn set_epsilon(self: Pin<&mut PartitionerBuilder>, epsilon: f64);
        fn set_seed(self: Pin<&mut PartitionerBuilder>, seed: u64);

        fn set_edge_weights(self: Pin<&mut PartitionerBuilder>, edge_weights: Vec<i32>);
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
    ///
    /// Create partition builder with epsilon (slack for partition size)
    ///
    #[must_use]
    pub fn with_epsilon(epsilon: f64) -> Self {
        Self {
            threads: std::thread::available_parallelism().ok(),
            epsilon,
            seed: 1,
        }
    }

    ///
    /// Create partition builder with specific number of threads
    ///
    #[must_use]
    pub fn threads(mut self, threads: std::num::NonZeroUsize) -> Self {
        self.threads = Some(threads);
        self
    }

    ///
    /// Fix seed to specific number
    ///
    #[must_use]
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    fn create_edges_and_nodes<N, E>(
        graph: &Graph<N, E, petgraph::Undirected>,
    ) -> Result<(Vec<u64>, Vec<u32>), KaminParError> {
        let mut nodes: Vec<u64> = Vec::with_capacity(graph.node_count() + 1);
        let mut edges: Vec<u32> = Vec::with_capacity(graph.edge_count());
        nodes.push(0);
        let mut cum_edge_count = 0;
        for node in graph.node_indices() {
            for edge in graph.edges(node) {
                edges.push(edge.target().index().try_into()?);
                cum_edge_count += 1;
            }
            nodes.push(cum_edge_count);
        }
        Ok((nodes, edges))
    }

    ///
    /// Run partitioning over undirected graph
    ///
    /// # Errors
    ///
    /// - Will return `Err` if node index can't be converted to u32
    ///
    pub fn partition<N, E>(
        self,
        graph: &Graph<N, E, petgraph::Undirected>,
        num_partitions: u32,
    ) -> Result<Vec<u32>, KaminParError> {
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

        let (nodes, edges) = Self::create_edges_and_nodes(graph)?;

        partition_builder.pin_mut().set_epsilon(self.epsilon);
        partition_builder.pin_mut().set_seed(self.seed);
        let output_assignments_cpp =
            partition_builder
                .pin_mut()
                .partition(nodes, edges, num_partitions);
        let output_assignments: Vec<u32> = output_assignments_cpp.iter().copied().collect();
        Ok(output_assignments)
    }

    ///
    /// Run partitioning over edge  weighted undirected graphs
    ///
    /// # Errors
    ///
    /// - Will return `Err` if node index can't be converted to u32
    ///
    pub fn partition_edge_weighted<N, E: Into<i32> + Copy>(
        self,
        graph: &Graph<N, E, petgraph::Undirected>,
        num_partitions: u32,
    ) -> Result<Vec<u32>, KaminParError> {
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

        let (nodes, edges) = Self::create_edges_and_nodes(graph)?;

        let mut edge_weights: Vec<i32> = Vec::with_capacity(graph.edge_count());
        for node in graph.node_indices() {
            for edge in graph.edges(node) {
                edge_weights.push((*edge.weight()).into());
            }
        }
        partition_builder.pin_mut().set_edge_weights(edge_weights);
        partition_builder.pin_mut().set_epsilon(self.epsilon);
        partition_builder.pin_mut().set_seed(self.seed);
        let output_assignments_cpp =
            partition_builder
                .pin_mut()
                .partition(nodes, edges, num_partitions);
        let output_assignments: Vec<u32> = output_assignments_cpp.iter().copied().collect();
        Ok(output_assignments)
    }

    ///
    /// Run partitioning over edge and node weighted undirected graph
    ///
    /// # Errors
    ///
    /// - Will return `Err` if not all nodes are weighted
    /// - Will return `Err` if node index can't be converted to u32
    ///x
    pub fn partition_weighted<N: Into<i32> + Copy, E: Into<i32> + Copy>(
        self,
        graph: &Graph<N, E, petgraph::Undirected>,
        num_partitions: u32,
    ) -> Result<Vec<u32>, KaminParError> {
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

        let (nodes, edges) = Self::create_edges_and_nodes(graph)?;

        let mut edge_weights: Vec<i32> = Vec::with_capacity(graph.edge_count());
        let mut node_weights: Vec<i32> = Vec::with_capacity(graph.node_count());

        for node in graph.node_indices() {
            node_weights.push(
                graph
                    .node_weight(node)
                    .map(|nw| (*nw).into())
                    .ok_or(KaminParError::NodeWeightMissing)?,
            );
            for edge in graph.edges(node) {
                edge_weights.push((*edge.weight()).into());
            }
        }
        partition_builder.pin_mut().set_edge_weights(edge_weights);
        partition_builder.pin_mut().set_node_weights(node_weights);

        partition_builder.pin_mut().set_epsilon(self.epsilon);
        partition_builder.pin_mut().set_seed(self.seed);
        let output_assignments_cpp =
            partition_builder
                .pin_mut()
                .partition(nodes, edges, num_partitions);
        let output_assignments: Vec<u32> = output_assignments_cpp.iter().copied().collect();
        Ok(output_assignments)
    }
}
