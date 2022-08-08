#include "kaminpar/include/kaminpar_wrapper.h"
#include "rust/cxx.h"

#include <iostream>

PartitionerBuilder::PartitionerBuilder() : m_threads(1), m_epsilon(0.03), m_seed(1), m_node_weights(), m_edge_weights() {}

void PartitionerBuilder::set_threads(int threads)
{
    m_threads = threads;
}

void PartitionerBuilder::set_epsilon(double epsilon)
{
    m_epsilon = epsilon;
}

void PartitionerBuilder::set_seed(uint64_t seed)
{
    m_seed = seed;
}

void PartitionerBuilder::set_edge_weights(rust::Vec<int64_t> edge_weights)
{
    m_edge_weights = edge_weights;
}

void PartitionerBuilder::set_node_weights(rust::Vec<int32_t> node_weights)
{
    m_node_weights = node_weights;
}

std::unique_ptr<std::vector<uint32_t>> PartitionerBuilder::partition(rust::Vec<uint64_t> nodes, rust::Vec<uint32_t> edges, uint32_t num_partitions)
{
    auto builder = libkaminpar::PartitionerBuilder::from_adjacency_array(nodes.size() - 1, nodes.data(), edges.data());
    if (m_edge_weights.has_value())
    {
        builder.with_edge_weights(m_edge_weights->data());
    }
    if (m_node_weights.has_value())
    {
        builder.with_node_weights(m_node_weights->data());
    }

    libkaminpar::Partitioner partitioner = builder.create();

    partitioner.set_option("--threads", std::to_string(m_threads));
    partitioner.set_option("--epsilon", std::to_string(m_epsilon));
    partitioner.set_option("--seed", std::to_string(m_seed));

    auto partition = partitioner.partition(num_partitions); // compute partition
    return std::make_unique<std::vector<uint32_t>>(partition.get(), partition.get() + partitioner.partition_size());
}

std::unique_ptr<PartitionerBuilder> new_partition_builder()
{
    return std::make_unique<PartitionerBuilder>();
}
