#[macro_use]
extern crate criterion;

use ark_bls12_381::g1::Parameters;
use ark_std::rand::RngCore;
use ark_std::test_rng;
use criterion::Criterion;
use indifferentiable_hashing::IndifferentiableHash;

criterion_main!(bench);
criterion_group!(bench, bench_hash_to_group);

fn bench_hash_to_group(c: &mut Criterion) {
    let mut rng = test_rng();
    let num_tests = 1000;
    let inputs: Vec<Vec<u8>> = (0..num_tests)
        .map(|_| (0..32).map(|_| rng.next_u32() as u8).collect::<Vec<u8>>())
        .collect();

    let mut bench_group = c.benchmark_group("hash to group");
    bench_group.sample_size(100);
    let bench_str = format!("indifferentiable hash for bls12-381");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res = <Parameters as IndifferentiableHash>::hash_to_curve(&inputs[i]);
            }
        });
    });
}
