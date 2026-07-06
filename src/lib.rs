//! Read graph-tool gt compressed graph-tool graph format
//!
//! ref. https://graph-tool.skewed.de/static/docs/stable/gt_format.html

/* std use */
use std::fs::File;
use std::io::Read;
use std::path::Path;

/* crate use */

/* project use */

use crate::property_maps::{PropertyMaps, PropertyMapsReader};

pub mod error;
pub mod property_maps;
pub mod reader;

/// A object handling the datastructures that we can find in the gt file format
pub struct GraphToolGraph {
    pub directed: bool, // true if the graph is directed, false if it is undirected
    pub edges: Vec<Vec<usize>>, // A vector of successor vertices for every vertex
    pub edge_properties: property_maps::PropertyMaps<(usize, usize)>, // for edge (u, v) key is (u, v)
    pub vertex_properties: property_maps::PropertyMaps<usize>,        // for vertex u, key is u
    pub graph_properties: property_maps::PropertyMaps<usize>,         // key is graph identifier
    pub comment: String,
}

/// Read a .gt compressed file
pub fn read_gt<P>(gt_path: P) -> std::io::Result<GraphToolGraph>
where
    P: AsRef<Path>,
{
    let mut file = File::open(gt_path)?;
    let mut buffer: Vec<u8> = Vec::new();
    let _read_bytes = file.read_to_end(&mut buffer);
    let mut bytes = buffer.iter();
    reader::read_magic_string(&mut bytes)
        .expect("gt file format should start with the magic string \"⛾ gt\"");
    let _version = reader::read_version_number(&mut bytes);
    let endianness: reader::Endianness = reader::read_endianness(&mut bytes).unwrap();
    let comment = reader::read_string(&mut bytes, endianness).unwrap();
    let directed: bool = reader::read_directed(&mut bytes).unwrap();
    let edges = reader::read_edges(&mut bytes, endianness).expect("Error reading edges");
    let mut graph = GraphToolGraph {
        directed: directed,
        edges: edges,
        edge_properties: PropertyMaps::new(),
        vertex_properties: PropertyMaps::new(),
        graph_properties: PropertyMaps::new(),
        comment: comment,
    };
    graph.read_property_maps(&mut bytes, endianness);
    Ok(graph)
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;

    /// Count the number of edges in a directed graph
    fn count_edges(edges: &Vec<Vec<usize>>) -> usize {
        edges.iter().map(|successors| successors.len()).sum()
    }

    #[test]
    fn read_pgp_gt() {
        let filename = "tests/test_data/pgp_graph_without_properties.gt";
        let edges = read_gt(filename).unwrap().edges;
        assert!(edges.len() > 0);
    }

    #[test]
    fn read_simple_gt() {
        let filename = "tests/test_data/vsg_graph.gt";
        let edge_list = read_gt(filename).unwrap().edges;
        assert!(edge_list.len() == 3);
        assert!(count_edges(&edge_list) == 2);
    }

    #[test]
    fn read_pgp() {
        let filename = "tests/test_data/pgp_graph_with_properties.gt";
        let graph = read_gt(filename).unwrap();

        assert!(graph
            .vertex_properties
            .bool_maps
            .contains_key(&String::from("valid")))
    }
}
