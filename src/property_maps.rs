//! # Handle node and edge property maps

/* std use */
use std::collections::HashMap;
use std::slice::Iter;

/* crate use */

use crate::GraphToolGraph;
/* project use */

use crate::reader::Endianness;
use crate::reader::{
    read_bool, read_f64, read_i16, read_i32, read_i64, read_size, read_string, read_vector,
};

/// First-class enum values representing supported types to read from gt format
///
/// python object, long double and long double vectors are not supported
#[derive(Debug, Clone)]
pub enum PropertyType {
    Bool,
    I16,
    I32,
    I64,
    F64,
    String,
    BoolVector,
    I16Vector,
    I32Vector,
    I64Vector,
    F64Vector,
    StringVector,
}

/// Data structure holding all property maps
/// Each data types has its own hashmap, where a string key allows to select the wanted hashmap,
/// with node maps
pub struct PropertyMaps<T> {
    pub bool_maps: HashMap<String, HashMap<T, bool>>,
    pub i16_maps: HashMap<String, HashMap<T, i16>>,
    pub i32_maps: HashMap<String, HashMap<T, i32>>,
    pub i64_maps: HashMap<String, HashMap<T, i64>>,
    pub f64_maps: HashMap<String, HashMap<T, f64>>,
    pub string_maps: HashMap<String, HashMap<T, String>>,
    pub bool_vector_maps: HashMap<String, HashMap<T, Vec<bool>>>,
    pub i16_vector_maps: HashMap<String, HashMap<T, Vec<i16>>>,
    pub i32_vector_maps: HashMap<String, HashMap<T, Vec<i32>>>,
    pub i64_vector_maps: HashMap<String, HashMap<T, Vec<i64>>>,
    pub f64_vector_maps: HashMap<String, HashMap<T, Vec<f64>>>,
    pub string_vector_maps: HashMap<String, HashMap<T, Vec<String>>>,
    pub key_to_type: HashMap<String, PropertyType>,
}

impl<T> Default for PropertyMaps<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PropertyMaps<T> {
    pub fn new() -> Self {
        PropertyMaps {
            bool_maps: HashMap::new(),
            i16_maps: HashMap::new(),
            i32_maps: HashMap::new(),
            i64_maps: HashMap::new(),
            f64_maps: HashMap::new(),
            string_maps: HashMap::new(),
            bool_vector_maps: HashMap::new(),
            i16_vector_maps: HashMap::new(),
            i32_vector_maps: HashMap::new(),
            i64_vector_maps: HashMap::new(),
            f64_vector_maps: HashMap::new(),
            string_vector_maps: HashMap::new(),
            key_to_type: HashMap::new(),
        }
    }
}

pub fn property_type_index_to_type(index: u8) -> PropertyType {
    match index {
        0x00 => PropertyType::Bool,
        0x01 => PropertyType::I16,
        0x02 => PropertyType::I32,
        0x03 => PropertyType::I64,
        0x04 => PropertyType::F64,
        0x05 => panic!("long double property maps parsing not implemented"),
        0x06 => PropertyType::String,
        0x07 => PropertyType::BoolVector,
        0x08 => PropertyType::I16Vector,
        0x09 => PropertyType::I32Vector,
        0x0a => PropertyType::I64Vector,
        0x0b => PropertyType::F64Vector,
        0x0c => panic!("long double vector property maps parsing not implemented"),
        0x0d => PropertyType::StringVector,
        0x0e => panic!("python object property maps parsing not implemented"),
        _ => panic!("property map index type not understood: {index:#}"),
    }
}

#[derive(Debug, Clone)]
pub enum PropertyMapType {
    Graph,
    Vertex,
    Edge,
}

pub fn property_map_type_index_to_type(index: u8) -> PropertyMapType {
    match index {
        0x00 => PropertyMapType::Graph,
        0x01 => PropertyMapType::Vertex,
        0x02 => PropertyMapType::Edge,
        _ => panic!("property map type index type not understood: {index:#}"),
    }
}

pub trait PropertyMapsReader {
    fn read_property_maps(&mut self, pos: &mut Iter<'_, u8>, endianness: Endianness);
    fn read_graph_property_map(&mut self, pos: &mut Iter<'_, u8>, endianness: Endianness);
    fn read_vertex_property_map(&mut self, pos: &mut Iter<'_, u8>, endianness: Endianness);
    fn read_edge_property_map(&mut self, pos: &mut Iter<'_, u8>, endianness: Endianness);
}

impl PropertyMapsReader for GraphToolGraph {
    fn read_property_maps(&mut self, pos: &mut Iter<'_, u8>, endianness: Endianness) {
        let n_property_maps = read_size(pos, endianness).unwrap();
        for _property_map in 0..n_property_maps {
            if let Some(&property_map_flag) = pos.next() {
                match property_map_type_index_to_type(property_map_flag) {
                    PropertyMapType::Graph => {
                        self.read_graph_property_map(pos, endianness);
                    }
                    PropertyMapType::Vertex => {
                        self.read_vertex_property_map(pos, endianness);
                    }
                    PropertyMapType::Edge => {
                        self.read_edge_property_map(pos, endianness);
                    }
                }
            }
        }
    }

    fn read_graph_property_map(&mut self, pos: &mut Iter<'_, u8>, endianness: Endianness) {
        let map_key = read_string(pos, endianness).unwrap();
        if let Some(&map_type_flag) = pos.next() {
            let map_type = property_type_index_to_type(map_type_flag);
            self.graph_properties
                .key_to_type
                .insert(map_key.clone(), map_type.clone());
            match map_type {
                PropertyType::Bool => {
                    let val = read_bool(pos, endianness).unwrap();
                    let mut property_map = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties
                        .bool_maps
                        .insert(map_key, property_map);
                }
                PropertyType::BoolVector => {
                    let val = read_vector(pos, read_bool, endianness).unwrap();
                    let mut property_map: HashMap<usize, Vec<bool>> = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties
                        .bool_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I16 => {
                    let val = read_i16(pos, endianness).unwrap();
                    let mut property_map = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties.i16_maps.insert(map_key, property_map);
                }
                PropertyType::I32 => {
                    let val = read_i32(pos, endianness).unwrap();
                    let mut property_map = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties.i32_maps.insert(map_key, property_map);
                }
                PropertyType::I64 => {
                    let val = read_i64(pos, endianness).unwrap();
                    let mut property_map = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties.i64_maps.insert(map_key, property_map);
                }
                PropertyType::F64 => {
                    let val = read_f64(pos, endianness).unwrap();
                    let mut property_map = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties.f64_maps.insert(map_key, property_map);
                }
                PropertyType::String => {
                    let val = read_string(pos, endianness).unwrap();
                    let mut property_map = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties
                        .string_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I16Vector => {
                    let val = read_vector(pos, read_i16, endianness).unwrap();
                    let mut property_map: HashMap<usize, Vec<i16>> = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties
                        .i16_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I32Vector => {
                    let val = read_vector(pos, read_i32, endianness).unwrap();
                    let mut property_map: HashMap<usize, Vec<i32>> = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties
                        .i32_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I64Vector => {
                    let val = read_vector(pos, read_i64, endianness).unwrap();
                    let mut property_map: HashMap<usize, Vec<i64>> = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties
                        .i64_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::F64Vector => {
                    let val = read_vector(pos, read_f64, endianness).unwrap();
                    let mut property_map: HashMap<usize, Vec<f64>> = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties
                        .f64_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::StringVector => {
                    let val = read_vector(pos, read_string, endianness).unwrap();
                    let mut property_map: HashMap<usize, Vec<String>> = HashMap::new();
                    property_map.insert(0, val);
                    self.graph_properties
                        .string_vector_maps
                        .insert(map_key, property_map);
                }
            }
        }
    }

    fn read_vertex_property_map(&mut self, pos: &mut Iter<'_, u8>, endianness: Endianness) {
        let map_key = read_string(pos, endianness).unwrap();
        if let Some(&map_type_flag) = pos.next() {
            let map_type = property_type_index_to_type(map_type_flag);
            self.vertex_properties
                .key_to_type
                .insert(map_key.clone(), map_type.clone());
            match map_type {
                PropertyType::Bool => {
                    let mut property_map = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_bool(pos, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .bool_maps
                        .insert(map_key, property_map);
                }
                PropertyType::BoolVector => {
                    let mut property_map: HashMap<usize, Vec<bool>> = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_vector(pos, read_bool, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .bool_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I16 => {
                    let mut property_map = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_i16(pos, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .i16_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I32 => {
                    let mut property_map = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_i32(pos, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .i32_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I64 => {
                    let mut property_map = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_i64(pos, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .i64_maps
                        .insert(map_key, property_map);
                }
                PropertyType::F64 => {
                    let mut property_map = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_f64(pos, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .f64_maps
                        .insert(map_key, property_map);
                }
                PropertyType::String => {
                    let mut property_map = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_string(pos, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .string_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I16Vector => {
                    let mut property_map: HashMap<usize, Vec<i16>> = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_vector(pos, read_i16, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .i16_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I32Vector => {
                    let mut property_map: HashMap<usize, Vec<i32>> = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_vector(pos, read_i32, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .i32_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I64Vector => {
                    let mut property_map: HashMap<usize, Vec<i64>> = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_vector(pos, read_i64, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .i64_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::F64Vector => {
                    let mut property_map: HashMap<usize, Vec<f64>> = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_vector(pos, read_f64, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .f64_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::StringVector => {
                    let mut property_map: HashMap<usize, Vec<String>> = HashMap::new();
                    for vertex in 0..self.edges.len() {
                        let val = read_vector(pos, read_string, endianness).unwrap();
                        property_map.insert(vertex, val);
                    }
                    self.vertex_properties
                        .string_vector_maps
                        .insert(map_key, property_map);
                }
            }
        }
    }
    fn read_edge_property_map(&mut self, pos: &mut Iter<'_, u8>, endianness: Endianness) {
        let map_key = read_string(pos, endianness).unwrap();
        if let Some(&map_type_flag) = pos.next() {
            let map_type = property_type_index_to_type(map_type_flag);
            self.edge_properties
                .key_to_type
                .insert(map_key.clone(), map_type.clone());
            match map_type {
                PropertyType::Bool => {
                    let mut property_map = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_bool(pos, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties.bool_maps.insert(map_key, property_map);
                }
                PropertyType::BoolVector => {
                    let mut property_map: HashMap<(usize, usize), Vec<bool>> = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_vector(pos, read_bool, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties
                        .bool_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I16 => {
                    let mut property_map = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_i16(pos, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties.i16_maps.insert(map_key, property_map);
                }
                PropertyType::I32 => {
                    let mut property_map = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_i32(pos, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties.i32_maps.insert(map_key, property_map);
                }
                PropertyType::I64 => {
                    let mut property_map = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_i64(pos, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties.i64_maps.insert(map_key, property_map);
                }
                PropertyType::F64 => {
                    let mut property_map = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_f64(pos, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties.f64_maps.insert(map_key, property_map);
                }
                PropertyType::String => {
                    let mut property_map = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_string(pos, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties
                        .string_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I16Vector => {
                    let mut property_map: HashMap<(usize, usize), Vec<i16>> = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_vector(pos, read_i16, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties
                        .i16_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I32Vector => {
                    let mut property_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_vector(pos, read_i32, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties
                        .i32_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::I64Vector => {
                    let mut property_map: HashMap<(usize, usize), Vec<i64>> = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_vector(pos, read_i64, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties
                        .i64_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::F64Vector => {
                    let mut property_map: HashMap<(usize, usize), Vec<f64>> = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_vector(pos, read_f64, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties
                        .f64_vector_maps
                        .insert(map_key, property_map);
                }
                PropertyType::StringVector => {
                    let mut property_map: HashMap<(usize, usize), Vec<String>> = HashMap::new();
                    for u in 0..self.edges.len() {
                        for &v in &self.edges[u] {
                            let val = read_vector(pos, read_string, endianness).unwrap();
                            property_map.insert((u, v), val);
                        }
                    }
                    self.edge_properties
                        .string_vector_maps
                        .insert(map_key, property_map);
                }
            }
        }
    }
}
