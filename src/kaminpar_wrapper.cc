#include "kaminpar/include/kaminpar_wrapper.h"
#include "rust/cxx.h"

#include <iostream>
#include <span>

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

void PartitionerBuilder::set_edge_weights(rust::Vec<int32_t> edge_weights)
{
    m_edge_weights = edge_weights;
}

void PartitionerBuilder::set_node_weights(rust::Vec<int32_t> node_weights)
{
    m_node_weights = node_weights;
}

std::unique_ptr<std::vector<uint32_t>> PartitionerBuilder::partition(
    rust::Vec<uint64_t> nodes,
    uint32_t nnodes,
    rust::Vec<uint32_t> edges,
    uint32_t nedges,
    uint32_t num_partitions)
{
    kaminpar::shm::Context ctx = kaminpar::shm::create_default_context();
    kaminpar::KaMinPar partitioner(m_threads, ctx);
    partitioner.set_output_level(kaminpar::OutputLevel::QUIET);

    if (m_edge_weights.has_value() && m_node_weights.has_value()) {
      partitioner.borrow_and_mutate_graph(
          std::span(nodes.data(), nnodes),
          std::span(edges.data(), nedges),
          std::span(m_node_weights->data(), nnodes),
          std::span(m_edge_weights->data(), nedges)
          );
    } else if (m_node_weights.has_value()) {
      partitioner.borrow_and_mutate_graph(
          std::span(nodes.data(), nnodes),
          std::span(edges.data(), nedges),
          std::span(m_node_weights->data(), nnodes),
          {});
    } else if (m_edge_weights.has_value()) {
      partitioner.borrow_and_mutate_graph(
          std::span(nodes.data(), nnodes),
          std::span(edges.data(), nedges),
          {},
          std::span(m_edge_weights->data(), nedges));
    } else {
      partitioner.borrow_and_mutate_graph(
          std::span(nodes.data(), nnodes),
          std::span(edges.data(), nedges),
          {},
          {});
    }

    std::vector<kaminpar::shm::BlockID> partition(nodes.size() - 1);
    partitioner.compute_partition(num_partitions, std::span(partition.data(), nodes.size() - 1)); // compute partition
    return std::make_unique<std::vector<uint32_t>>(partition.begin(), partition.end());
}

std::unique_ptr<PartitionerBuilder> new_partition_builder()
{
    return std::make_unique<PartitionerBuilder>();
}
