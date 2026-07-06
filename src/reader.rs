/* std use */

use std::slice::Iter;

/* crate use */
// use petgraph::graph::DiGraph;
// use petgraph::graph::UnGraph;
// use petgraph::graph::Graph;

/* module declaration */

/* project use */
use crate::error;

pub fn take_n_bytes(pos: &mut Iter<'_, u8>, n: usize) -> Vec<u8> {
    pos.take(n).cloned().collect()
}

/// Read the `gt` magic string
///
/// the magic string is utf-8 encoded
/// it totalizes 6 bytes.
pub fn read_magic_string(pos: &mut Iter<'_, u8>) -> error::Result<bool> {
    let buffer = take_n_bytes(pos, 6);
    let magic_string = "⛾ gt";
    Ok(buffer == magic_string.as_bytes().to_vec())
}

/// Read the `gt` version number
///
/// The `gt` version number is a 1 byte integer
pub fn read_version_number(pos: &mut Iter<'_, u8>) -> error::Result<u8> {
    let buffer = take_n_bytes(pos, 1);
    let version_number = buffer[0];
    Ok(version_number)
}

pub fn read_bool(pos: &mut Iter<'_, u8>, _endianness: Endianness) -> error::Result<bool> {
    let buffer = take_n_bytes(pos, 1);
    let number = buffer[0];
    Ok(number == 0x01)
}

pub fn read_f64(pos: &mut Iter<'_, u8>, endianness: Endianness) -> error::Result<f64> {
    let bytes: Vec<u8> = take_n_bytes(pos, 8);
    let bytes: [u8; 8] = vec2array::<u8, 8>(bytes[..8].to_owned())
        .expect("Error: cannot convert Vec<u8> to [u8; 8]");
    match endianness {
        Endianness::LittleEndian => Ok(f64::from_le_bytes(bytes)),
        Endianness::BigEndian => Ok(f64::from_be_bytes(bytes)),
    }
}

/// Integer endianness
/// ref. https://en.wikipedia.org/wiki/Endianness
#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

/// Read the integer endianness
/// 0x00 for little-endian, 0x01 for big-endian
pub fn read_endianness(pos: &mut Iter<'_, u8>) -> error::Result<Endianness> {
    let buffer = take_n_bytes(pos, 1);
    let endianness_flag = buffer[0];
    let endianness = match endianness_flag {
        0x00 => Endianness::LittleEndian,
        0x01 => Endianness::BigEndian,
        _ => panic!("Endianness flag not understood"),
    };
    Ok(endianness)
}

pub fn vec2array<T, const N: usize>(vec: Vec<T>) -> error::Result<[T; N]> {
    vec.try_into()
        .map_err(|_| error::Error::MismatchedLength { expect: N })
}

pub fn bytes_to_u64(bytes: &Vec<u8>, endianness: Endianness) -> u64 {
    let bytes: [u8; 8] = vec2array::<u8, 8>(bytes[..8].to_owned())
        .expect("Error: cannot convert Vec<u8> to [u8; 8]");
    match endianness {
        Endianness::LittleEndian => u64::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => u64::from_be_bytes(bytes.to_owned()),
    }
}

pub fn bytes_to_u32(bytes: &Vec<u8>, endianness: Endianness) -> u32 {
    let bytes: [u8; 4] = vec2array::<u8, 4>(bytes[..4].to_owned())
        .expect("Error: cannot convert Vec<u8> to [u8; 4]");
    match endianness {
        Endianness::LittleEndian => u32::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => u32::from_be_bytes(bytes.to_owned()),
    }
}

pub fn bytes_to_u16(bytes: &Vec<u8>, endianness: Endianness) -> u16 {
    let bytes: [u8; 2] = vec2array::<u8, 2>(bytes[..2].to_owned())
        .expect("Error: cannot convert Vec<u8> to [u8; 2]");
    match endianness {
        Endianness::LittleEndian => u16::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => u16::from_be_bytes(bytes.to_owned()),
    }
}

pub fn bytes_to_u8(bytes: &Vec<u8>) -> u8 {
    bytes[0]
}

pub fn bytes_to_int<T>(
    bytes: &Vec<u8>,
    endianness: Endianness,
    n_bytes_per_node_identifier: usize,
) -> error::Result<u64> {
    let result = match n_bytes_per_node_identifier {
        1 => bytes_to_u8(bytes) as u64,
        2 => bytes_to_u16(bytes, endianness) as u64,
        4 => bytes_to_u32(bytes, endianness) as u64,
        8 => bytes_to_u64(bytes, endianness),
        _ => panic!("Error: support encoding on 1, 2, 4 or 8 bytes only."),
    };
    Ok(result)
}

pub fn read_u64(pos: &mut Iter<'_, u8>, endianness: Endianness) -> error::Result<u64> {
    let bytes = take_n_bytes(pos, 8);
    bytes_to_int::<u64>(&bytes, endianness, 8)
}

pub fn read_size(pos: &mut Iter<'_, u8>, endianness: Endianness) -> error::Result<usize> {
    let size = read_u64(pos, endianness)?;
    Ok(size as usize)
}

pub fn read_i16(pos: &mut Iter<'_, u8>, endianness: Endianness) -> error::Result<i16> {
    let bytes = take_n_bytes(pos, 2);
    let bytes: [u8; 2] = vec2array::<u8, 2>(bytes[..2].to_owned())
        .expect("Error: cannot convert Vec<u8> to [u8; 2]");
    let result = match endianness {
        Endianness::LittleEndian => i16::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => i16::from_be_bytes(bytes.to_owned()),
    };
    Ok(result)
}

pub fn read_i32(pos: &mut Iter<'_, u8>, endianness: Endianness) -> error::Result<i32> {
    let bytes = take_n_bytes(pos, 4);
    let bytes: [u8; 4] = vec2array::<u8, 4>(bytes[..4].to_owned())
        .expect("Error: cannot convert Vec<u8> to [u8; 4]");
    let result = match endianness {
        Endianness::LittleEndian => i32::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => i32::from_be_bytes(bytes.to_owned()),
    };
    Ok(result)
}

pub fn read_i64(pos: &mut Iter<'_, u8>, endianness: Endianness) -> error::Result<i64> {
    let bytes = take_n_bytes(pos, 8);
    let bytes: [u8; 8] = vec2array::<u8, 8>(bytes[..8].to_owned())
        .expect("Error: cannot convert Vec<u8> to [u8; 8]");
    let result = match endianness {
        Endianness::LittleEndian => i64::from_le_bytes(bytes.to_owned()),
        Endianness::BigEndian => i64::from_be_bytes(bytes.to_owned()),
    };
    Ok(result)
}

pub fn read_vector<T>(
    pos: &mut Iter<'_, u8>,
    reader: fn(&mut Iter<'_, u8>, Endianness) -> error::Result<T>,
    endianness: Endianness,
) -> error::Result<Vec<T>> {
    let size = read_size(pos, endianness)?;
    let mut vector: Vec<T> = Vec::with_capacity(size);
    for _ in 0..size {
        let element = reader(pos, endianness).unwrap();
        vector.push(element);
    }
    Ok(vector)
}

pub fn read_string(pos: &mut Iter<'_, u8>, endianness: Endianness) -> error::Result<String> {
    let length_buffer = take_n_bytes(pos, 8); // length is encoded on 8 bytes
    let length = bytes_to_u64(&length_buffer, endianness) as usize;
    let string_buffer = take_n_bytes(pos, length);
    let string = String::from_utf8(string_buffer).expect("Error: cannot parse UTF-8 string");
    Ok(string)
}

/// Read whether the graph is directed or undirected
pub fn read_directed(pos: &mut Iter<'_, u8>) -> error::Result<bool> {
    let flag = take_n_bytes(pos, 1);
    Ok(is_directed(flag[0]))
}

/// Check if the stored graph is directed or not
pub fn is_directed(directed_flag: u8) -> bool {
    match directed_flag {
        0x00 => false,
        0x01 => true,
        _ => panic!("The gt file directed/undirected flag is not understood"),
    }
}

/// Identify the number of bytes used to encode the nodes identifiers
/// Returns None if the number of nodes exceeds u64 max capacity.
pub fn n_bytes_node_encoding(n_nodes: usize) -> Option<usize> {
    let sizes: Vec<usize> = vec![1, 2, 4, 8];
    let capacities: Vec<usize> = vec![
        u8::MAX as usize,
        u16::MAX as usize,
        u32::MAX as usize,
        u64::MAX as usize,
    ];
    for (i, capacity) in capacities.iter().enumerate() {
        if *capacity > n_nodes {
            return Some(sizes[i]);
        }
    }
    None
}

/// Read edges
pub fn read_n_successors(
    pos: &mut Iter<'_, u8>,
    endianness: Endianness,
    n_bytes_per_node_identifier: usize,
    n_successors: usize,
) -> std::io::Result<Vec<usize>> {
    let mut successors: Vec<usize> = Vec::with_capacity(n_successors);
    for _nodes in 0..n_successors {
        let node_identifier_buffer = take_n_bytes(pos, n_bytes_per_node_identifier);
        successors.push(
            bytes_to_int::<u64>(
                &node_identifier_buffer,
                endianness,
                n_bytes_per_node_identifier,
            )
            .unwrap() as usize,
        );
    }
    Ok(successors)
}

/// Parse the edges
pub fn read_edges(
    pos: &mut Iter<'_, u8>,
    endianness: Endianness,
) -> std::io::Result<Vec<Vec<usize>>> {
    let length_buffer = take_n_bytes(pos, 8);
    let n_nodes: usize = bytes_to_u64(&length_buffer, endianness).try_into().unwrap();
    let mut edges: Vec<Vec<usize>> = Vec::new();
    let n_bytes_per_node_identifier: usize = n_bytes_node_encoding(n_nodes).expect(
        "Error: could not identify the number of bytes required to encode the node identifiers.",
    );
    for _node in 0..n_nodes {
        let n_successors_buffer = take_n_bytes(pos, 8);
        let n_successors: usize = bytes_to_u64(&n_successors_buffer, endianness)
            .try_into()
            .unwrap();
        edges.push(
            read_n_successors(pos, endianness, n_bytes_per_node_identifier, n_successors)
                .expect("Error: could not read edges"),
        );
    }
    Ok(edges)
}
