#pragma once
#include <memory>
#include <vector>
#include <optional>
#include "rust/cxx.h"

#include "kaminpar/vendor/KaMinPar/library/libkaminpar.h"

class PartitionerBuilder
{
public:
    PartitionerBuilder();
    void set_threads(int threads);
    void set_epsilon(double epsilon);
    void set_seed(uint64_t seed);
    void set_edge_weights(rust::Vec<int64_t> edge_weights);
    void set_node_weights(rust::Vec<int32_t> node_weights);
    std::unique_ptr<std::vector<uint32_t>> partition(rust::Vec<uint64_t> nodes, rust::Vec<uint32_t> edges, uint32_t num_partitions);

private:
    int m_threads;
    double m_epsilon;
    uint64_t m_seed;
    std::optional<rust::Vec<int32_t>> m_node_weights;
    std::optional<rust::Vec<int64_t>> m_edge_weights;
};

std::unique_ptr<PartitionerBuilder> new_partition_builder();
