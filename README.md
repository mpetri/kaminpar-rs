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

1. Setup commands on baremetal Ubuntu

- Update GCC version:

```bash
sudo apt update
sudo apt install software-properties-common
sudo add-apt-repository ppa:ubuntu-toolchain-r/test
sudo apt install gcc-13 g++-13
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-13 100 --slave /usr/bin/g++ g++ /usr/bin/g++-13
gcc --version
```

- Install `libtbb` & `libnuma`:

```bash
sudo apt-get install libnuma-dev
sudo apt install libtbb-dev
```

2. Setup commands using conda

```bash
conda create -n <name> python=<version>
conda install conda-forge::gcc_linux-64"
conda install conda-forge::gcc -y"
conda install 'gxx[version=">=14"]'
conda install conda-forge::tbb-devel
conda install libnuma numactl
```

---

# Usage

as a library call with a node and edge weighted graph:

```rust
fn main() {
    let mut graph = petgraph::graph::UnGraph::<i32, i64>::new_undirected();
    let a = graph.add_node(5);
    let b = graph.add_node(1);
    let c = graph.add_node(1);
    let d = graph.add_node(3);
    let e = graph.add_node(3);
    let f = graph.add_node(4);
    let g = graph.add_node(3);

    graph.add_edge(a, b, 1);
    graph.add_edge(a, g, 3);
    graph.add_edge(b, c, 3);
    graph.add_edge(b, g, 1);
    graph.add_edge(c, d, 1);
    graph.add_edge(d, g, 4);
    graph.add_edge(d, e, 1);
    graph.add_edge(e, f, 1);
    graph.add_edge(e, g, 1);
    graph.add_edge(f, g, 6);

    let num_partitions: u32 = 2;

    let partition = kaminpar::PartitionerBuilder::with_epsilon(0.03)
        .seed(123)
        .threads(std::num::NonZeroUsize::new(6).unwrap())
        .partition_weighted(&graph, num_partitions);

    println!("{:?}", partition);
}
```

or unweighted

```rust
fn main() {
    let mut graph = petgraph::graph::UnGraph::<(), ()>::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());

    graph.add_edge(a, b, ());
    graph.add_edge(a, e, ());

    graph.add_edge(b, c, ());
    graph.add_edge(b, e, ());

    graph.add_edge(c, d, ());
    graph.add_edge(d, e, ());

    let num_partitions: u32 = 2;

    let partition = kaminpar::PartitionerBuilder::with_epsilon(0.03)
        .seed(123)
        .threads(std::num::NonZeroUsize::new(6).unwrap())
        .partition(&graph, num_partitions);

    println!("{:?}", partition);
}
```

# License

MIT
