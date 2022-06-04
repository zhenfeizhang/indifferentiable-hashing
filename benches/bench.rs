#[macro_use]
extern crate criterion;

use ark_bls12_377::g1::Parameters as Param377;
use ark_bls12_381::g1::Parameters as Param381;
use ark_ff::fields::Field;
use ark_ff::fields::PrimeField;
use ark_std::rand::RngCore;
use ark_std::test_rng;
use ark_std::UniformRand;
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
    let inputs_clone = inputs.clone();
    bench_group.sample_size(100);
    let bench_str = format!("indifferentiable hash for bls12-381");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res = <Param381 as IndifferentiableHash>::hash_to_curve(&inputs_clone[i]);
            }
        });
    });

    bench_group.sample_size(100);
    let bench_str = format!("indifferentiable hash for bls12-377");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res = <Param377 as IndifferentiableHash>::hash_to_curve(&inputs[i]);
            }
        });
    });

    bench_group.sample_size(100);
    let t1: Vec<ark_bls12_377::Fr> = (0..num_tests)
        .map(|_| ark_bls12_377::Fr::rand(&mut rng))
        .collect();
    let t2: Vec<ark_bls12_377::Fr> = (0..num_tests)
        .map(|_| ark_bls12_377::Fr::rand(&mut rng))
        .collect();
    let bench_str = format!("field exp");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _ = t1[i].pow(t2[i].into_repr());
            }
        });
    });
}
