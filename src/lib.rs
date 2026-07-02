//! Read graph-tool gt compressed graph tool format
//! 
//! ref. https://graph-tool.skewed.de/static/docs/stable/gt_format.html

/* std use */
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Write};

/* crate use */
// use petgraph::graph::DiGraph;
// use petgraph::graph::UnGraph;
// use petgraph::graph::Graph;


/* module declaration */

pub mod error;

/* project use */
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Read the `gt` magic string
///
/// the magic string is utf-8 encoded
/// it totalizes 6 bytes.
pub fn read_magic_string(reader: &mut BufReader<File>) -> std::io::Result<bool> {
    let mut buffer = vec![0u8; 6];
    let _read_bytes = reader.read(&mut buffer)?;
    let magic_string = "⛾ gt";
    Ok(buffer == magic_string.as_bytes().to_vec())
}

/// Read the `gt` version number
///
/// The `gt` version number is a 1 byte integer
pub fn read_version_number(reader: &mut BufReader<File>) -> std::io::Result<u8> {
    let mut buffer = vec![0u8; 1];
    let _read_bytes = reader.read(&mut buffer)?;
    let version_number = buffer[0];
    Ok(version_number)
}

/// Integer endianness
/// https://en.wikipedia.org/wiki/Endianness
#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian
}

/// Read the integer endianness
/// 0x00 for little-endian, 0x01 for big-endian
pub fn read_endianness(reader: &mut BufReader<File>) -> std::io::Result<Endianness> {
    let mut buffer = vec![0u8; 1];
    let _read_bytes = reader.read(&mut buffer)?;
    let endianness_flag = buffer[0];
    let endianness = match endianness_flag {
        0x00 => Endianness::LittleEndian,
        0x01 => Endianness::BigEndian,
        _ => panic!("Endianness flag not understood")
    };
    Ok(endianness)
}

pub fn vec2array<T, const N: usize>(vec: Vec<T>) -> Result<[T; N]> {
	vec.try_into().map_err(|_| Error::MismatchedLength { expect: N })
}

pub fn bytes_to_u64(bytes: &Vec<u8>, endianness: Endianness) -> u64 {    
    let bytes: [u8; 8] = vec2array::<u8, 8>(bytes[..8].to_owned()).expect("Error: cannot convert Vec<u8> to [u8; 8]");
    match endianness {
        Endianness::LittleEndian => u64::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => u64::from_be_bytes(bytes.to_owned())
    }
}

pub fn bytes_to_u32(bytes: &Vec<u8>, endianness: Endianness) -> u32 {    
    let bytes: [u8; 4] = vec2array::<u8, 4>(bytes[..4].to_owned()).expect("Error: cannot convert Vec<u8> to [u8; 4]");
    match endianness {
        Endianness::LittleEndian => u32::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => u32::from_be_bytes(bytes.to_owned())
    }
}

pub fn bytes_to_u16(bytes: &Vec<u8>, endianness: Endianness) -> u16 {    
    let bytes: [u8; 2] = vec2array::<u8, 2>(bytes[..2].to_owned()).expect("Error: cannot convert Vec<u8> to [u8; 2]");
    match endianness {
        Endianness::LittleEndian => u16::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => u16::from_be_bytes(bytes.to_owned())
    }
}

pub fn bytes_to_u8(bytes: &Vec<u8>) -> u8 {    
    bytes[0]
}

pub fn bytes_to_int<T>(bytes: &Vec<u8>, endianness: Endianness, n_bytes_per_node_identifier: usize) -> u64 {
    match n_bytes_per_node_identifier {
        1 => bytes_to_u8(bytes) as u64,
        2 => bytes_to_u16(bytes, endianness) as u64,
        4 => bytes_to_u32(bytes, endianness) as u64,
        8 => bytes_to_u64(bytes, endianness) as u64,
        _ => panic!("Error: support encoding on 1, 2, 4 or 8 bytes only.")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    MismatchedLength {
        expect: usize,
    }
}

pub fn read_string(reader: &mut BufReader<File>, endianness: Endianness) -> std::io::Result<String> {
    let mut length_buffer = vec![0u8; 8]; // length is encoded on 8 bytes
    let _read_bytes = reader.read(&mut length_buffer)?;
    let length = bytes_to_u64(&length_buffer, endianness) as usize;
    let mut string_buffer = vec![0u8; length];
    let _read_bytes = reader.read(&mut string_buffer)?;
    let string = String::from_utf8(string_buffer).expect("Error: cannot parse UTF-8 string");
    dbg!(&string);
    Ok(string)
}

/// Read whether the graph is directed or undirected
pub fn read_directed(reader: &mut BufReader<File>) -> std::io::Result<bool> {
    let mut flag = vec![0u8; 1];
    reader.read(&mut flag)?;
    Ok(is_directed(flag[0]))
}

/// Check if the stored graph is directed or not
pub fn is_directed(directed_flag: u8) -> bool {
    match directed_flag {
        0x00 => false,
        0x01 => true,
        _ => panic!("The gt file directed/undirected flag is not understood")
    }
}

/// Identify the number of bytes used to encode the nodes identifiers
/// Returns None if the number of nodes exceeds u64 max capacity.
pub fn n_bytes_node_encoding(n_nodes: usize) -> Option<usize> {
    let sizes: Vec<usize> = vec![1, 2, 4, 8];
    let capacities: Vec<usize> = vec![u8::MAX as usize, u16::MAX as usize, u32::MAX as usize, u64::MAX as usize];
    for (i, capacity) in capacities.iter().enumerate() {
        if *capacity > n_nodes {
            return Some(sizes[i]);
        }
    }
    None
}

/// Read edges
pub fn read_n_successors(reader: &mut BufReader<File>, endianness: Endianness, n_bytes_per_node_identifier: usize, n_successors: usize) -> std::io::Result<Vec<usize>> {
    let mut successors: Vec<usize> = Vec::new();
    let mut node_identifier_buffer = vec![0u8; n_bytes_per_node_identifier];
    for _nodes in 0..n_successors {
        let _read_bytes = reader.read(&mut node_identifier_buffer)?;
        successors.push(bytes_to_int::<u64>(&node_identifier_buffer, endianness, n_bytes_per_node_identifier) as usize);
    }
    Ok(successors)
}

/// Parse the edges
pub fn read_edges(reader: &mut BufReader<File>, endianness: Endianness) -> std::io::Result<Vec<Vec<usize>>> {
    let mut length_buffer = vec![0u8; 8]; // length is encoded on 8 bytes
    let _read_bytes = reader.read(&mut length_buffer)?;
    let n_nodes: usize = bytes_to_u64(&length_buffer, endianness).try_into().unwrap();
    dbg!(&n_nodes);
    let mut edges: Vec<Vec<usize>> = Vec::new();
    let n_bytes_per_node_identifier: usize = n_bytes_node_encoding(n_nodes).expect("Error: could not identify the number of bytes required to encode the node identifiers.");
    dbg!(&n_bytes_per_node_identifier);
    for node in 0..n_nodes {
        dbg!(&node);
        let mut n_successors_buffer = vec![0u8; 8];
        let _read_bytes = reader.read(&mut n_successors_buffer);
        let n_successors: usize = bytes_to_u64(&n_successors_buffer, endianness).try_into().unwrap();
        edges.push(read_n_successors(reader, endianness, n_bytes_per_node_identifier, n_successors).expect("Error: could not read edges"));
    }
    Ok(edges)
}

/// Read a .gt compressed file
/// TODO support for undirected graph
pub fn read_gt<P>(gt_path: P) -> std::io::Result<Vec<Vec<usize>>>
where P: AsRef<Path> {
    let file = File::open(gt_path)?;
    let mut reader = BufReader::new(file);
    read_magic_string(&mut reader)?;
    let version = read_version_number(&mut reader)?;
    dbg!(&version);
    let endianness: Endianness = read_endianness(&mut reader)?;
    let comment = read_string(&mut reader, endianness);
    dbg!(&comment);
    let directed: bool = read_directed(&mut reader).unwrap();
    let edges = read_edges(&mut reader, endianness).expect("Error reading edges");
    Ok(edges)
}

/// Count the number of edges in the graph
pub fn count_edges(edges: &Vec<Vec<usize>>) -> usize {
    edges.iter().map(|successors| successors.len()).sum()
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;

    #[test]
    fn read_pgp_gt() {
        let filename = "tests/test_data/pgp_graph_without_properties.gt";
        let edges = read_gt(filename).unwrap();
        assert!(edges.len() > 0);
    }

    #[test]
    fn read_simple_gt() {
        let filename = "tests/test_data/vsg_graph.gt";
        let edge_list = read_gt(filename).unwrap();
        assert!(edge_list.len() == 3);
        assert!(count_edges(&edge_list) == 2);
    }
}
