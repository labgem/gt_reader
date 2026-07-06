# Example

```rust
let filename = "tests/test_data/pgp_graph_with_properties.gt";
// read_gt function reads a .gt file
let graph: gt_reader::GraphToolGraph = gt_reader::read_gt();
// The edges are represented in linked list, for every vertex, there is a vector containing their successor vertices
let edges: Vec<Vec<usize>> = graph.edges;

// We can check whether a graph is directed by looking in .directed flag
let is_directed = graph.directed;

// We can access a vertex property by the name of the property and the index of the vertex in the proper typed map (here a boolean map)
let first_vertex_is_valid = graph.vertex_properties.bool_maps.get(&String::from("valid").get(0));
```
