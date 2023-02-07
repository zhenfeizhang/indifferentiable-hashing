#[macro_use]
extern crate criterion;

use ark_bls12_377::g1::Config as Param377;
use ark_bls12_381::g1::Config as Param381;
use ark_ec::hashing::curve_maps::wb::WBMap;
use ark_ec::hashing::map_to_curve_hasher::MapToCurveBasedHasher;
use ark_ec::hashing::HashToCurve;
use ark_ff::field_hashers::DefaultFieldHasher;
use ark_ff::fields::Field;
use ark_ff::fields::PrimeField;
use ark_std::rand::RngCore;
use ark_std::test_rng;
use ark_std::UniformRand;
use criterion::Criterion;
use indifferentiable_hashing::IndifferentiableHash;
use sha2::Sha512;

criterion_main!(bench);
criterion_group!(bench, bench_hash_to_group, bench_wb_hash);

fn bench_hash_to_group(c: &mut Criterion) {
    let mut rng = test_rng();
    let num_tests = 1000;

    let inputs: Vec<Vec<u8>> = (0..num_tests)
        .map(|_| (0..32).map(|_| rng.next_u32() as u8).collect::<Vec<u8>>())
        .collect();

    let mut bench_group = c.benchmark_group("indifferentiable hash");
    bench_group.sample_size(100);

    let inputs_clone = inputs.clone();
    let bench_str = format!("hash to group bls12-381");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res = <Param381 as IndifferentiableHash>::hash_to_curve(&inputs_clone[i]);
            }
        });
    });

    let inputs_clone = inputs.clone();
    let bench_str = format!("hash to group bls12-377");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res = <Param377 as IndifferentiableHash>::hash_to_curve(&inputs_clone[i]);
            }
        });
    });

    let inputs_clone = inputs.clone();
    let bench_str = format!("hash to curve bls12-381");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res =
                    <Param381 as IndifferentiableHash>::hash_to_curve_unchecked(&inputs_clone[i]);
            }
        });
    });

    let bench_str = format!("hash to curve bls12-377");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res = <Param377 as IndifferentiableHash>::hash_to_curve_unchecked(&inputs[i]);
            }
        });
    });

    let t1: Vec<ark_bls12_377::Fq> = (0..num_tests)
        .map(|_| ark_bls12_377::Fq::rand(&mut rng))
        .collect();
    let t2: Vec<ark_bls12_377::Fq> = (0..num_tests)
        .map(|_| ark_bls12_377::Fq::rand(&mut rng))
        .collect();
    let bench_str = format!("field exp");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _ = t1[i].pow(t2[i].into_bigint());
            }
        });
    });
}

fn bench_wb_hash(c: &mut Criterion) {
    let mut rng = test_rng();
    let num_tests = 1000;

    let inputs: Vec<Vec<u8>> = (0..num_tests)
        .map(|_| (0..32).map(|_| rng.next_u32() as u8).collect::<Vec<u8>>())
        .collect();

    let mut bench_group = c.benchmark_group("SWU hash");
    bench_group.sample_size(100);

    let hasher = MapToCurveBasedHasher::<
        ark_bls12_377::G1Projective,
        DefaultFieldHasher<Sha512, 128>,
        WBMap<ark_bls12_377::g1::Config>,
    >::new(b"")
    .unwrap();

    let inputs_clone = inputs.clone();
    let bench_str = format!("hash to group bls12-377");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res = hasher.hash(&inputs_clone[i]);
            }
        });
    });

    let hasher = MapToCurveBasedHasher::<
        ark_bls12_381::G1Projective,
        DefaultFieldHasher<Sha512, 128>,
        WBMap<ark_bls12_381::g1::Config>,
    >::new(b"")
    .unwrap();

    let bench_str = format!("hash to group bls12-381");
    bench_group.bench_function(bench_str, move |b| {
        b.iter(|| {
            for i in 0..num_tests {
                let _res = hasher.hash(&inputs[i]);
            }
        });
    });
}
