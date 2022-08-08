#![warn(clippy::pedantic)]

const DEFAULT_EPSILON: f64 = 0.03;

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
    pub fn partition(
        self,
        nodes: Vec<u64>,
        node_weights: Option<Vec<i32>>,
        edges: Vec<u32>,
        edge_weights: Option<Vec<i64>>,
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

        if let Some(edge_weights) = edge_weights {
            partition_builder.pin_mut().set_edge_weights(edge_weights);
        }

        if let Some(node_weights) = node_weights {
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
