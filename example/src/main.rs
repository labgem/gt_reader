
use gt_reader;

fn main() {
    let graph = gt_reader::read_gt("../tests/test_data/pgp_graph_with_properties.gt").unwrap();

    println!("type of 'valid' vertex property: {:?}", graph.vertex_properties.key_to_type.get(&String::from("valid")));
}
