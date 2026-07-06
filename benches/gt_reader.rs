//! Measure gt reading function execution time

/* std use */
/* crate use */

/* project use */

pub fn gt_reader_bench(c: &mut criterion::Criterion) {
    let gt_file = "../tests/test_data/pgp_simple_graph_without_properties.gt";

    c.bench_function("gt_reader", |b| {
        criterion::black_box({ gt_reader::read_gt(gt_file) })
    });
}

criterion::criterion_group! {
name = benches;
config = criterion::Criterion::default()               // Check https://docs.rs/criterion/latest/criterion/struct.Criterion.html
    .warm_up_time(core::time::Duration::from_secs(1))  // for full parameter
.measurement_time(core::time::Duration::from_secs(10))
.sample_size(1);
targets = gt_reader_bench
}
criterion::criterion_main!(benches);
