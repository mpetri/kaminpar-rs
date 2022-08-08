# KaMinPar Rust Wrapper [![Crates.io][crates-badge]][crates-url] [![Docs.rs][docs-badge]][docs-rs] [![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/kaminpar.svg
[crates-url]: https://crates.io/crates/kaminpar
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[docs-rs]: https://docs.rs/kaminpar
[docs-badge]: https://img.shields.io/docsrs/kaminpar/0.2.6

KaMinPar is a shared-memory parallel tool to heuristically solve the graph partitioning problem:. This code provides a 
small rust wrapper around the main KaMinPar repostitory here: [https://github.com/KaHIP/KaMinPar](https://github.com/KaHIP/KaMinPar)

This KaMinPar algorithm is described in:

```
@inproceedings{DBLP:conf/esa/GottesburenH00S21,
  author    = {Lars Gottesb{\"{u}}ren and
               Tobias Heuer and
               Peter Sanders and
               Christian Schulz and
               Daniel Seemaier},
  title     = {Deep Multilevel Graph Partitioning},
  booktitle = {29th Annual European Symposium on Algorithms, {ESA} 2021, September
               6-8, 2021, Lisbon, Portugal (Virtual Conference)},
  series    = {LIPIcs},
  volume    = {204},
  pages     = {48:1--48:17},
  publisher = {Schloss Dagstuhl - Leibniz-Zentrum f{\"{u}}r Informatik},
  year      = {2021},
  url       = {https://doi.org/10.4230/LIPIcs.ESA.2021.48},
  doi       = {10.4230/LIPIcs.ESA.2021.48}
}
```

Note: This is only a simple wrapper, all credit belongs to the original authors!

# What is KaMinPar (taken from original repo)

KaMinPar is a shared-memory parallel tool to heuristically solve the graph partitioning problem: divide a graph into k disjoint 
blocks of roughly equal weight while minimizing the number of edges between blocks. Competing algorithms are mostly evaluated for 
small values of k. If k is large, they often compute highly imbalance solutions, solutions of low quality or suffer excessive 
running time. KaMinPar substantially mitigates these problems. It computes partitions of comparable quality to other high-quality 
graph partitioning tools while guaranteeing the balance constraint for unweighted input graphs. Moreover, for large values of k, it is 
an order of magnitude faster than competing algorithms.

# Requirements

The actual C++ code requires:

- Modern C++-20 ready compiler such as g++ version 10 or higher
- A C++17 port requiring g++ version 7.2.0 or higher is available in branch c++17
- CMake
- Intel Thread Building Blocks library (TBB)
- `libnuma-dev` on ubuntu

# Usage

as a library call

```rust
fn main() {
    let nodes: Vec<u64> = vec![0, 2, 5, 7, 9, 12];
    let node_weights: Vec<i32> = vec![1, 2, 3, 4, 5];
    let edges: Vec<u32> = vec![1, 4, 0, 2, 4, 1, 3, 2, 4, 0, 1, 3];
    let edge_weights: Vec<i64> = vec![1, 2, 3, 4, 3, 2, 1, 2, 3, 4, 3, 2];
    let num_partitions: u32 = 2;

    let partition = kaminpar::PartitionerBuilder::with_epsilon(0.03)
        .seed(123)
        .threads(std::num::NonZeroUsize::new(6).unwrap())
        .partition(
            nodes,
            Some(node_weights),
            edges,
            Some(edge_weights),
            num_partitions,
        );

    println!("{:?}", partition);
}

```

# License

MIT